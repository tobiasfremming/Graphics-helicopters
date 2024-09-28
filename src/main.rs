// Uncomment these following global attributes to silence most warnings of "low" interest:

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_code)]
#![allow(unused_mut)]
#![allow(unused_unsafe)]
#![allow(unused_variables)]

extern crate nalgebra_glm as glm;
use std::option::Iter;
use std::{ mem, ptr, os::raw::c_void };
use std::{path, thread};
use std::sync::{Mutex, Arc, RwLock};
use std::time::Instant;


mod shader;
mod util;
mod mesh;
mod scene_graph;

use glm::{normalize_dot, Mat4, Vec3};
use glutin::event::ElementState;
use glutin::event::{Event, WindowEvent, DeviceEvent, KeyboardInput, ElementState::{Pressed, Released}, VirtualKeyCode::{self, *}};
use glutin::event_loop::ControlFlow;
use mesh::{Helicopter, Mesh};
use scene_graph::{Node, SceneNode};



// Camera motion struct
// struct CameraMotion {
//     position: glm::Vec3,  // For position (x, y, z)
//     rotation: glm::Vec3,  // For rotation (angle in radians for each axis)
// }

// impl CameraMotion {
//     fn new() -> CameraMotion {
//         CameraMotion {
//             position: glm::vec3(0.0, 0.0, 0.0),  // Initialize at origin
//             rotation: glm::vec3(0.0, 0.0, 0.0),  // Initialize with no rotation
//         }
//     }
// }

static mut is_helicopter: bool = false;

// initial window size
const INITIAL_SCREEN_W: u32 = 800;
const INITIAL_SCREEN_H: u32 = 600;

// == // Helper functions to make interacting with OpenGL a little bit prettier. You *WILL* need these! // == //

// Get the size of an arbitrary array of numbers measured in bytes
// Example usage:  byte_size_of_array(my_array)
fn byte_size_of_array<T>(val: &[T]) -> isize {
    std::mem::size_of_val(&val[..]) as isize
}

// Get the OpenGL-compatible pointer to an arbitrary array of numbers
// Example usage:  pointer_to_array(my_array)
fn pointer_to_array<T>(val: &[T]) -> *const c_void {
    &val[0] as *const T as *const c_void
}

// Get the size of the given type in bytes
// Example usage:  size_of::<u64>()
fn size_of<T>() -> i32 {
    mem::size_of::<T>() as i32
}

// Get an offset in bytes for n units of type T, represented as a relative pointer
// Example usage:  offset::<u64>(4)
fn offset<T>(n: u32) -> *const c_void {
    (n * mem::size_of::<T>() as u32) as *const T as *const c_void
}

// Get a null pointer (equivalent to an offset of 0)
// ptr::null()




// == // Generate your VAO here
unsafe fn create_vao(vertices: &Vec<f32>, indices: &Vec<u32>, colors: &Vec<f32>, normals: &Vec<f32>) -> u32 {
    

    // Also, feel free to delete comments :)

    // This should:
    // * Generate a VAO and bind it
    // * Generate a VBO and bind it
    // * Fill it with data
    // * Configure a VAP for the data and enable it
    // * Generate a IBO and bind it
    // * Fill it with data
    // * Return the ID of the VAO

    // Generate the VAO
    let mut vao: u32 = 0;
    gl::GenVertexArrays(1, &mut vao);
    gl::BindVertexArray(vao);
    

    

    
    
    

    // Generate the IBO
    let mut ibo: u32 = 0;
    gl::GenBuffers(1, &mut ibo);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
    gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        (indices.len() * std::mem::size_of::<u32>()) as isize,
        indices.as_ptr() as *const c_void,
        gl::STATIC_DRAW,
    );

    // Generate the VBO
    let mut vbo: u32 = 0;
    gl::GenBuffers(1, &mut vbo);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        (vertices.len() * std::mem::size_of::<f32>()) as isize,
        vertices.as_ptr() as *const c_void,
        gl::STATIC_DRAW,
        
    );

    // Configure the VAP
    gl::VertexAttribPointer(
        0,
        3,  // number of type_ to describe a vertex
        gl::FLOAT,
        gl::FALSE,
        3 * std::mem::size_of::<f32>() as i32,
        ptr::null(),
    );
    // Enable the VAP
    gl::EnableVertexAttribArray(0);


    // Generate the VBO for the colors
    let mut color_vbo: u32 = 0;
    gl::GenBuffers(1, &mut color_vbo);
    gl::BindBuffer(gl::ARRAY_BUFFER, color_vbo);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        (colors.len() * std::mem::size_of::<f32>()) as isize,
        colors.as_ptr() as *const c_void,
        gl::STATIC_DRAW,
    );

    gl::VertexAttribPointer(
        1,
        4,                  // 4 components per color (r, g, b, a)
        gl::FLOAT,
        gl::FALSE,
        4 * std::mem::size_of::<f32>() as i32, // stride: 4 floats per color
        ptr::null(),
    );

    gl::EnableVertexAttribArray(1);

    // Generate the VBO for the normals
    let mut normal_vbo: u32 = 0;
    gl::GenBuffers(1, &mut normal_vbo);
    gl::BindBuffer(gl::ARRAY_BUFFER, normal_vbo);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        (normals.len() * std::mem::size_of::<f32>()) as isize,
        normals.as_ptr() as *const c_void,
        gl::STATIC_DRAW,
    );

    gl::VertexAttribPointer(
        2,
        3,                  // x,y,z components per normal
        gl::FLOAT,
        gl::FALSE,
        3 * std::mem::size_of::<f32>() as i32, // stride: 3 floats per normal
        ptr::null(),
    );

    gl::EnableVertexAttribArray(2);
    

    // Enable the VAP
    
    


    vao


}

