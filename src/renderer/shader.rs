use js_sys::WebAssembly;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGl2RenderingContext, WebGlShader, WebGlUniformLocation};

pub struct Shader {
    program: WebGlProgram,
}


impl Shader {

    pub fn new(context: &WebGl2RenderingContext, vertex_str: &str, fragment_str: &str
    ) -> Result<Shader, String> {
        let vertex_shader = Shader::compile_shader(
            &context,
            WebGl2RenderingContext::VERTEX_SHADER,
            include_str!(vertex_str),
        )?;
        let frag_shader = Shader::compile_shader(
            &context,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            include_str!(fragment_str),
        )?;
        let program = Shader::link_program(&context, [vert_shader, frag_shader].iter())?;

        Ok(Shader{
            program,
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
}