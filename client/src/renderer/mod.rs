use js_sys::WebAssembly;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlProgram, WebGlBuffer, WebGl2RenderingContext, WebGlShader, WebGlUniformLocation, WebGlVertexArrayObject};
use specs::{World, Join};
//use glm;

mod shader;

use self::shader::Shader;
use engine::components;
use engine::components::{Transform, StaticMesh, Solid, Light};
use engine::mesh_manager::{MeshManager, mesh::MeshIndex};
use math::{Vert3, Vert4, Mat4};
use math::glm;
use javascript::get_canvas;

pub struct Renderer {
    attribute: (u32, u32, u32),
    buffer:    (WebGlBuffer, WebGlBuffer, WebGlBuffer),
    context:    WebGl2RenderingContext,
    canvas:     HtmlCanvasElement,
    shader:     Shader,
    ls_shader:  Shader,
    vao:        WebGlVertexArrayObject,
}

impl Renderer {
    pub fn new() -> Result<(Renderer), JsValue> {
        // Gather our canvas from the DOM
        let canvas: HtmlCanvasElement = get_canvas()?;

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
        let ls_shader = Shader::new(
            &context,
            include_str!("glsl/light_source_vertex.glsl"),
            include_str!("glsl/light_source_fragment.glsl"),
        )?;

        // create a vertex array object (stores attribute state)
        let vao = context.create_vertex_array()
            .expect("Could not create a Vertex Array Object.");
        context.bind_vertex_array(Some(&vao));

        // positions in the shader
        let a_position = 0 as u32;
        let a_color = 1 as u32;
        let a_normal = 2 as u32;

        // Attributes in shaders come from buffers, first get the buffer
        let buffer = context.create_buffer().ok_or("failed to create a vertex buffer")?;
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        // color buffer
        let color_buffer = context.create_buffer().ok_or("failed to create a color buffer")?;
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&color_buffer));

        // normal buffer
        let normal_buffer = context.create_buffer().ok_or("failed to create a normal buffer")?;
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&normal_buffer));

        // Cull triangles (counter-clockwise = front facing)
        context.enable(WebGl2RenderingContext::CULL_FACE);

        // Test Depth
        context.enable(WebGl2RenderingContext::DEPTH_TEST);
        context.depth_func(WebGl2RenderingContext::LEQUAL);

        // Return our WebGL object
        Ok(Renderer {
            attribute: (a_position, a_color, a_normal),
            buffer: (buffer, color_buffer, normal_buffer),
            context,
            canvas,
            shader,
            ls_shader,
            vao,
        })
    }

    pub fn draw(&mut self, world: &World, mesh_manager: &mut MeshManager) -> Result<(), JsValue> {

        // Draw over the entire canvas and clear buffer to ur present one
        self.context.clear_color(0.1, 0.1, 0.1, 1.0);
        self.context.clear_depth(1.0);
        self.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);

        // set resolution to the canvas
        Renderer::resize_canvas_to_display_size(&mut self.canvas);
        self.context.viewport(0, 0, self.canvas.width() as i32, self.canvas.height() as i32);

        // TODO: bind vao here?
        self.context.bind_vertex_array(Some(&self.vao));

        if mesh_manager.updated() {
            self.buffer_data(mesh_manager)?;
        }

        // camera stuff
        let mut proj = Renderer::build_projection(
            self.canvas.width() as f32,
            self.canvas.height() as f32,
        );
        let (view_pos, view_target) = Renderer::get_view(&world);

        // basic_shader
        self.shader.use_shader(&self.context);
        self.shader.set_mat4(&self.context,"u_projection", &mut proj);
        self.shader.set_vec3_xyz(&self.context,"u_view_pos", view_pos.x(), view_pos.y(), view_pos.z());
        self.shader.set_vec3_xyz(&self.context,"u_view_target", view_target.x(), view_target.y(), view_target.z());

        self.draw_solids(world, mesh_manager);

        // ls_shader
        self.ls_shader.use_shader(&self.context);
        self.ls_shader.set_mat4(&self.context,"u_projection", &mut proj);
        self.ls_shader.set_vec3_xyz(&self.context,"u_view_pos", view_pos.x(), view_pos.y(), view_pos.z());
        self.ls_shader.set_vec3_xyz(&self.context,"u_view_target", view_target.x(), view_target.y(), view_target.z());

        self.draw_lights(world, mesh_manager);

        Ok(())
    }
    
    // non pub //

    fn resize_canvas_to_display_size(canvas: &mut HtmlCanvasElement) {
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
        self.context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.buffer.2));
        // Bind buffer to generic vertex attribute of the current vertex buffer object
        self.context.vertex_attrib_pointer_with_i32(self.attribute.2, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
        // Buffer_data will copy the data to the GPU memory
        self.context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &normal_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );
        // Attributes need to be enabled before use
        self.context.enable_vertex_attrib_array(self.attribute.2);

        Ok(())
    }

    fn build_projection(width: f32, height: f32) -> Mat4 {
        glm::perspective( width/height, 75.0, 0.1, 200.0)
    }

    fn get_view(world: &World) -> (Vert3, Vert3) {
        let _camera_storage = world.read_storage::<components::Camera>();

        let mut result = (
            Vert3::new(0.0, 0.0, 0.0),
            Vert3::new(0.0, 0.0, 0.0),
        );

        // TODO, avoid using a loop? .get() .get_unchecked()
        for camera in (&_camera_storage).join() {
            result = (camera.target+camera.rotation,camera.target);
            break;
        }

        result
    }

    fn get_vectors(transform: &Transform, mesh: &StaticMesh, mesh_manager: &MeshManager
    ) -> Option<(MeshIndex, Mat4)> {
        if let Some(mesh_index) = mesh_manager.get(&mesh.mesh_id) {
            Some((
                mesh_index,
                 glm::translate(transform.translation)
                 * glm::rotate_x(transform.rotation[0])
                 * glm::rotate_y(transform.rotation[1])
                 * glm::rotate_z(transform.rotation[2])
                 * glm::scale(transform.scale),
            ))
        }
        else {
            None
        }
    }

    fn get_lights(world: &World) -> Vec<(Vert3, Vert4)> {
        let _transform_storage = world.read_storage::<Transform>();
        let _light_storage = world.read_storage::<Light>();

        let mut list: Vec<(Vert3, Vert4)> = Vec::new();

        for (transform, light) in (&_transform_storage, &_light_storage).join() {
            list.push((transform.translation, light.color));
        }

        list
    }

    fn draw_solids(&mut self, world: &World, mesh_manager: &MeshManager){

        let lights = Renderer::get_lights(world);

        if !lights.is_empty() {
            let (pos, color) = lights.get(0).unwrap();
            self.shader.set_vec3_xyz(&self.context, "u_light_pos",pos[0], pos[1], pos[2]);
            self.shader.set_vec3_xyz(&self.context, "u_light_color", color.x(), color.y(), color.z());
        }

        let _transform_storage = world.read_storage::<Transform>();
        let _mesh_storage = world.read_storage::<StaticMesh>();
        let _solid_storage = world.read_storage::<Solid>();

        for (transform, mesh, _) in (&_transform_storage, &_mesh_storage, &_solid_storage).join() {

            if let Some((mesh_index, model)) = Renderer::get_vectors(&transform, &mesh, &mesh_manager) {

                let mut model = model;
                // model data
                self.shader.set_mat4(&self.context, "u_model", &mut model);

                // Draw our shape (Triangles, first_index, count) Our vertex shader will run $count times.
                self.context.draw_arrays(
                    WebGl2RenderingContext::TRIANGLES,
                    mesh_index.index,
                    mesh_index.count,
                );
            }
        }
    }

    fn draw_lights(&mut self, world: &World, mesh_manager: &MeshManager){

        let _transform_storage = world.read_storage::<Transform>();
        let _mesh_storage = world.read_storage::<StaticMesh>();
        let _light_storage = world.read_storage::<Light>();

        for (transform, mesh, light) in (&_transform_storage, &_mesh_storage, &_light_storage).join() {

            if let Some((mesh_index, model)) = Renderer::get_vectors(&transform, &mesh, &mesh_manager) {

                let mut model = model;
                // model data
                self.ls_shader.set_mat4(&self.context, "u_model", &mut model);

                let mut color = light.color;

                // u_color
                self.ls_shader.set_vec4(&self.context, "u_color", &mut color);

                // Draw our shape (Triangles, first_index, count) Our vertex shader will run $count times.
                self.context.draw_arrays(
                    WebGl2RenderingContext::TRIANGLES,
                    mesh_index.index,
                    mesh_index.count,
                );
            }
        }
    }
}