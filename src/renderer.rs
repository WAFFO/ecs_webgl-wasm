use js_sys::WebAssembly;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGl2RenderingContext, WebGlShader, WebGlUniformLocation};
use specs::{World, Join};
use glm;
use glm::vec3;

use engine::components;
use engine::mesh_manager::{MeshManager, mesh::MeshIndex};
use javascript::get_canvas;

pub struct Renderer {
    attribute: (u32, u32),
    buffer:    (web_sys::WebGlBuffer, web_sys::WebGlBuffer),
    context:    web_sys::WebGl2RenderingContext,
    canvas:     web_sys::HtmlCanvasElement,
    uniform:   (WebGlUniformLocation, WebGlUniformLocation, WebGlUniformLocation),
    vao:        web_sys::WebGlVertexArrayObject,
}

impl Renderer {
    pub fn new() -> Result<(Renderer), JsValue> {
        // Gather our canvas from the DOM
        let canvas: web_sys::HtmlCanvasElement = get_canvas()?;

        // Cast our canvas into a WebGl context
        let context = canvas
            .get_context("webgl2")?
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()?;

        // Compile our shaders
        let vert_shader = Renderer::compile_shader(
            &context,
            WebGl2RenderingContext::VERTEX_SHADER,
            include_str!("shaders/basic_vertex.glsl"),
        )?;
        let frag_shader = Renderer::compile_shader(
            &context,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            include_str!("shaders/basic_fragment.glsl"),
        )?;

        // A WebGLProgram is the object that holds the two compiled shaders
        let program = Renderer::link_program(&context, [vert_shader, frag_shader].iter())?;
        context.use_program(Some(&program));

        // create a vertex array object (stores attribute state)
        let vao = context.create_vertex_array()
            .expect("Could not create a Vertex Array Object.");
        context.bind_vertex_array(Some(&vao));

        // positions in the shader
        let mut attribute: (u32, u32) = (0, 0);
        attribute.0 = context.get_attrib_location(&program, "a_position") as u32;
        attribute.1 = context.get_attrib_location(&program, "a_color") as u32;

        // Attributes in shaders come from buffers, first get the buffer
        let buffer = context.create_buffer().ok_or("failed to create a vertex buffer")?;
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        // color buffer
        let color_buffer = context.create_buffer().ok_or("failed to create a color buffer")?;
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&color_buffer));

        // index buffer
