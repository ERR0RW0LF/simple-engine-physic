use glutin::event_loop::ControlFlow;
use glutin::event::{WindowEvent, Event};
use glutin::Api;
use glutin::GlRequest;
use glutin::ContextBuilder;
use glutin::window::WindowBuilder;
use glutin::event_loop::EventLoop;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("Learn OpenGL with Rust");
    
    let gl_context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .build_windowed(window, &event_loop)
        .expect("Cannot create windowed context");
    
    let gl_context = unsafe {
        gl_context
            .make_current()
            .expect("Failed to make context current")
    };
    
    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);
    
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
    
        match event {
            Event::LoopDestroyed => (),
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
                WindowEvent::Resized(physical_size) => gl_context.resize(physical_size),
            },
            Event::RedrawRequested(_) => {
                unsafe {
                    gl::ClearColor(0.0, 0.0, 1.0, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }
                gl_context.swap_buffers().unwrap();
            }            
            _ => (),
        }
    });
}
