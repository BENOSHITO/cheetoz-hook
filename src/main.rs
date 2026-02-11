use std::time::Instant;

use glium::{glutin, Surface};
use imgui::*;
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};

#[derive(Debug)]
struct AppState {
    menu_open: bool,
    autoclicker_enabled: bool,
    cps: i32,
}

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();

    let wb = glutin::window::WindowBuilder::new()
        .with_title("cheetozhook")
        .with_inner_size(glutin::dpi::LogicalSize::new(900f64, 600f64));
    let cb = glutin::ContextBuilder::new().with_vsync(true);

    let display = glium::Display::new(wb, cb, &event_loop).expect("Nie udalo sie utworzyc okna");

    let mut imgui = Context::create();
    imgui.set_ini_filename(None);

    let mut platform = WinitPlatform::init(&mut imgui);
    {
        let window = display.gl_window().window();
        platform.attach_window(imgui.io_mut(), window, HiDpiMode::Rounded);
    }

    let mut renderer = Renderer::init(&mut imgui, &display).expect("Nie udalo sie utworzyc renderer'a");

    let mut last_frame = Instant::now();
    let mut state = AppState {
        menu_open: true,
        autoclicker_enabled: false,
        cps: 10,
    };

    event_loop.run(move |event, _, control_flow| {
        let window = display.gl_window().window();
        platform.handle_event(imgui.io_mut(), window, &event);

        match event {
            glutin::event::Event::NewEvents(_) => {
                imgui.io_mut().update_delta_time(last_frame.elapsed());
                last_frame = Instant::now();
            }
            glutin::event::Event::MainEventsCleared => {
                platform
                    .prepare_frame(imgui.io_mut(), window)
                    .expect("prepare_frame nieudane");
                window.request_redraw();
            }
            glutin::event::Event::RedrawRequested(_) => {
                let ui = imgui.frame();

                if state.menu_open {
                    Window::new("cheetozhook")
                        .size([360.0, 180.0], Condition::FirstUseEver)
                        .build(&ui, || {
                            ui.text("autoclicker wlacz / wylacz");
                            ui.separator();
                            Slider::new("CPS", 1, 30).build(&ui, &mut state.cps);
                            ui.checkbox("Autoclicker", &mut state.autoclicker_enabled);

                            ui.separator();
                            ui.text(format!(
                                "Status: {} | CPS: {}",
                                if state.autoclicker_enabled {
                                    "WLACZONY"
                                } else {
                                    "WYLACZONY"
                                },
                                state.cps
                            ));
                        });
                }

                let mut target = display.draw();
                target.clear_color_srgb(0.07, 0.07, 0.09, 1.0);

                platform.prepare_render(&ui, window);
                renderer
                    .render(&mut target, ui.render())
                    .expect("Render nieudany");
                target.finish().expect("Swap buffers nieudane");
            }
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                }
                glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(glutin::event::VirtualKeyCode::Insert) = input.virtual_keycode {
                        if input.state == glutin::event::ElementState::Pressed {
                            state.menu_open = !state.menu_open;
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
    });
}
