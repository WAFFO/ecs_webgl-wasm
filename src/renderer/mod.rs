use js_sys::WebAssembly;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGl2RenderingContext, WebGlShader, WebGlUniformLocation};
use specs::{World, Join};
use glm;
use glm::vec3;

mod shader;

use self::shader::Shader;
use engine::components;
use engine::mesh_manager::{MeshManager, mesh::MeshIndex};
use javascript::get_canvas;

pub struct Renderer {
    attribute: (u32, u32, u32),
    buffer:    (web_sys::WebGlBuffer, web_sys::WebGlBuffer, web_sys::WebGlBuffer),
    context:    web_sys::WebGl2RenderingContext,
    canvas:     web_sys::HtmlCanvasElement,
    shader:     Shader,
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
        let shader = Shader::new(
            &context,
            include_str!("glsl/basic_vertex.glsl"),
            include_str!("glsl/basic_fragment.glsl"),
        )?;

        // A WebGLProgram is the object that holds the two compiled shaders
        shader.use_shader(&context);

        // create a vertex array object (stores attribute state)
        let vao = context.create_vertex_array()
            .expect("Could not create a Vertex Array Object.");
        context.bind_vertex_array(Some(&vao));

        // positions in the shader
        let a_position = 0 as u32;
        let a_color = 1 as u32;
//        attribute.2 = 2 as u32;

        // Attributes in shaders come from buffers, first get the buffer
        let buffer = context.create_buffer().ok_or("failed to create a vertex buffer")?;
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        // color buffer
        let color_buffer = context.create_buffer().ok_or("failed to create a color buffer")?;
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&color_buffer));

        // normal buffer
        let normal_buffer = context.create_buffer().ok_or("failed to create a normal buffer")?;
//        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&normal_buffer));

        // Cull triangles (counter-clockwise = front facing)
        context.enable(WebGl2RenderingContext::CULL_FACE);

        // Test Depth
        context.enable(WebGl2RenderingContext::DEPTH_TEST);
        context.depth_func(WebGl2RenderingContext::LEQUAL);

        // Return our WebGL object
        Ok(Renderer {
            attribute: (a_position, a_color, 0),
            buffer: (buffer, color_buffer, normal_buffer),
            context,
            canvas,
            shader,
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
        let mut view = Renderer::build_view(&world);

        // u_projection
        self.shader.set_mat4(&self.context,"u_projection", &mut proj);

        // u_view
        self.shader.set_mat4(&self.context,"u_view", &mut view);

        self.draw_arrays(world, mesh_manager);

        Ok(())
    }
    
    // non pub //

    fn resize_canvas_to_display_size(canvas: &mut web_sys::HtmlCanvasElement) {
        let w = canvas.client_width() as u32;
        let h = canvas.client_height() as u32;
        if canvas.width() != w || canvas.height() != h {
            canvas.set_width(w);
            canvas.set_height(h);
        }
    }

    fn buffer_data(&self, mesh_manager: &mut MeshManager) -> Result<(), JsValue> {

        let (vertices, colors, normals) = mesh_manager.get_storage();
        let vertices = vertices.as_slice();
        let colors = colors.as_slice();
//        let indices = indices.as_slice();
        let normals = normals.as_slice();

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
        let normals_location = normals.as_ptr() as u32 / 4;
        let normal_array = js_sys::Float32Array::new(&memory_buffer)
            .subarray(normals_location, normals_location + normals.len() as u32);

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

        // start of normal binding
//        self.context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.buffer.2));
//        // Bind buffer to generic vertex attribute of the current vertex buffer object
//        self.context.vertex_attrib_pointer_with_i32(self.attribute.2, 4, WebGl2RenderingContext::FLOAT, false, 0, 0);
//        // Buffer_data will copy the data to the GPU memory
//        self.context.buffer_data_with_array_buffer_view(
//            WebGl2RenderingContext::ARRAY_BUFFER,
//            &normal_array,
//            WebGl2RenderingContext::STATIC_DRAW,
//        );
//        // Attributes need to be enabled before use
//        self.context.enable_vertex_attrib_array(self.attribute.2);

        Ok(())
    }

    fn build_projection() -> glm::Mat4 {
        glm::perspective(1.0, 45.0, 0.1, 200.0)
    }

    fn build_view(world: &World) -> glm::Mat4 {
        let _camera_storage = world.read_storage::<components::Camera>();

        let mut result = glm::Mat4::identity();

        // TODO, avoid using a loop? .get() .get_unchecked()
        for camera in (&_camera_storage).join() {
            result = glm::look_at(
                &(camera.target+camera.rotation),
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
            if let Some(mesh_index) = mesh_manager.get(&mesh.mesh_id) {
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

    fn draw_arrays(&mut self, world: &World, mesh_manager: &MeshManager){
        let matrices = Renderer::build_matrices(world, mesh_manager);

        self.context.bind_vertex_array(Some(&self.vao));

        for (mut matrix, mesh_index) in matrices {

            // u_matrix
            self.shader.set_mat4(&self.context,"u_matrix",&mut matrix);

            // Draw our shape (Triangles, first_index, count) Our vertex shader will run $count times.
            self.context.draw_arrays(
                WebGl2RenderingContext::TRIANGLES,
                mesh_index.index,
                mesh_index.count,
            );
        }
    }
}