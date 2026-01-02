use std::sync::Arc;

use wgpu::*;
use winit::window::Window;

#[derive(Debug)]
pub struct Renderer {
    surface: Surface<'static>,
    config: SurfaceConfiguration,
    device: Device,
    queue: Queue,
    pipeline: RenderPipeline,
}

impl Renderer {
    pub async fn new(window: Arc<Window>) -> Self {
        let instance = Instance::new(&InstanceDescriptor::default());
        let surface = instance
            .create_surface(window.clone())
            .expect("Cannot create surface");

        let required_features = wgpu::Features::EXPERIMENTAL_MESH_SHADER;
        let required_limits = Limits::defaults().using_recommended_minimum_mesh_shader_values();

        let adapter = instance
            .enumerate_adapters(Backends::VULKAN)
            .await
            .into_iter()
            .find(|adapter| {
                adapter.features().contains(required_features)
                    && required_limits.check_limits(&adapter.limits())
            })
            .expect("No adapter found with Mesh Shader support!");

        println!("GPU: {}", adapter.get_info().name);
        println!("Render Backend: {:?}", adapter.get_info().backend);

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                experimental_features: unsafe { wgpu::ExperimentalFeatures::enabled() },
                required_features,
                required_limits,
                ..Default::default()
            })
            .await
            .unwrap();

        let mut config = surface
            .get_default_config(
                &adapter,
                window.inner_size().width,
                window.inner_size().height,
            )
            .expect("Adapter does not support creation of surface");

        println!("Surface format: {:?}", config.format);
        config.present_mode = PresentMode::AutoVsync;

        surface.configure(&device, &config);

        let module = &device.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let pipeline = device.create_mesh_pipeline(&MeshPipelineDescriptor {
            label: None,
            cache: None,
            layout: Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor {
                bind_group_layouts: &[],
                ..Default::default()
            })),
            task: Some(TaskState {
                module,
                entry_point: None,
                compilation_options: Default::default(),
            }),
            mesh: MeshState {
                module,
                entry_point: None,
                compilation_options: Default::default(),
            },
            fragment: Some(FragmentState {
                module,
                entry_point: None,
                targets: &[Some(ColorTargetState {
                    format: config.format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                polygon_mode: PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            multisample: MultisampleState::default(),
            depth_stencil: None,
            multiview: None,
        });

        Renderer {
            surface,
            config,
            device,
            queue,
            pipeline,
        }
    }

    pub fn render(&mut self) {
        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("Cannot get next texture");
        let surface_texture_view = surface_texture
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&Default::default());

        let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &surface_texture_view,
                depth_slice: None,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(wgpu::Color {
                        r: 0.01,
                        g: 0.01,
                        b: 0.01,
                        a: 1.0,
                    }),
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            ..Default::default()
        });
        pass.set_pipeline(&self.pipeline);
        pass.draw_mesh_tasks(1, 1, 1);
        drop(pass);

        self.queue.submit(Some(encoder.finish()));
        surface_texture.present();
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        if size.width == 0 || size.height == 0 {
            return;
        }
        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.device, &self.config);
    }
}
