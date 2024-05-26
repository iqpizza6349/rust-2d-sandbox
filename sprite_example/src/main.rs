mod sprite;
mod render;

use crate::sprite::{load_sprite};
use crate::render::{render};

use glium::{glutin};

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_srgb(true);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let (width, height) = display.get_framebuffer_dimensions();
    println!("Framebuffer dimensions: {}x{}", width, height);

    // 스프라이트 로드
    let sprite = load_sprite(&display, "assets/_idle2.png", (402.0, 304.0), 700.0); // 비율은 유지하지만, 5배 커지게.

    event_loop.run(move |ev, _, control_flow| {
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => *control_flow = glutin::event_loop::ControlFlow::Exit,
                _ => (),
            },
            glutin::event::Event::MainEventsCleared => {
                // 렌더링 호출
                render(&display, vec![sprite.clone()]);
            }
            _ => (),
        }
    });
}
