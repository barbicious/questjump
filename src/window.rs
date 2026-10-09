use std::cell::Cell;
use crate::renderer::Renderer;
use glfw::{Context, log_errors};
use std::rc::Rc;
use glow::HasContext;

pub struct Window {
    glfw: glfw::Glfw,
    window: glfw::PWindow,
    event: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    width: Rc<Cell<i32>>,
    height: Rc<Cell<i32>>,
}

impl Window {
    pub fn new(title: &str, width: i32, height: i32) -> anyhow::Result<(Self, Renderer)> {
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
                width as u32,
                height as u32,
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
        
        let width = Rc::new(Cell::new(width));
        let height = Rc::new(Cell::new(height));
        
        let callback_width = width.clone();
        let callback_height = height.clone();
        
        window.set_framebuffer_size_callback(move |window, width, height| unsafe {
            callback_width.set(width);
            callback_height.set(height);
        });

        Ok((
            Self {
                glfw,
                window,
                event,
                width,
                height,
            },
            renderer,
        ))
    }

    pub fn width(&self) -> i32 {
        self.width.get()
    }
    
    pub fn height(&self) -> i32 {
        self.height.get()
    }
    
    pub fn should_close(&mut self) -> bool {
        self.glfw.poll_events();
        self.window.should_close()
    }

    pub fn splat(&mut self) {
        self.window.swap_buffers();
    }
}