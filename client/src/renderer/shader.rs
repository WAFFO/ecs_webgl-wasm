use std::collections::HashMap;

use web_sys::{WebGlProgram, WebGl2RenderingContext, WebGlShader, WebGlUniformLocation};
use crate::math;

pub struct Shader {
    program: WebGlProgram,
    uniform: HashMap<&'static str, WebGlUniformLocation>,
}

#[allow(dead_code)]
impl Shader {

    pub fn new(context: &WebGl2RenderingContext, vertex_str: &'static str, fragment_str: &'static str
    ) -> Result<Shader, String> {

        let functions = include_str!("glsl/common_functions.glsl");

        let vertex_shader = Shader::compile_shader(
            &context,
            WebGl2RenderingContext::VERTEX_SHADER,
            &[functions,vertex_str].concat(),
        )?;
        let fragment_shader = Shader::compile_shader(
            &context,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            fragment_str,
        )?;
        let program = Shader::link_program(&context, [vertex_shader, fragment_shader].iter())?;

        let uniform = HashMap::new();

        Ok(Shader{
            program,
            uniform,
        })
    }

    pub fn use_shader(&self, context: &WebGl2RenderingContext) {
        context.use_program(Some(&self.program));
    }

    fn compile_shader(context: &WebGl2RenderingContext, shader_type: u32, source: &str
    ) -> Result<WebGlShader, String> {
        let shader = context
            .create_shader(shader_type)
            .ok_or_else(|| String::from("Unable to create shader object"))?;
        context.shader_source(&shader, source);
        context.compile_shader(&shader);

        if context
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            Err(context
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| "Unknown error creating shader".into()))
        }
    }

    fn link_program<'a, T>(context: &WebGl2RenderingContext, shaders: T
    ) -> Result<WebGlProgram, String>
        where T: IntoIterator<Item=&'a WebGlShader> {
        let program = context
            .create_program()
            .ok_or_else(|| String::from("Unable to create shader object"))?;
        for shader in shaders {
            context.attach_shader(&program, shader)
        }
        context.link_program(&program);

        if context
            .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(program)
        } else {
            Err(context
                .get_program_info_log(&program)
                .unwrap_or_else(|| "Unknown error creating program object".into()))
        }
    }

    // utility uniform functions
    pub fn set_bool(&mut self, context: &WebGl2RenderingContext, name: &'static str, value: bool) {
        context.uniform1i(Some(&self.get_uniform_location(&context, name)), value as i32);
    }

    pub fn set_int(&mut self, context: &WebGl2RenderingContext, name: &'static str, value: i32) {
        context.uniform1i(Some(&self.get_uniform_location(&context, name)), value);
    }

    pub fn set_float(&mut self, context: &WebGl2RenderingContext, name: &'static str, value: f32) {
        context.uniform1f(Some(&self.get_uniform_location(&context, name)), value);
    }

//    pub fn set_vec2(&mut self, context: &WebGl2RenderingContext, name: &'static str, value: &mut glm::Vec2) {
//        context.uniform2fv_with_f32_array(
//            Some(&self.get_uniform_location(&context, name)),
//            value.data_ref_mut()),
//        );
//    }
//    pub fn set_vec2_xy(&mut self, context: &WebGl2RenderingContext, name: &'static str, x: f32, y: f32) {
//        context.uniform2f(Some(&self.get_uniform_location(&context, name)), x, y);
//    }

    pub fn set_vec3(&mut self, context: &WebGl2RenderingContext, name: &'static str, value: &mut math::Vert3) {
        context.uniform3fv_with_f32_array(
            Some(&self.get_uniform_location(&context, name)),
            value.data_ref_mut(),
        );
    }
    pub fn set_vec3_xyz(&mut self, context: &WebGl2RenderingContext, name: &'static str, x: f32, y: f32, z: f32) {
        context.uniform3f(Some(&self.get_uniform_location(&context, name)), x, y, z);
    }

    pub fn set_vec4(&mut self, context: &WebGl2RenderingContext, name: &'static str, value: &mut math::Vert4) {
        context.uniform4fv_with_f32_array(
            Some(&self.get_uniform_location(&context, name)),
            value.data_ref_mut(),
        );
    }
    pub fn set_vec4_xyzw(&mut self, context: &WebGl2RenderingContext, name: &'static str, x: f32, y: f32, z: f32, w: f32) {
        context.uniform4f(Some(&self.get_uniform_location(&context, name)), x, y, z, w);
    }

//    pub fn set_mat2(&mut self, context: &WebGl2RenderingContext, name: &'static str, mat: &mut glm::Mat2) {
//        context.uniform_matrix2fv_with_f32_array(
//            Some(&self.get_uniform_location(&context, name)),
//            false,
//            mat.data_ref_mut(),
//        );
//    }

    pub fn set_mat3(&mut self, context: &WebGl2RenderingContext, name: &'static str, mat: &mut math::Mat3) {
        context.uniform_matrix3fv_with_f32_array(
            Some(&self.get_uniform_location(&context, name)),
            false,
            mat.data_ref_mut(),
        );
    }

    pub fn set_mat4(&mut self, context: &WebGl2RenderingContext, name: &'static str, mat: &mut math::Mat4) {
        context.uniform_matrix4fv_with_f32_array(
            Some(&self.get_uniform_location(&context, name)),
            false,
            mat.data_ref_mut(),
        );
    }

    // hash map cache
    pub fn get_uniform_location(&mut self, context: &WebGl2RenderingContext, name: &'static str
    ) -> WebGlUniformLocation {
        let location = if let Some(x) = self.uniform.get(name){
            x.clone()
        } else {
            context.get_uniform_location(&self.program, name)
                .expect("Shader Error: Could not find shader.")
        };
        self.uniform.insert(name,location.clone());
        location
    }
}