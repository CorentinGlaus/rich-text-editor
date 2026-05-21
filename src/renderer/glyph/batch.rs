use std::{collections::HashMap, ops::Range};

use slotmap::SlotMap;
use wgpu::{SurfaceConfiguration, util::DeviceExt};

use crate::renderer::{
    draw_manager::LayerId,
    glyph::{
        instance::GlyphInstance,
        vertex::{GLYPH_INDICES, GLYPH_VERTICES, GlyphVertex},
    },
    text::text_manager::TextManager,
};

slotmap::new_key_type! {
    pub struct GlyphHandle;
}

pub struct GlyphBatch {
    render_pipeline: wgpu::RenderPipeline,
    shader: wgpu::ShaderModule,

    slots: SlotMap<GlyphHandle, (LayerId, usize)>,
    reverse: HashMap<LayerId, Vec<GlyphHandle>>,

    vertex_buffer: wgpu::Buffer,

    index_buffer: wgpu::Buffer,
    num_indices: u32,

    instances: HashMap<LayerId, Vec<GlyphInstance>>,
    instance_buffer: wgpu::Buffer,
    instance_buffer_capacity: usize,
    instance_count: usize,

    layer_ranges: HashMap<LayerId, Range<u32>>,

    dirty: bool,
}

impl GlyphBatch {
    pub fn new(
        device: &wgpu::Device,
        surface_config: &SurfaceConfiguration,
        bind_groups: &[Option<&wgpu::BindGroupLayout>],
    ) -> Self {
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let render_pipeline =
            Self::new_render_pipeline(device, &shader, surface_config, bind_groups);

        let instances: HashMap<LayerId, Vec<GlyphInstance>> = HashMap::new();
        let reverse: HashMap<LayerId, Vec<GlyphHandle>> = HashMap::new();
        let layer_ranges: HashMap<LayerId, Range<u32>> = HashMap::new();

        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Image Instance Buffer"),
            size: Self::INITAL_INSTANCE_BUFFER_CAPACITY as u64
                * std::mem::size_of::<GlyphInstance>() as u64,
            mapped_at_creation: false,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Image Vertex Buffer"),
            contents: bytemuck::cast_slice(GLYPH_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Image Index buffer"),
            contents: bytemuck::cast_slice(GLYPH_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let num_indices = GLYPH_INDICES.len() as u32;

        GlyphBatch {
            slots: SlotMap::with_key(),
            instances,
            reverse,
            instance_buffer,
            instance_buffer_capacity: Self::INITAL_INSTANCE_BUFFER_CAPACITY,
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

    pub fn create(&mut self, instance: GlyphInstance, layer: LayerId) -> GlyphHandle {
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

    pub fn modify(&mut self, handle: GlyphHandle, function: impl FnOnce(&mut GlyphInstance)) {
        if let Some(&instance_key) = self.slots.get(handle) {
            let layer_instances = self.instances.entry(instance_key.0).or_default();

            function(&mut layer_instances[instance_key.1]);
            self.dirty = true;
        }
    }

    pub fn remove(&mut self, handle: GlyphHandle) {
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
                label: Some("Image Instance Buffer"),
                size: (self.instance_buffer_capacity * std::mem::size_of::<GlyphInstance>()) as u64,
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

    pub fn draw_layer(
        &mut self,
        render_pass: &mut wgpu::RenderPass,
        text_manager: &mut TextManager,
        layer: &LayerId,
    ) {
        let Some(range) = self.layer_ranges.get(&layer) else {
            return;
        };

        if range.is_empty() {
            return;
        }

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(1, text_manager.glyph_atlas.bind_group(), &[]);
        text_manager.bind_buffer(render_pass);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, range.clone());
    }
}

impl GlyphBatch {
    fn new_render_pipeline(
        device: &wgpu::Device,
        shader: &wgpu::ShaderModule,
        config: &wgpu::SurfaceConfiguration,
        bind_groups: &[Option<&wgpu::BindGroupLayout>],
    ) -> wgpu::RenderPipeline {
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Glyph Render Pipeline Layout"),
                bind_group_layouts: bind_groups,
                immediate_size: 0,
            });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Glyph Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: Some("vs_main"),
                buffers: &[GlyphVertex::desc(), GlyphInstance::desc()],
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
