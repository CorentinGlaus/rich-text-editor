use std::{collections::HashMap, ops::Range};

use slotmap::SlotMap;
use wgpu::{BindGroupLayout, SurfaceConfiguration, util::DeviceExt};

use crate::renderer::{
    draw_manager::LayerId,
    rectangle::{
        instance::RectangleInstance,
        vertex::{RECTANGLE_INDICES, RECTANGLE_VERTICES, RectangleVertex},
    },
};

slotmap::new_key_type! {
    pub struct RectangleHandle;
}

pub struct RectangleBatch {
    render_pipeline: wgpu::RenderPipeline,
    shader: wgpu::ShaderModule,

    slots: SlotMap<RectangleHandle, (LayerId, usize)>,
    reverse: HashMap<LayerId, Vec<RectangleHandle>>,

    vertex_buffer: wgpu::Buffer,

    index_buffer: wgpu::Buffer,
    num_indices: u32,

    instances: HashMap<LayerId, Vec<RectangleInstance>>,
    instance_buffer: wgpu::Buffer,
    instance_buffer_capacity: usize,
    instance_count: usize,

    layer_ranges: HashMap<LayerId, Range<u32>>,

    dirty: bool,
}

impl RectangleBatch {
    pub fn new(
        device: &wgpu::Device,
        surface_config: &SurfaceConfiguration,
        camera_bind_group_layout: &BindGroupLayout,
    ) -> RectangleBatch {
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let render_pipeline =
            Self::new_render_pipeline(device, &shader, surface_config, camera_bind_group_layout);

        let instances: HashMap<LayerId, Vec<RectangleInstance>> = HashMap::new();
        let reverse: HashMap<LayerId, Vec<RectangleHandle>> = HashMap::new();
        let layer_ranges: HashMap<LayerId, Range<u32>> = HashMap::new();

        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Rectangle Instance Buffer"),
            size: Self::INITAL_INSTANCE_BUFFER_CAPACITY as u64
                * std::mem::size_of::<RectangleInstance>() as u64,
            mapped_at_creation: false,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Rectangle Vertex Buffer"),
            contents: bytemuck::cast_slice(RECTANGLE_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Rectangle Index buffer"),
            contents: bytemuck::cast_slice(RECTANGLE_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let num_indices = RECTANGLE_INDICES.len() as u32;

        RectangleBatch {
            slots: SlotMap::with_key(),
            instances,
            instance_buffer,
            instance_buffer_capacity: Self::INITAL_INSTANCE_BUFFER_CAPACITY,
            reverse,
            dirty: false,
            vertex_buffer,
            index_buffer,
            shader,
            render_pipeline,
            num_indices,
            instance_count: 0,
            layer_ranges,
        }
    }

    const INITAL_INSTANCE_BUFFER_CAPACITY: usize = 50;

    pub fn create(&mut self, instance: RectangleInstance, layer: LayerId) -> RectangleHandle {
        let layer_instances = self.instances.entry(layer).or_default();
        let layer_reverse = self.reverse.entry(layer).or_default();

        let dense_idx = layer_instances.len();
        layer_instances.push(instance);
        let key = self.slots.insert((layer, dense_idx));
        layer_reverse.push(key);

        self.instance_count += 1;
        self.dirty = true;

        key
    }

    pub fn modify(
        &mut self,
        handle: RectangleHandle,
        function: impl FnOnce(&mut RectangleInstance),
    ) {
        if let Some(&instance_key) = self.slots.get(handle) {
            let layer_instances = self.instances.entry(instance_key.0).or_default();

            function(&mut layer_instances[instance_key.1]);
            self.dirty = true;
        }
    }

    pub fn remove(&mut self, handle: RectangleHandle) {
        let Some(instance_key) = self.slots.remove(handle) else {
            return;
        };
        let layer_instances = self.instances.entry(instance_key.0).or_default();
        let layer_reverse = self.reverse.entry(instance_key.0).or_default();

        let last = layer_instances.len() - 1;
        layer_instances.swap_remove(instance_key.1);
        layer_reverse.swap_remove(instance_key.1);
        if instance_key.1 != last {
            let moved_key = layer_reverse[instance_key.1];
            self.slots[moved_key] = instance_key;
        }

        self.instance_count -= 1;
        self.dirty = true;
    }

    pub fn update_buffer(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layers: &Vec<LayerId>,
    ) {
        if !self.dirty {
            return;
        }
        if self.instance_count > self.instance_buffer_capacity {
            self.instance_buffer_capacity *= 2;
            self.instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Rectangle Instance Buffer"),
                size: (self.instance_buffer_capacity * std::mem::size_of::<RectangleInstance>())
                    as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }

        let mut packed = Vec::with_capacity(self.instance_count);
        for layer in layers {
            if let Some(layer_instances) = self.instances.get(&layer) {
                if layer_instances.is_empty() {
                    continue;
                }
                let start = packed.len() as u32;
                packed.extend_from_slice(layer_instances);
                self.layer_ranges
                    .insert(layer.clone(), start..packed.len() as u32);
            }
        }

        queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&packed));
        self.dirty = false;
    }

    pub fn draw_layer(&mut self, render_pass: &mut wgpu::RenderPass, layer: &LayerId) {
        let Some(range) = self.layer_ranges.get(&layer) else {
            return;
        };

        if range.is_empty() {
            return;
        }

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, range.clone());
    }
}

impl RectangleBatch {
    fn new_render_pipeline(
        device: &wgpu::Device,
        shader: &wgpu::ShaderModule,
        config: &wgpu::SurfaceConfiguration,
        camera_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::RenderPipeline {
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Rectangle Render Pipeline Layout"),
                bind_group_layouts: &[Some(camera_bind_group_layout)],
                immediate_size: 0,
            });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Rectangle Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: Some("vs_main"),
                buffers: &[RectangleVertex::desc(), RectangleInstance::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        })
    }
}