fn create_square(uv: f32, offset_x: f32, offset_y: f32, offset_z: f32) -> Vec<f32> {
    // Create a square with the given uv, offset_x, offset_y, and offset_z
    let vertices: Vec<f32> = vec![
        // Triangle 1
        -uv + offset_x, -uv + offset_y, offset_z,   // Bottom left
        uv + offset_x, -uv + offset_y, offset_z,    // Bottom right
        -uv + offset_x, uv + offset_y, offset_z,    // Top left

        // Triangle 2 
        uv + offset_x, -uv + offset_y, offset_z,    // Bottom right
        uv + offset_x, uv + offset_y, offset_z,     // Top right
        -uv + offset_x, uv + offset_y, offset_z,    // Top left
    ];
    vertices
}

fn create_box(
    start_point: (f32, f32, f32), 
    width: f32, 
    height: f32, 
    depth: f32
) -> Vec<f32> {
    let (x, y, z) = start_point;
    let point0 = [x, y, z];
    let point1 = [x + width, y, z];
    let point2 = [x, y + height, z];
    let point3 = [x + width, y + height, z];
    let point4 = [x, y, z - depth];
    let point5 = [x + width, y, z - depth];
    let point6 = [x, y + height, z - depth];
    let point7 = [x + width, y + height, z - depth];
    

    let mut vertices: Vec<f32> = Vec::new();

    vertices.extend_from_slice(&point0);
    vertices.extend_from_slice(&point1);
    vertices.extend_from_slice(&point2);

    vertices.extend_from_slice(&point1);
    vertices.extend_from_slice(&point3);
    vertices.extend_from_slice(&point2);


    vertices.extend_from_slice(&point1);
    vertices.extend_from_slice(&point5);
    vertices.extend_from_slice(&point3);

    vertices.extend_from_slice(&point5);
    vertices.extend_from_slice(&point7);
    vertices.extend_from_slice(&point3);


    vertices.extend_from_slice(&point5);
    vertices.extend_from_slice(&point4);
    vertices.extend_from_slice(&point7);

    vertices.extend_from_slice(&point4);
    vertices.extend_from_slice(&point6);
    vertices.extend_from_slice(&point7);


    vertices.extend_from_slice(&point4);
    vertices.extend_from_slice(&point0);
    vertices.extend_from_slice(&point6);

    vertices.extend_from_slice(&point0);
    vertices.extend_from_slice(&point2);
    vertices.extend_from_slice(&point6);


    vertices.extend_from_slice(&point6);
    vertices.extend_from_slice(&point7);
    vertices.extend_from_slice(&point2);

    vertices.extend_from_slice(&point7);
    vertices.extend_from_slice(&point3);
    vertices.extend_from_slice(&point2);


    vertices.extend_from_slice(&point4);
    vertices.extend_from_slice(&point5);
    vertices.extend_from_slice(&point0);

    vertices.extend_from_slice(&point5);
    vertices.extend_from_slice(&point1);
    vertices.extend_from_slice(&point0);


    vertices.extend_from_slice(&point2);
    vertices.extend_from_slice(&point3);
    vertices.extend_from_slice(&point6);

    vertices.extend_from_slice(&point3);
    vertices.extend_from_slice(&point7);
    vertices.extend_from_slice(&point6);







    vertices
}

