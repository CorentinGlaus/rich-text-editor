use slotmap::SlotMap;
use wgpu::{SurfaceConfiguration, util::DeviceExt};

use crate::renderer::image::{
    instance::ImageInstance,
    vertex::{IMAGE_INDICES, IMAGE_VERTICES, ImageVertex},
};

slotmap::new_key_type! {
    pub struct ImageHandle;
}

pub struct ImageBatch {
    slots: SlotMap<ImageHandle, usize>,
    instances: Vec<ImageInstance>,
    reverse: Vec<ImageHandle>,
    instance_buffer: wgpu::Buffer,
    instance_buffer_capacity: usize,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    #[expect(unused)]
    shader: wgpu::ShaderModule,
    num_indices: u32,
    dirty: bool,
    render_pipeline: wgpu::RenderPipeline,
}

impl ImageBatch {
    pub fn new(
        device: &wgpu::Device,
        surface_config: &SurfaceConfiguration,
        bind_groups: &[Option<&wgpu::BindGroupLayout>],
    ) -> ImageBatch {
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let render_pipeline =
            Self::new_render_pipeline(device, &shader, surface_config, bind_groups);

        let instances: Vec<ImageInstance> = vec![];

        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Image Instance Buffer"),
            size: Self::INITAL_INSTANCE_BUFFER_CAPACITY as u64
                * std::mem::size_of::<ImageInstance>() as u64,
            mapped_at_creation: false,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Image Vertex Buffer"),
            contents: bytemuck::cast_slice(IMAGE_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Image Index buffer"),
            contents: bytemuck::cast_slice(IMAGE_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let num_indices = IMAGE_INDICES.len() as u32;

        ImageBatch {
            slots: SlotMap::with_key(),
            instances,
            reverse: vec![],
            instance_buffer,
            instance_buffer_capacity: Self::INITAL_INSTANCE_BUFFER_CAPACITY,
            dirty: false,
            vertex_buffer,
            index_buffer,
            shader,
            render_pipeline,
            num_indices,
        }
    }

    const INITAL_INSTANCE_BUFFER_CAPACITY: usize = 50;

    pub fn create(&mut self, inst: ImageInstance) -> ImageHandle {
        // TODO: Order the instances so that the instances are drawn back to fron
        let dense_idx = self.instances.len();
        self.instances.push(inst);
        let key = self.slots.insert(dense_idx);
        self.reverse.push(key);
        self.dirty = true;
        key
    }

    pub fn modify(&mut self, h: ImageHandle, f: impl FnOnce(&mut ImageInstance)) {
        if let Some(&d) = self.slots.get(h) {
            f(&mut self.instances[d]);
            self.dirty = true;
        }
    }

    pub fn remove(&mut self, handle: ImageHandle) {
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

    fn update_buffer(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        if self.dirty {
            if self.instances.len() > self.instance_buffer_capacity {
                self.instance_buffer_capacity *= 2;
                self.instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("Rectangle Instance Buffer"),
                    size: (self.instance_buffer_capacity * std::mem::size_of::<ImageInstance>())
                        as u64,
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });
            }
            let mut sorted_instances = self.instances.clone();
            sorted_instances.sort_by(|a, b| a.position.z.total_cmp(&b.position.z));
            queue.write_buffer(
                &self.instance_buffer,
                0,
                bytemuck::cast_slice(&sorted_instances),
            );
            self.dirty = false;
        }
    }

    pub fn draw(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_pass: &mut wgpu::RenderPass,
        texture_bind_group: &wgpu::BindGroup,
    ) {
        if self.instances.is_empty() {
            return;
        }

        self.update_buffer(device, queue);
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(1, texture_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..self.instances.len() as u32);
    }
}

impl ImageBatch {
    fn new_render_pipeline(
        device: &wgpu::Device,
        shader: &wgpu::ShaderModule,
        config: &wgpu::SurfaceConfiguration,
        bind_groups: &[Option<&wgpu::BindGroupLayout>],
    ) -> wgpu::RenderPipeline {
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Image Render Pipeline Layout"),
                bind_group_layouts: bind_groups,
                immediate_size: 0,
            });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Image Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: Some("vs_main"),
                buffers: &[ImageVertex::desc(), ImageInstance::desc()],
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
