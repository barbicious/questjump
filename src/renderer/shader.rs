use crate::renderer::bindable::Bindable;
use glow::{HasContext, NativeProgram};
use std::path::Path;
use std::rc::Rc;
use nalgebra_glm::Mat4;

pub struct Shader {
    id: NativeProgram,
    gl: Rc<glow::Context>,
}

impl Shader {
    pub fn new(gl: Rc<glow::Context>) -> anyhow::Result<Self> {
        Ok(Self {
            id: unsafe { gl.create_program().map_err(anyhow::Error::msg)? },
            gl,
        })
    }

    pub fn compile_path<P: AsRef<Path>>(&self, path: P, r#type: u32) -> anyhow::Result<()> {
        let src = std::fs::read_to_string(path.as_ref())?;

        self.compile_str(&src, r#type)
    }

    pub fn compile_str(&self, src: &str, r#type: u32) -> anyhow::Result<()> {
        let shader = unsafe { self.gl.create_shader(r#type).map_err(anyhow::Error::msg)? };

        unsafe {
            self.gl.shader_source(shader, src);
            self.gl.compile_shader(shader);

            if !self.gl.get_shader_compile_status(shader) {
                anyhow::bail!("{}", self.gl.get_shader_info_log(shader));
            }

            self.gl.attach_shader(self.id, shader);
            self.gl.delete_shader(shader);
        }

        Ok(())
    }

    pub fn complete(&self) -> anyhow::Result<()> {
        unsafe {
            self.gl.link_program(self.id);

            if !self.gl.get_program_link_status(self.id) {
                anyhow::bail!("{}", self.gl.get_program_info_log(self.id));
            }
        }

        Ok(())
    }

    pub fn set_mat4(&self, name: &str, mat4: &Mat4) {
        unsafe {
            self.gl.uniform_matrix_4_f32_slice(self.gl.get_uniform_location(self.id, name).as_ref(), false, mat4.data.as_slice());
        }
    }
}

impl Bindable for Shader {
    fn bind(&self) {
        unsafe {
            self.gl.use_program(Some(self.id));
        }
    }

    fn unbind(&self) {
        unsafe {
            self.gl.use_program(None);
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_program(self.id);
        }
    }
}

#[macro_export]
macro_rules! shader {
    ($gl:expr, $($shader_prop:expr),+) => {{
        use crate::renderer::shader::Shader;

        let shader = Shader::new($gl)?;

        $(
            shader.compile_str($shader_prop.src, $shader_prop.r#type)?;
        )+

        shader.complete()?;

        shader
    }};
}

pub struct ShaderProps {
    pub src: &'static str,
    pub r#type: u32,
}
