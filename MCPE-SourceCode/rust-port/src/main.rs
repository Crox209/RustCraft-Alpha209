use rust_port::ninecraft_app::NinecraftApp;
use rust_port::app::App;
use rust_port::renderer::GameRenderer;
use std::time::{Duration, Instant};
use std::thread;
use std::env;
use std::collections::HashSet;

fn run_headless(mut app: NinecraftApp) {
    println!("No display detected — running headless for 5 seconds.");
    let start = Instant::now();
    while start.elapsed().as_secs() < 5 {
        app.update();
        thread::sleep(Duration::from_millis(50));
    }
    println!("Headless run finished.");
}

fn perspective_matrix(fovy_deg: f32, aspect: f32, near: f32, far: f32) -> [f32; 16] {
    let fovy = fovy_deg.to_radians();
    let f = 1.0 / (fovy / 2.0).tan();
    let nf = 1.0 / (near - far);

    [
        f / aspect, 0.0, 0.0, 0.0,
        0.0, f, 0.0, 0.0,
        0.0, 0.0, (far + near) * nf, -1.0,
        0.0, 0.0, (2.0 * far * near) * nf, 0.0,
    ]
}

fn main() {
    // Detect display availability (avoid winit backend init in headless containers)
    let has_display = env::var("DISPLAY").is_ok() || env::var("WAYLAND_DISPLAY").is_ok();

    // Initialize app state early so both headless and GUI paths run the same logic
    let mut app = NinecraftApp::new();
    app.init();

    if !has_display {
        // Headless mode for CI/container environments without X/Wayland
        run_headless(app);
        return;
    }

    // GUI path (display present)
    use winit::{
        event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    };

    let event_loop = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title("RustCraft")
        .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0));

    // Build a GL context with glutin + winit
    let windowed_context = {
        use glutin::ContextBuilder;
        let ctx = ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(wb, &event_loop)
            .expect("Failed to create windowed GL context");
        unsafe { ctx.make_current().expect("Failed to make GL context current") }
    };

    let gl = unsafe { glow::Context::from_loader_function(|s| windowed_context.get_proc_address(s) as *const _) };
    let gl = std::rc::Rc::new(gl);

    // Generate merged mesh vertices from level (y = 64 layer)
    let mesh_vertices: Vec<f32> = if let Some(level) = app.minecraft_mut().level_ref() {
        // generate mesh across a small chunk radius around origin
        rust_port::mesh::generate_chunked_mesh_vertices(level, 2)
    } else {
        Vec::new()
    };

    let mut renderer = GameRenderer::new(Some(gl.clone()));
    // Upload generated mesh to the GPU
    if !mesh_vertices.is_empty() {
        renderer.upload_mesh(&mesh_vertices);
    }

    // Simple camera state
    let mut cam_pos = [0.0f32, 0.0f32, 5.0f32];
    let mut keys: HashSet<VirtualKeyCode> = HashSet::new();
    let mut last_frame = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                app.quit();
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent { event: WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode: Some(key), state, .. }, .. }, .. } => {
                match state {
                    ElementState::Pressed => { keys.insert(key); }
                    ElementState::Released => { keys.remove(&key); }
                }
            }
            Event::MainEventsCleared => {
                let now = Instant::now();
                let dt = (now - last_frame).as_secs_f32();
                last_frame = now;

                // Simple WASD movement
                let speed = 3.0f32;
                if keys.contains(&VirtualKeyCode::W) { cam_pos[2] -= speed * dt; }
                if keys.contains(&VirtualKeyCode::S) { cam_pos[2] += speed * dt; }
                if keys.contains(&VirtualKeyCode::A) { cam_pos[0] -= speed * dt; }
                if keys.contains(&VirtualKeyCode::D) { cam_pos[0] += speed * dt; }

                app.update();
                windowed_context.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                let size = windowed_context.window().inner_size();
                let width = size.width as i32;
                let height = size.height as i32;
                renderer.set_viewport(width, height);
                let proj = perspective_matrix(60.0, size.width as f32 / size.height as f32, 0.1, 100.0);
                // Draw merged mesh
                renderer.render_scene(&proj, cam_pos);
                windowed_context.swap_buffers().ok();
                app.draw();
            }
            _ => {}
        }
    });
}