// never mind this just now
fn draw_object(mesh: &Mesh, vao: u32, indices_lenght: i32) {
    unsafe {
        gl::BindVertexArray(vao);
        gl::Uniform1i(2, mesh.is_helicopter as i32);
        gl::DrawElements(
            gl::TRIANGLES,
            indices_lenght as i32,
            gl::UNSIGNED_INT,
            std::ptr::null()
        );
    }


}

unsafe fn get_vao_from_mesh(path: &str ) -> (u32, u32){
    // Load the texture, and create the vao
    let model: Mesh = mesh::Terrain::load(path);
        let vertices: Vec<f32> = model.vertices;
        let indices: Vec<u32> = model.indices;
        let colors: Vec<f32> = model.colors;
        let normals: Vec<f32> = model.normals;

        // Create the VAO
        let vao: u32 =create_vao(&vertices, &indices, &colors, &normals);
        

        (vao, indices.len() as u32)

}




unsafe fn draw_scene(node: &scene_graph::SceneNode,
    view_projection_matrix: &glm::Mat4,
    transformation_so_far: &glm::Mat4, shader_program_id: u32) {
    // Perform any logic needed before drawing the node
    // Check if node is drawable, if so: set uniforms, bind VAO and draw VAO

    // The node should be drawable if it has a VAO ID not equal to 0
    if node.index_count != -1 {

        
        // Compute the transformation matrix for the current node
        let mut transformation_matrix = glm::identity();

        //transformation_matrix = glm::translate(&transformation_matrix, &-node.reference_point);
        println!("node.position: {:?}", node.position);
        println!("node.reference_point: {:?}", node.reference_point);
        transformation_matrix = glm::translate(&transformation_matrix, &node.position);
        // Rotate
        transformation_matrix = glm::rotate(&transformation_matrix, node.rotation[0], &glm::vec3(1.0, 0.0, 0.0));
        transformation_matrix = glm::rotate(&transformation_matrix, node.rotation[1], &glm::vec3(0.0, 1.0, 0.0));
        transformation_matrix = glm::rotate(&transformation_matrix, node.rotation[2], &glm::vec3(0.0, 0.0, 1.0));
    
        transformation_matrix = glm::translate(&transformation_matrix, &-node.position);
        
        //transformation_matrix = glm::translate(&transformation_matrix, &-node.reference_point);
        // Scale
        //transformation_matrix = glm::scale(&transformation_matrix, &node.scale);
        // Combine the transformation matrix with the transformation so far
        let transformation_so_far = transformation_matrix * transformation_so_far;
        let mixed_matrix =  view_projection_matrix * transformation_so_far;

        // Pass the transformation matrix to the shader
        let transformation_so_far_location = gl::GetUniformLocation(shader_program_id, "transformation_so_far\0".as_ptr() as *const i8);
        gl::UniformMatrix4fv(transformation_so_far_location, 1, gl::FALSE, transformation_so_far.as_ptr());
        
        // Pass the view-projection matrix to the shader
        let view_projection_matrix_location = gl::GetUniformLocation(shader_program_id, "view_projection_matrix\0".as_ptr() as *const i8);
        gl::UniformMatrix4fv(view_projection_matrix_location, 1, gl::FALSE, mixed_matrix.as_ptr());

        // Bind the VAO
        gl::BindVertexArray(node.vao_id);

        //gl::Uniform1i(2, node.vao_id.mesh.is_helicopter as i32);


        // Draw the VAO
        gl::DrawElements(
            gl::TRIANGLES,
            node.index_count,
            gl::UNSIGNED_INT,
            std::ptr::null()
        );

    }

    // Recurse
    for &child in &node.children {
    draw_scene(&*child, view_projection_matrix, transformation_so_far, shader_program_id);
    }
    }



