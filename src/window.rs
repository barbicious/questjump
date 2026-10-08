use crate::renderer::Renderer;
use glfw::{Context, log_errors};
use std::rc::Rc;

pub struct Window {
    glfw: glfw::Glfw,
    window: glfw::PWindow,
    event: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> anyhow::Result<(Self, Renderer)> {
        let mut glfw = glfw::init(log_errors!())?;

        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        if cfg!(target_os = "macos") {
            glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        }

        let (mut window, event) = glfw
            .create_window(
                width,
                height,
                format!("{}: v{}", title, env!("CARGO_PKG_VERSION")).as_str(),
                glfw::WindowMode::Windowed,
            )
            .ok_or(anyhow::anyhow!("Couldn't create GLFW window!"))?;

        window.make_current();

        window.set_all_polling(true);

        let renderer = Renderer::new(Rc::new(unsafe {
            glow::Context::from_loader_function(|s| {
                if let Some(proc) = glfw.get_proc_address_raw(s) {
                    proc as *const std::ffi::c_void
                } else {
                    std::ptr::null()
                }
            })
        }));

        Ok((
            Self {
                glfw,
                window,
                event,
            },
            renderer,
        ))
    }

    pub fn should_close(&mut self) -> bool {
        self.glfw.poll_events();
        self.window.should_close()
    }

    pub fn splat(&mut self) {
        self.window.swap_buffers();
    }
}
