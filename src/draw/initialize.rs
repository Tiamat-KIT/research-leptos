pub struct InitilizeWgpu<'a> {
    pub instance: wgpu::Instance,
    pub surface: wgpu::Surface<'a>,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
}

impl<'a> InitilizeWgpu<'a> {
    pub async fn new(window: &'a winit::window::Window) -> Self {
        use gloo::utils::document_element;
        use leptos::wasm_bindgen::JsCast;

        let canvas = document_element()
            .query_selector("canvas")
            .expect("Failed Query")
            .expect("Failed to get canvas")
            .dyn_into::<leptos::web_sys::HtmlCanvasElement>()
            .unwrap();

        let instance = wgpu::Instance::new(
            wgpu::InstanceDescriptor { 
                backends: wgpu::Backends::all(),
                ..Default::default()
            }
        );
        
        let surface = instance.create_surface(
            window
        ).unwrap();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }
        ).await.unwrap();

        let surface_config = surface
            .get_default_config(&adapter, window.inner_size().width, window.inner_size().height)
            .unwrap();

        let (device,queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: Some("Device And Queue"),
                memory_hints: wgpu::MemoryHints::Performance
            },
            None
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_caps
                .formats.iter().find(|format| format.is_srgb())
                .copied().unwrap_or(
                    surface_caps.formats[0]
                ),
            width: canvas.width(),
            height: canvas.height(),
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2
        };

        surface.configure(&device, &surface_config);

        Self {
            instance,
            surface,
            adapter,
            device,
            queue,
            config
        }
    }
}