fn main() {
    // Set up the necessary objects to deal with windows and event handling
    let start_time = Instant::now();


    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Gloom-rs")
        .with_resizable(true)
        .with_inner_size(glutin::dpi::LogicalSize::new(INITIAL_SCREEN_W, INITIAL_SCREEN_H));
    let cb = glutin::ContextBuilder::new()
        .with_vsync(true);
    let windowed_context = cb.build_windowed(wb, &el).unwrap();
    // Uncomment these if you want to use the mouse for controls, but want it to be confined to the screen and/or invisible.
    // windowed_context.window().set_cursor_grab(true).expect("failed to grab cursor");
    // windowed_context.window().set_cursor_visible(false);

    // Set up a shared vector for keeping track of currently pressed keys
    let arc_pressed_keys = Arc::new(Mutex::new(Vec::<VirtualKeyCode>::with_capacity(10)));
    // Make a reference of this vector to send to the render thread
    let pressed_keys = Arc::clone(&arc_pressed_keys);

    // Set up shared tuple for tracking mouse movement between frames
    let arc_mouse_delta = Arc::new(Mutex::new((0f32, 0f32)));
    // Make a reference of this tuple to send to the render thread
    let mouse_delta = Arc::clone(&arc_mouse_delta);

    // Set up shared tuple for tracking changes to the window size
    let arc_window_size = Arc::new(Mutex::new((INITIAL_SCREEN_W, INITIAL_SCREEN_H, false)));
    // Make a reference of this tuple to send to the render thread
    let window_size = Arc::clone(&arc_window_size);

    // Spawn a separate thread for rendering, so event handling doesn't block rendering
    let render_thread = thread::spawn(move || {
        // Acquire the OpenGL Context and load the function pointers.
        // This has to be done inside of the rendering thread, because
        // an active OpenGL context cannot safely traverse a thread boundary
        let context = unsafe {
            let c = windowed_context.make_current().unwrap();
            gl::load_with(|symbol| c.get_proc_address(symbol) as *const _);
            c
        };

        let mut window_aspect_ratio = INITIAL_SCREEN_W as f32 / INITIAL_SCREEN_H as f32;

        // Set up openGL
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::Enable(gl::CULL_FACE);
            gl::Disable(gl::MULTISAMPLE);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::DebugMessageCallback(Some(util::debug_callback), ptr::null());
            // Transparency stuff
            // gl::Enable(gl::BLEND);
            // gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            // Print some diagnostics
            println!("{}: {}", util::get_gl_string(gl::VENDOR), util::get_gl_string(gl::RENDERER));
            println!("OpenGL\t: {}", util::get_gl_string(gl::VERSION));
            println!("GLSL\t: {}", util::get_gl_string(gl::SHADING_LANGUAGE_VERSION));
        }

        // == // Set up your VAO around here

        let my_vao = unsafe { 1337 };


        // == // Set up your shaders here

        // Basic usage of shader helper:
        // The example code below creates a 'shader' object.
        // It which contains the field `.program_id` and the method `.activate()`.
        // The `.` in the path is relative to `Cargo.toml`.
        // This snippet is not enough to do the exercise, and will need to be modified (outside
        // of just using the correct path), but it only needs to be called once

        let shader_program = unsafe {
            shader::ShaderBuilder::new()
                .attach_file("./shaders/simple.vert")
                .attach_file("./shaders/simple.frag")
                .link()
        };
    
        // Activate the shader program
        unsafe {
            shader_program.activate();
        }

    

        // load the terrain
        // let terrain: Mesh = mesh::Terrain::load("./resources/lunarsurface.obj");
        // let vertices: Vec<f32> = terrain.vertices;
        // let terrain_indices: Vec<u32> = terrain.indices;
        // let colors: Vec<f32> = terrain.colors;
        // let normals: Vec<f32> = terrain.normals;

        // // Create the VAO
        // let terrain_vao: u32 = unsafe {
        //     create_vao(&vertices, &terrain_indices, &colors, &normals)
        // };
        let (terrain_vao, terrain_indices_lenght) = unsafe { get_vao_from_mesh("./resources/lunarsurface.obj") };
        


        // load the helicopter
        let helicopter: Helicopter = mesh::Helicopter::load("./resources/helicopter.obj");
        let mut body: Mesh = helicopter.body;
        body.is_helicopter = true;
        let vertices: Vec<f32> = body.vertices;
        let body_indices: Vec<u32> = body.indices;
        let colors: Vec<f32> = body.colors;
        let normals: Vec<f32> = body.normals;
        

        let helicopter_body_Vao: u32 = unsafe {
            create_vao(&vertices, &body_indices, &colors, &normals)
        };

        let mut helicopter_door: Mesh = helicopter.door;
        helicopter_door.is_helicopter = true;
        let vertices: Vec<f32> = helicopter_door.vertices;
        let door_indices: Vec<u32> = helicopter_door.indices;
        let colors: Vec<f32> = helicopter_door.colors;
        let normals: Vec<f32> = helicopter_door.normals;

        let helicopter_door_Vao: u32 = unsafe {
            create_vao(&vertices, &door_indices, &colors, &normals)
        };

        let mut helicopter_main_rotor: Mesh = helicopter.main_rotor;
        helicopter_main_rotor.is_helicopter = true;
        let vertices: Vec<f32> = helicopter_main_rotor.vertices;
        let main_rotor_indices: Vec<u32> = helicopter_main_rotor.indices;
        let colors: Vec<f32> = helicopter_main_rotor.colors;
        let normals: Vec<f32> = helicopter_main_rotor.normals;

        let helicopter_main_rotor_Vao: u32 = unsafe {
            create_vao(&vertices, &main_rotor_indices, &colors, &normals)
        };

        let mut helicopter_tail_rotor: Mesh = helicopter.tail_rotor;
        helicopter_tail_rotor.is_helicopter = true;
        let vertices: Vec<f32> = helicopter_tail_rotor.vertices;
        let tail_rotor_indices: Vec<u32> = helicopter_tail_rotor.indices;
        let colors: Vec<f32> = helicopter_tail_rotor.colors;
        let normals: Vec<f32> = helicopter_tail_rotor.normals;

        let helicopter_tail_rotor_Vao: u32 = unsafe {
            create_vao(&vertices, &tail_rotor_indices, &colors, &normals)
        };


        let mut root_node: Node = SceneNode::new();
        let mut terrain_node: Node = SceneNode::from_vao(terrain_vao, terrain_indices_lenght as i32);
        let mut helicopter_body_node: Node = SceneNode::from_vao(helicopter_body_Vao, body_indices.len() as i32);
        let mut helicopter_door_node: Node = SceneNode::from_vao(helicopter_door_Vao, door_indices.len() as i32);
        let mut helicopter_main_rotor_node: Node = SceneNode::from_vao(helicopter_main_rotor_Vao, main_rotor_indices.len() as i32);
        let mut helicopter_tail_rotor_node: Node = SceneNode::from_vao(helicopter_tail_rotor_Vao, tail_rotor_indices.len() as i32);

        
        helicopter_body_node.add_child(&helicopter_door_node);
        helicopter_body_node.add_child(&helicopter_main_rotor_node);
        helicopter_body_node.add_child(&helicopter_tail_rotor_node);
        terrain_node.add_child(&helicopter_body_node);
        root_node.add_child(&terrain_node);

        terrain_node.position = glm::vec3(0.0, 0.0, 0.0);
        helicopter_body_node.position = glm::vec3(0.0, 0.0, 0.0); // how do i get the correct position?
        helicopter_door_node.position = glm::vec3(0.0, 0.0, 0.0);
        helicopter_main_rotor_node.position = glm::vec3(0.0, 0.0, 0.0);
        helicopter_tail_rotor_node.position = glm::vec3(0.35, 2.3, 10.4);

        helicopter_body_node.rotation = glm::vec3(0.0, 0.0, 0.0); // rotation around y axis
        helicopter_door_node.rotation = glm::vec3(0.0, 0.0, 0.0);
        helicopter_main_rotor_node.rotation = glm::vec3(0.0, 0.1, 0.0);
        helicopter_tail_rotor_node.rotation = glm::vec3(0.1, 0.0, 0.0);

    

        

        let mut pitch: f32 = 0.0; // rotation around x-axis
        let mut yaw: f32 = -90.0_f32.to_radians(); // rotation around y-axis
        let mut roll:f32  = 0.0; // rotation around z-axis

        let mut camera_position:Vec3 = glm::vec3(0.0, 0.0, 0.0);
        let mut camera_front: Vec3 = glm::vec3(0.0, 0.0, -1.0);
        let camera_up = glm::vec3(0.0, 1.0, 0.0);

        
        


        // Used to demonstrate keyboard handling for exercise 2.
        let mut _arbitrary_number = 0.0; // feel free to remove


        // The main rendering loop
        let first_frame_time = std::time::Instant::now();
        let mut previous_frame_time = first_frame_time;
        loop {
            // Compute time passed since the previous frame and since the start of the program
            let now = std::time::Instant::now();
            let elapsed = now.duration_since(first_frame_time).as_secs_f32();
            let delta_time = now.duration_since(previous_frame_time).as_secs_f32();
            previous_frame_time = now;


            let rotor_rotation_speed = 10.0; 
            helicopter_main_rotor_node.rotation[1] += rotor_rotation_speed * delta_time;
            helicopter_tail_rotor_node.rotation[0] += rotor_rotation_speed * delta_time;

            // Handle resize events
            if let Ok(mut new_size) = window_size.lock() {
                if new_size.2 {
                    context.resize(glutin::dpi::PhysicalSize::new(new_size.0, new_size.1));
                    window_aspect_ratio = new_size.0 as f32 / new_size.1 as f32;
                    (*new_size).2 = false;
                    println!("Window was resized to {}x{}", new_size.0, new_size.1);
                    unsafe { gl::Viewport(0, 0, new_size.0 as i32, new_size.1 as i32); }
                }
            }


            let speed: f32 = 50.0 * delta_time;  
            let rotation_speed: f32 = 1.0 * delta_time;  
            

            // Handle keyboard input
            if let Ok(keys) = pressed_keys.lock() {
                for key in keys.iter() {
                    // if key is pressed, move the camera



                    match key {
                        VirtualKeyCode::W => camera_position += speed * camera_front,  // Move forward
                        VirtualKeyCode::S => camera_position -= speed * camera_front,  // Move backward
                        VirtualKeyCode::A => camera_position -= glm::normalize(&glm::cross(&camera_front, &camera_up)) * speed,   // Move left
                        VirtualKeyCode::D => camera_position += glm::normalize(&glm::cross(&camera_front, &camera_up)) * speed,  // Move right
                        VirtualKeyCode::Space => camera_position[1] += speed,  // Move up
                        VirtualKeyCode::LShift => camera_position[1] -= speed,  // Move down
                        
                        // Rotation
                        VirtualKeyCode::Up => pitch += rotation_speed,  // Rotate up (around X-axis)
                        VirtualKeyCode::Down => pitch -= rotation_speed,  // Rotate down (around X-axis)
                        VirtualKeyCode::Left => yaw -= rotation_speed,  // Rotate left (around Y-axis)
                        VirtualKeyCode::Right => yaw += rotation_speed,  // Rotate right (around Y-axis)
                        // The `VirtualKeyCode` enum is defined here:
                        //    https://docs.rs/winit/0.25.0/winit/event/enum.VirtualKeyCode.html

_ => { }
                    
                }
                // Clamp the pitch value to prevent camera flip
                if pitch > 89.0_f32.to_radians() {
                    pitch = 89.0_f32.to_radians();
                }
                if pitch < -89.0_f32.to_radians() {
                    pitch = -89.0_f32.to_radians();
                    }
                }
            }
            // Handle mouse movement. delta contains the x and y movement of the mouse since last frame in pixels
            if let Ok(mut delta) = mouse_delta.lock() {

                // == // Optionally access the accumulated mouse movement between
                // == // frames here with `delta.0` and `delta.1`

                *delta = (0.0, 0.0); // reset when done
            }

                        // default handler:
                        
            // == // Please compute camera transforms here (exercise 2 & 3)
            let front = glm::vec3(
                yaw.cos() * pitch.cos(),
                pitch.sin(),
                yaw.sin() * pitch.cos()
            );
            camera_front = glm::normalize(&front);

            // Compute the view matrix
            let view = glm::look_at(
                &camera_position,
                &(camera_position + camera_front),
                &camera_up
            );

            // Compute the projection matrix
            let aspect = window_aspect_ratio;
            let fovy = 45.0; // FOV (field of view) in degrees
            let near = 1.0; // near plane
            let far = 1000.0; // far plane

            let perspective_projection: glm::Mat4 = glm::perspective(    
                aspect, 
                fovy, 
                near, 
                far
            );

            

            // Compute the view matrix
            // let translation = glm::translation(&glm::vec3(camera_position[0], camera_position[1], camera_position[2]));
            // let rotation_x = glm::rotation(camera_front[0], &glm::vec3(1.0, 0.0, 0.0));
            // let rotation_y = glm::rotation(camera_front[1], &glm::vec3(0.0, 1.0, 0.0));
            // let camera_transformation_matrix = translation * rotation_x * rotation_y;
            
            let identity_matrix: glm::Mat4 = glm::identity();  
            let mut mixed_matrix = perspective_projection * view * identity_matrix;

            // to fix light
            let model_matrix: glm::Mat4 = glm::identity(); // TODO: Add transformations here, pass to vertex shader like its done below




            unsafe {
                // Clear the color and depth buffers
                gl::ClearColor(0.035, 0.046, 0.078, 1.0); // night sky
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);


                
                // Pass the 'elapsed_time' to the 'time' uniform in the shader
                //let time_uniform_location = gl::GetUniformLocation(shader_program.program_id, "time".as_ptr() as *const i8);
                gl::Uniform1f(1, elapsed);

                
                
          

                // == // Issue the necessary gl:: commands to draw your scene here
                shader_program.activate();
                // Pass the 'elapsed_time' to the 'time' uniform in the shader
                
                draw_scene(&root_node, &mixed_matrix, &identity_matrix, shader_program.program_id);
                
                
                




                // Bind the VAO

                // gl::BindVertexArray(helicopter_body_Vao);
                // is_helicopter = body.is_helicopter;
                // gl::Uniform1i(2, is_helicopter as i32);
                // gl::DrawElements(
                //     gl::TRIANGLES,
                //     body_indices.len() as i32,
                //     gl::UNSIGNED_INT,
                //     std::ptr::null()
                // );


                // Unbind the VAO (optional, good practice to prevent accidental modifications)
                gl::BindVertexArray(0);

            }

            // Display the new color buffer on the display
            context.swap_buffers().unwrap(); // we use "double buffering" to avoid artifacts
        }
    });


    // == //
    // == // From here on down there are only internals.
    // == //


    // Keep track of the health of the rendering thread
    let render_thread_healthy = Arc::new(RwLock::new(true));
    let render_thread_watchdog = Arc::clone(&render_thread_healthy);
    thread::spawn(move || {
        if !render_thread.join().is_ok() {
            if let Ok(mut health) = render_thread_watchdog.write() {
                println!("Render thread panicked!");
                *health = false;
            }
        }
    });

    // Start the event loop -- This is where window events are initially handled
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Terminate program if render thread panics
        if let Ok(health) = render_thread_healthy.read() {
            if *health == false {
                *control_flow = ControlFlow::Exit;
            }
        }

        match event {
            Event::WindowEvent { event: WindowEvent::Resized(physical_size), .. } => {
                println!("New window size received: {}x{}", physical_size.width, physical_size.height);
                if let Ok(mut new_size) = arc_window_size.lock() {
                    *new_size = (physical_size.width, physical_size.height, true);
                }
            }
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            }
            // Keep track of currently pressed keys to send to the rendering thread
            Event::WindowEvent { event: WindowEvent::KeyboardInput {
                    input: KeyboardInput { state: key_state, virtual_keycode: Some(keycode), .. }, .. }, .. } => {

                if let Ok(mut keys) = arc_pressed_keys.lock() {
                    match key_state {
                        Released => {
                            if keys.contains(&keycode) {
                                let i = keys.iter().position(|&k| k == keycode).unwrap();
                                keys.remove(i);
                            }
                        },
                        Pressed => {
                            if !keys.contains(&keycode) {
                                keys.push(keycode);
                            }
                        }
                    }
                }

                // Handle Escape and Q keys separately
                match keycode {
                    Escape => { *control_flow = ControlFlow::Exit; }
                    Q      => { *control_flow = ControlFlow::Exit; }
                    _      => { }
                }
            }
            Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => {
                // Accumulate mouse movement
                if let Ok(mut position) = arc_mouse_delta.lock() {
                    *position = (position.0 + delta.0 as f32, position.1 + delta.1 as f32);
                }
            }
            _ => { }
        }
    });
}
