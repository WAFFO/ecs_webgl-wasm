use js_sys::WebAssembly;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader, WebGlUniformLocation, WebGlBuffer};
use specs::{World, Join};
use glm;
use glm::{Vec3, vec3};

use engine::components;

pub struct Renderer {
    attribute: (u32, u32),
    buffer: (web_sys::WebGlBuffer, web_sys::WebGlBuffer, web_sys::WebGlBuffer),
    context: web_sys::WebGlRenderingContext,
    canvas: web_sys::HtmlCanvasElement,
    uniform: (WebGlUniformLocation, WebGlUniformLocation),
}

impl Renderer {
    pub fn new() -> Result<(Renderer), JsValue> {
        // Gather our canvas from the DOM
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = element.dyn_into::<web_sys::HtmlCanvasElement>()?;

        // Cast our canvas into a WebGl context
        let context = canvas
            .get_context("webgl")?
            .unwrap()
            .dyn_into::<WebGlRenderingContext>()?;

        // Compile our shaders
        let vert_shader = Renderer::compile_shader(
            &context,
            WebGlRenderingContext::VERTEX_SHADER,
            include_str!("shaders/basic_vertex.glsl"),
        )?;
        let frag_shader = Renderer::compile_shader(
            &context,
            WebGlRenderingContext::FRAGMENT_SHADER,
            include_str!("shaders/basic_fragment.glsl"),
        )?;

        // A WebGLProgram is the object that holds the two compiled shaders
        let program = Renderer::link_program(&context, [vert_shader, frag_shader].iter())?;
        context.use_program(Some(&program));

        // positions in the shader
        let mut attribute: (u32, u32) = (0, 0);
        attribute.0 = context.get_attrib_location(&program, "a_position") as u32;
        attribute.1 = context.get_attrib_location(&program, "a_color") as u32;

        // Attributes in shaders come from buffers, first get the buffer
        let buffer = context.create_buffer().ok_or("failed to create a vertex buffer")?;
        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

        // color buffer
        let color_buffer = context.create_buffer().ok_or("failed to create a color buffer")?;
        context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&color_buffer));

        // index buffer
        let index_buffer = context.create_buffer().ok_or("failed to create an index buffer")?;
        context.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));

        // Get uniform variable locations from our shaders
        let u_camera =
            context.get_uniform_location(&program, "u_camera")
            .expect("Could not find u_camera.");
        let u_matrix =
            context.get_uniform_location(&program, "u_matrix")
            .expect("Could not find u_matrix.");

        // Cull triangles (counter-clockwise = front facing)
        context.enable(WebGlRenderingContext::CULL_FACE);

        // Test Depth
        context.enable(WebGlRenderingContext::DEPTH_TEST);
        context.depth_func(WebGlRenderingContext::LEQUAL);

        // Return our WebGL object
        Ok(Renderer {
            attribute,
            buffer: (buffer, color_buffer, index_buffer),
            context,
            canvas,
            uniform: (u_camera, u_matrix),
        })
    }

    pub fn draw(&mut self, world: &World) -> Result<(), JsValue> {

        // Draw over the entire canvas and clear buffer to ur present one
        self.context.clear_color(0.9, 0.9, 0.9, 1.0);
        self.context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT | WebGlRenderingContext::DEPTH_BUFFER_BIT);

        let (vertices, colors, indices) = self.build_vertices(world);
        let vertices = vertices.as_slice();
        let colors = colors.as_slice();
        let indices = indices.as_slice();

        // set resolution to the canvas
        Renderer::resize_canvas_to_display_size(&mut self.canvas);
        self.context.viewport(0, 0, self.canvas.width() as i32, self.canvas.width() as i32);

        // Get the buffer out of WebAssembly memory
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()?
            .buffer();
        // Figure out where the vertices are in the memory_buffer (convert pointer to index)
        let vertices_location = vertices.as_ptr() as u32 / 4;
        let vert_array = js_sys::Float32Array::new(&memory_buffer)
            .subarray(vertices_location, vertices_location + vertices.len() as u32);
        let colors_location = colors.as_ptr() as u32 / 4;
        let color_array = js_sys::Float32Array::new(&memory_buffer)
            .subarray(colors_location, colors_location + colors.len() as u32);
        let indices_location = indices.as_ptr() as u32 / 2;
        let index_array = js_sys::Uint16Array::new(&memory_buffer)
            .subarray(indices_location, indices_location + indices.len() as u32);

        // start of vertex binding
        self.context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.buffer.0));
        // Bind buffer to generic vertex attribute of the current vertex buffer object
        self.context.vertex_attrib_pointer_with_i32(self.attribute.0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
        // Buffer_data will copy the data to the GPU memory
        self.context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
        // Attributes need to be enabled before use
        self.context.enable_vertex_attrib_array(self.attribute.0);


        // start of color binding
        self.context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.buffer.1));
        // Bind buffer to generic vertex attribute of the current vertex buffer object
        self.context.vertex_attrib_pointer_with_i32(self.attribute.1, 4, WebGlRenderingContext::FLOAT, false, 0, 0);
        // Buffer_data will copy the data to the GPU memory
        self.context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &color_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
        // Attributes need to be enabled before use
        self.context.enable_vertex_attrib_array(self.attribute.1);


        // start of index binding
        self.context.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(&self.buffer.2));
        // Buffer_data will copy the data to the GPU memory
        self.context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            &index_array,
            WebGlRenderingContext::STATIC_DRAW,
        );

        // camera
        let mut camera = self.build_camera();
        let mut camera = glm::value_ptr_mut(&mut camera);

        // u_camera
        self.context.uniform_matrix4fv_with_f32_array(Some(&self.uniform.0), false, &mut camera);

        self.draw_elements(world, indices.len() as i32);

        Ok(())
    }
    
    // non pub //
    
    fn compile_shader(context: &WebGlRenderingContext, shader_type: u32, source: &str
    ) -> Result<WebGlShader, String> {
        let shader = context
            .create_shader(shader_type)
            .ok_or_else(|| String::from("Unable to create shader object"))?;
        context.shader_source(&shader, source);
        context.compile_shader(&shader);

        if context
            .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
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

    fn link_program<'a, T>(context: &WebGlRenderingContext, shaders: T
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
            .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
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

    fn resize_canvas_to_display_size(canvas: &mut web_sys::HtmlCanvasElement) {
        let w = canvas.client_width() as u32;
        let h = canvas.client_height() as u32;
        if canvas.width() != w || canvas.height() != h {
            canvas.set_width(w);
            canvas.set_height(h);
        }
    }

    fn build_camera(&self) -> glm::Mat4 {

        let perspective =
            glm::perspective(1.0, 45.0, 0.1, 100.0);
        let view =
            glm::look_at(
                &vec3(4.0,3.0,3.0),
                &vec3(0.0, 0.0, 0.0),
                &vec3(0.0, 1.0, 0.0),
            );

        let matrix = perspective * view * glm::Mat4::identity();

        matrix
    }

    fn build_matrices(&self, world: &World) -> Vec<glm::Mat4> {
        let mut matrices: Vec<glm::Mat4> = Vec::new();

        let _transform_storage = world.read_storage::<components::Transform>();

        for transform in (&_transform_storage).join() {
            let matrix = glm::translate(
                &glm::Mat4::identity(),
                &transform.translation,
            );
            let matrix = glm::rotate_x(
                &matrix,
                transform.rotation[0],
            );
            let matrix = glm::rotate_y(
                &matrix,
                transform.rotation[1],
            );
            let matrix = glm::rotate_z(
                &matrix,
                transform.rotation[2],
            );
            let matrix = glm::scale(
                &matrix,
                &transform.scale,
            );
            matrices.push(matrix);
        }

        matrices
    }

    fn build_vertices(&self, world: &World) -> (Vec<f32>, Vec<f32>, Vec<u16>) {
        let mut vertices: Vec<f32> = Vec::new();
        let mut colors: Vec<f32> = Vec::new();
        let mut indices: Vec<u16> = Vec::new();

        let _mesh_storage = world.read_storage::<components::StaticMesh>();

        for mesh in (&_mesh_storage).join() {
            for vertex in &mesh.vertices {
                vertices.push(*vertex);
            }
            for vertex in &mesh.colors {
                colors.push(*vertex);
            }
            for vertex in &mesh.indices {
                indices.push(*vertex);
            }
            // we actually only need one mesh for now (until we add more variety)
            // TODO: actually manage a vertex memory space !!
            break;
        }

        (vertices, colors, indices)
    }

    fn draw_elements(&self, world: &World, size: i32){
        let mut matrices = self.build_matrices(world);

        for mut matrix in matrices {
            let mut _matrix_ptr = glm::value_ptr_mut(&mut matrix);

            // u_matrix
            self.context.uniform_matrix4fv_with_f32_array(Some(&self.uniform.1), false, &mut _matrix_ptr);

            // Draw our shape (Triangles, count, type, offset) Our vertex shader will run <count> times.
            self.context.draw_elements_with_i32(
                WebGlRenderingContext::TRIANGLES,
                size,
                WebGlRenderingContext::UNSIGNED_SHORT,
                0,
            );
        }
    }
}