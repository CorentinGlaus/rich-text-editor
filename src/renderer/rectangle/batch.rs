use slotmap::SlotMap;
use wgpu::{BindGroupLayout, SurfaceConfiguration, util::DeviceExt};

use crate::renderer::rectangle::{
    instance::{RectangleInstance, RectangleInstanceRaw},
    vertex::{RECTANGLE_INDICES, RECTANGLE_VERTICES, RectangleVertex},
};

slotmap::new_key_type! {
    pub struct RectangleHandle;
}

pub struct RectangleBatch {
    slots: SlotMap<RectangleHandle, usize>,
    instances: Vec<RectangleInstance>,
    reverse: Vec<RectangleHandle>,
    instance_buffer: wgpu::Buffer,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    #[expect(unused)]
    shader: wgpu::ShaderModule,
    num_indices: u32,
    dirty: bool,
    render_pipeline: wgpu::RenderPipeline,
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

        let instances: Vec<RectangleInstance> = vec![];

        let instance_data = instances
            .iter()
            .map(RectangleInstance::to_raw)
            .collect::<Vec<_>>();
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Rectangle Instance Buffer"),
            contents: bytemuck::cast_slice(&instance_data),
            usage: wgpu::BufferUsages::VERTEX,
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
            reverse: vec![],
            instance_buffer,
            dirty: false,
            vertex_buffer,
            index_buffer,
            shader,
            render_pipeline,
            num_indices,
        }
    }

    pub fn create(&mut self, inst: RectangleInstance) -> RectangleHandle {
        let dense_idx = self.instances.len();
        self.instances.push(inst);
        let key = self.slots.insert(dense_idx);
        self.reverse.push(key);
        self.dirty = true;
        key
    }

    pub fn modify(&mut self, h: RectangleHandle, f: impl FnOnce(&mut RectangleInstance)) {
        if let Some(&d) = self.slots.get(h) {
            f(&mut self.instances[d]);
            self.dirty = true;
        }
    }

    pub fn remove(&mut self, handle: RectangleHandle) {
        let Some(idx) = self.slots.remove(handle) else {
            return;
        };
        let last = self.instances.len() - 1;
        self.instances.swap_remove(idx);
        self.reverse.swap_remove(idx);
        if idx != last {
            let moved_key = self.reverse[idx];
            self.slots[moved_key] = idx;
        }
        self.dirty = true;
    }

    fn update_buffer(&mut self, device: &wgpu::Device) {
        if self.dirty {
            let instance_data = self
                .instances
                .iter()
                .map(RectangleInstance::to_raw)
                .collect::<Vec<_>>();
            self.instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Rectangle Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX,
            });
            self.dirty = false;
        }
    }

    pub fn draw(&mut self, device: &wgpu::Device, render_pass: &mut wgpu::RenderPass) {
        if self.instances.is_empty() {
            return;
        }

        self.update_buffer(device);
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..self.instances.len() as u32);
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
                buffers: &[RectangleVertex::desc(), RectangleInstanceRaw::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
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