//        let index_buffer = context.create_buffer().ok_or("failed to create an index buffer")?;
//        context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));

        // Get uniform variable locations from our shaders
        let u_projection =
            context.get_uniform_location(&program, "u_projection")
                .expect("Could not find u_camera.");
        let u_view =
            context.get_uniform_location(&program, "u_view")
            .expect("Could not find u_camera.");
        let u_matrix =
            context.get_uniform_location(&program, "u_matrix")
            .expect("Could not find u_matrix.");

        // Cull triangles (counter-clockwise = front facing)
        context.enable(WebGl2RenderingContext::CULL_FACE);

        // Test Depth
        context.enable(WebGl2RenderingContext::DEPTH_TEST);
        context.depth_func(WebGl2RenderingContext::LEQUAL);

        // Return our WebGL object
        Ok(Renderer {
            attribute,
            buffer: (buffer, color_buffer),
            context,
            canvas,
            uniform: (u_projection, u_view, u_matrix),
            vao,
        })
    }

    pub fn draw(&mut self, world: &World, mesh_manager: &mut MeshManager) -> Result<(), JsValue> {

        // Draw over the entire canvas and clear buffer to ur present one
        self.context.clear_color(0.9, 0.9, 0.9, 1.0);
        self.context.clear_depth(1.0);
        self.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);

        // set resolution to the canvas
        Renderer::resize_canvas_to_display_size(&mut self.canvas);
        self.context.viewport(0, 0, self.canvas.width() as i32, self.canvas.width() as i32);

        if mesh_manager.updated() {
            self.buffer_data(mesh_manager)?;
        }

        // camera stuff
        let mut proj = Renderer::build_projection();
        let mut proj = glm::value_ptr_mut(&mut proj);
        let mut view = Renderer::build_view(&world);
        let mut view = glm::value_ptr_mut(&mut view);

        // u_projection
        self.context.uniform_matrix4fv_with_f32_array(Some(&self.uniform.0), false, &mut proj);

        // u_view
        self.context.uniform_matrix4fv_with_f32_array(Some(&self.uniform.1), false, &mut view);

        self.draw_arrays(world, mesh_manager);

        Ok(())
    }
    
    // non pub //
    
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

    fn resize_canvas_to_display_size(canvas: &mut web_sys::HtmlCanvasElement) {
        let w = canvas.client_width() as u32;
        let h = canvas.client_height() as u32;
        if canvas.width() != w || canvas.height() != h {
            canvas.set_width(w);
            canvas.set_height(h);
        }
    }

    fn buffer_data(&self, mesh_manager: &mut MeshManager) -> Result<(), JsValue> {

        let (vertices, colors) = mesh_manager.get_storage();
        let vertices = vertices.as_slice();
        let colors = colors.as_slice();
//        let indices = indices.as_slice();

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
//        let indices_location = indices.as_ptr() as u32 / 2;
//        let index_array = js_sys::Uint16Array::new(&memory_buffer)
//            .subarray(indices_location, indices_location + indices.len() as u32);

        // start of vertex binding
        self.context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.buffer.0));
        // Bind buffer to generic vertex attribute of the current vertex buffer object
        self.context.vertex_attrib_pointer_with_i32(self.attribute.0, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
        // Buffer_data will copy the data to the GPU memory
        self.context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );
        // Attributes need to be enabled before use
        self.context.enable_vertex_attrib_array(self.attribute.0);


        // start of color binding
        self.context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.buffer.1));
        // Bind buffer to generic vertex attribute of the current vertex buffer object
        self.context.vertex_attrib_pointer_with_i32(self.attribute.1, 4, WebGl2RenderingContext::FLOAT, false, 0, 0);
        // Buffer_data will copy the data to the GPU memory
        self.context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &color_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );
        // Attributes need to be enabled before use
        self.context.enable_vertex_attrib_array(self.attribute.1);


        // start of index binding
//        self.context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&self.buffer.2));
//        // Buffer_data will copy the data to the GPU memory
//        self.context.buffer_data_with_array_buffer_view(
//            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
//            &index_array,
//            WebGl2RenderingContext::STATIC_DRAW,
//        );

        Ok(())
    }

    fn build_projection() -> glm::Mat4 {
        glm::perspective(1.0, 45.0, 0.1, 200.0)
    }

    fn build_view(world: &World) -> glm::Mat4 {
        let _camera_storage = world.read_storage::<components::Camera>();

        let mut result = glm::Mat4::identity();

        for camera in (&_camera_storage).join() {

            result = glm::look_at(
                &camera.position,
                &camera.target,
                &vec3(0.0, 1.0, 0.0),
            );
            break;
        }

        result
    }

    fn build_matrices(world: &World, mesh_manager: &MeshManager) -> Vec<(glm::Mat4, MeshIndex)> {
        let mut matrices: Vec<(glm::Mat4, MeshIndex)> = Vec::new();

        let _transform_storage = world.read_storage::<components::Transform>();
        let _mesh_storage = world.read_storage::<components::StaticMesh>();

        for (transform, mesh) in (&_transform_storage, &_mesh_storage).join() {
            if let Some(mesh_index) = mesh_manager.get(mesh.mesh_id.clone()) {
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
                matrices.push((matrix, mesh_index));
            }
        }

        matrices
    }

    fn draw_arrays(&self, world: &World, mesh_manager: &MeshManager){
        let matrices = Renderer::build_matrices(world, mesh_manager);

        self.context.bind_vertex_array(Some(&self.vao));

        for (mut matrix, mesh_index) in matrices {
            let mut _matrix_ptr = glm::value_ptr_mut(&mut matrix);

            // u_matrix
            self.context.uniform_matrix4fv_with_f32_array(Some(&self.uniform.2), false, &mut _matrix_ptr);

            // Draw our shape (Triangles, first_index, count) Our vertex shader will run $count times.
            self.context.draw_arrays(
                WebGl2RenderingContext::TRIANGLES,
                mesh_index.index,
                mesh_index.count,
            );
        }
    }
}