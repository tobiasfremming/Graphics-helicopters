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
use glutin::window::CursorGrabMode;
use std::{path, thread};
use std::sync::{Mutex, Arc, RwLock};
use std::time::Instant;



mod shader;
mod util;
mod mesh;
mod scene_graph;
mod toolbox;

use glm::{normalize_dot, vec3, Mat3, Mat4, Vec3};
use glutin::event::{ElementState, MouseScrollDelta};
use glutin::event::{Event, WindowEvent, DeviceEvent, KeyboardInput, ElementState::{Pressed, Released}, VirtualKeyCode::{self, *}};
use glutin::event_loop::ControlFlow;
use mesh::{Helicopter, Mesh};
use scene_graph::{Node, SceneNode};
use toolbox::Heading;



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






unsafe fn draw_scene(
        node: &scene_graph::SceneNode,
        view_projection_matrix: &glm::Mat4,
        transformation_so_far: &glm::Mat4,
        shader_program_id: u32,
    ) {
        // Compute the transformation matrix for the current node
        let mut transformation_matrix = glm::identity();

    
        // Apply node's transformations
        transformation_matrix = glm::translate(&transformation_matrix, &node.position);
        transformation_matrix = glm::translate(&transformation_matrix, &node.reference_point);
    
        // Apply rotations
        transformation_matrix = glm::rotate(&transformation_matrix, node.rotation[0], &glm::vec3(1.0, 0.0, 0.0));
        transformation_matrix = glm::rotate(&transformation_matrix, node.rotation[1], &glm::vec3(0.0, 1.0, 0.0));
        transformation_matrix = glm::rotate(&transformation_matrix, node.rotation[2], &glm::vec3(0.0, 0.0, 1.0));
    
        // Apply scaling
        transformation_matrix = glm::scale(&transformation_matrix, &node.scale);
    
        // Move back by reference point if necessary
        transformation_matrix = glm::translate(&transformation_matrix, &-node.reference_point);
        transformation_matrix = glm::translate(&transformation_matrix, &-&node.position);
    

        // Combine with the transformation so far
        let updated_transformation = transformation_so_far * transformation_matrix;
        let mixed_matrix = view_projection_matrix * updated_transformation;
        let normal_matrix: Mat3 = updated_transformation.fixed_slice::<3, 3>(0, 0).into();
        // Pass matrices to the shader
        let transformation_so_far_location = gl::GetUniformLocation(
            shader_program_id,
            "transformation_so_far\0".as_ptr() as *const i8,
        );
        gl::UniformMatrix3fv(
            transformation_so_far_location,
            1,
            gl::FALSE,
            normal_matrix.as_ptr(),
        );
    
        let view_projection_matrix_location = gl::GetUniformLocation(
            shader_program_id,
            "view_projection_matrix\0".as_ptr() as *const i8,
        );
        gl::UniformMatrix4fv(
            view_projection_matrix_location,
            1,
            gl::FALSE,
            mixed_matrix.as_ptr(),
        );
        gl::Uniform1i(2, node.is_helicopter as i32);
    
        // Draw the node if it's drawable
        if node.index_count != -1 {
            gl::BindVertexArray(node.vao_id);
            gl::DrawElements(
                gl::TRIANGLES,
                node.index_count,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
    
        // Recurse with the updated transformation
        for &child in &node.children {
            draw_scene(&*child, view_projection_matrix, &updated_transformation, shader_program_id);
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
    
    windowed_context.window().set_fullscreen(Some(glutin::window::Fullscreen::Borderless(windowed_context.window().current_monitor())));
    windowed_context.window().set_cursor_grab(CursorGrabMode::Confined).expect("failed to grab cursor");
    windowed_context.window().set_cursor_visible(false);

    // if unsafe{mouse_enabled}{
    //     windowed_context.window().set_cursor_grab(CursorGrabMode::Confined).expect("failed to grab cursor");
    //     windowed_context.window().set_cursor_visible(false);
    // }
    // else {
    //     windowed_context.window().set_cursor_grab(CursorGrabMode::None).expect("failed to grab cursor");
    //     windowed_context.window().set_cursor_visible(false);
    // }
    
    

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
        let vertices: Vec<f32> = body.vertices;
        let body_indices: Vec<u32> = body.indices;
        let colors: Vec<f32> = body.colors;
        let normals: Vec<f32> = body.normals;
        

        let helicopter_body_Vao: u32 = unsafe {
            create_vao(&vertices, &body_indices, &colors, &normals)
        };

        let mut helicopter_door: Mesh = helicopter.door;
        let vertices: Vec<f32> = helicopter_door.vertices;
        let door_indices: Vec<u32> = helicopter_door.indices;
        let colors: Vec<f32> = helicopter_door.colors;
        let normals: Vec<f32> = helicopter_door.normals;

        let helicopter_door_Vao: u32 = unsafe {
            create_vao(&vertices, &door_indices, &colors, &normals)
        };

        let mut helicopter_main_rotor: Mesh = helicopter.main_rotor;
        let vertices: Vec<f32> = helicopter_main_rotor.vertices;
        let main_rotor_indices: Vec<u32> = helicopter_main_rotor.indices;
        let colors: Vec<f32> = helicopter_main_rotor.colors;
        let normals: Vec<f32> = helicopter_main_rotor.normals;

        let helicopter_main_rotor_Vao: u32 = unsafe {
            create_vao(&vertices, &main_rotor_indices, &colors, &normals)
        };

        let mut helicopter_tail_rotor: Mesh = helicopter.tail_rotor;
        let vertices: Vec<f32> = helicopter_tail_rotor.vertices;
        let tail_rotor_indices: Vec<u32> = helicopter_tail_rotor.indices;
        let colors: Vec<f32> = helicopter_tail_rotor.colors;
        let normals: Vec<f32> = helicopter_tail_rotor.normals;

        let helicopter_tail_rotor_Vao: u32 = unsafe {
            create_vao(&vertices, &tail_rotor_indices, &colors, &normals)
        };


        let mut root_node: Node = SceneNode::new();
        let mut terrain_node: Node = SceneNode::from_vao(terrain_vao, terrain_indices_lenght as i32);
        let mut camera_node: Node = SceneNode::new();
        

        terrain_node.position = glm::vec3(0.0, 0.0, 0.0);
        
        let mut aniamtion_nodes: Vec<Node> = vec![];
        let mut helicopter_body_nodes: Vec<Node>  = vec![];
        let mut helicopter_door_nodes: Vec<Node>  = vec![];
        let mut helicopter_main_rotor_nodes: Vec<Node>  = vec![];
        let mut helicopter_tail_rotor_nodes: Vec<Node>  = vec![];

        // loop through a number of helicopters to create them. 

        for i in 0..5 {

            
            let mut aniamtion_node: Node = SceneNode::new();
            
            let mut helicopter_body_node: Node = SceneNode::from_vao(helicopter_body_Vao, body_indices.len() as i32);
            let mut helicopter_door_node: Node = SceneNode::from_vao(helicopter_door_Vao, door_indices.len() as i32);
            let mut helicopter_main_rotor_node: Node = SceneNode::from_vao(helicopter_main_rotor_Vao, main_rotor_indices.len() as i32);
            let mut helicopter_tail_rotor_node: Node = SceneNode::from_vao(helicopter_tail_rotor_Vao, tail_rotor_indices.len() as i32);

            helicopter_body_node.position = glm::vec3(0.0, 0.0, 0.0); // how do i get the correct position?
            helicopter_body_node.is_helicopter = true;
            helicopter_door_node.position = glm::vec3(0.0, 0.0, 0.0);
            helicopter_door_node.is_helicopter = true;
            helicopter_main_rotor_node.position = glm::vec3(0.0, 0.0, 0.0);
            helicopter_main_rotor_node.is_helicopter = true;
            helicopter_tail_rotor_node.reference_point = glm::vec3(0.35, 2.3, 10.4);
            helicopter_tail_rotor_node.is_helicopter = true;

            helicopter_body_node.rotation = glm::vec3(0.0, 0.0, 0.0); // rotation around y axis
            helicopter_door_node.rotation = glm::vec3(0.0, 0.0, 0.0);
            helicopter_main_rotor_node.rotation = glm::vec3(0.0, 0.1, 0.0);
            helicopter_tail_rotor_node.rotation = glm::vec3(0.1, 0.0, 0.0);


            
            helicopter_body_node.add_child(&helicopter_door_node);
            helicopter_body_node.add_child(&helicopter_main_rotor_node);
            helicopter_body_node.add_child(&helicopter_tail_rotor_node);
            
            
            aniamtion_node.position = glm::vec3(100.0, 500.0, 100.0);
            aniamtion_node.add_child(&helicopter_body_node);
            terrain_node.add_child(&aniamtion_node);

            
            
            terrain_node.add_child(&aniamtion_node);
            aniamtion_nodes.push(aniamtion_node);
            
            
            
            
            

            
            helicopter_body_nodes.push(helicopter_body_node);
            helicopter_door_nodes.push(helicopter_door_node);
            helicopter_main_rotor_nodes.push(helicopter_main_rotor_node);
            helicopter_tail_rotor_nodes.push(helicopter_tail_rotor_node);


        }
        root_node.add_child(&terrain_node);

        let mut camera_node: Node = SceneNode::new();
        camera_node.position = glm::vec3(0.0, 0.0, 0.0);
        camera_node.rotation = glm::vec3(0.0, 0.0, 0.0);
        terrain_node.add_child(&camera_node);

        


        

        

        let mut pitch: f32 = 0.0; // rotation around x-axis
        let mut yaw: f32 = -90.0_f32.to_radians(); // rotation around y-axis
        let mut roll:f32  = 0.0; // rotation around z-axis

        
        let mut camera_position:Vec3 = glm::vec3(0.0, 0.0, 0.0);
        //let mut camera_front: Vec3 = glm::vec3(0.0, 0.0, -1.0);
        let camera_up = glm::vec3(0.0, 1.0, 0.0);

        let mouse_sensitivity = 0.003;
        let mut mouse_enabled = true;



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
            let helicopter_animation_offset: f32 = 2.0;

            // Update the rotation of the rotors
            for i in 0..helicopter_body_nodes.len() {
                helicopter_main_rotor_nodes[i].rotation[1] += rotor_rotation_speed * delta_time;
                helicopter_tail_rotor_nodes[i].rotation[0] += rotor_rotation_speed * delta_time;
            }

            
            
            
            
            
            for i in 1..aniamtion_nodes.len() {
                
                let animation: Heading = toolbox::simple_heading_animation(elapsed + (i as f32)*helicopter_animation_offset as f32);
                aniamtion_nodes[i].position = glm::vec3(animation.x,  aniamtion_nodes[i].position.y, animation.z);
                aniamtion_nodes[i].rotation[2] = animation.roll;
                aniamtion_nodes[i].rotation[1] = animation.yaw;
                aniamtion_nodes[i].rotation[0] = animation.pitch;

                
            
            }

            

            

            

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


            let speed: f32 = 30.0 * delta_time;  
            let rotation_speed: f32 = 1.0 * delta_time;  
            

            // Handle keyboard input
            if let Ok(keys) = pressed_keys.lock() {
                
                let rotation =  camera_node.rotation.clone();
                

                for key in keys.iter() {
                    // if key is pressed, move the camera



                    match key {
                        


                        VirtualKeyCode::W => camera_node.position += speed * rotation,  // Move forward
                        VirtualKeyCode::S => camera_node.position -= speed * rotation,  // Move backward
                        VirtualKeyCode::A => camera_node.position -= glm::normalize(&glm::cross(&rotation, &camera_up)) * speed,   // Move left
                        VirtualKeyCode::D => camera_node.position += glm::normalize(&glm::cross(&rotation, &camera_up)) * speed,  // Move right
                        VirtualKeyCode::Space => camera_node.position[1] += speed,  // Move up
                        VirtualKeyCode::LShift => camera_node.position[1] -= speed,  // Move down
                        
                        // Rotation
                        VirtualKeyCode::Up => pitch += rotation_speed*0.1,  // Rotate up (around X-axis)
                        VirtualKeyCode::Down => pitch -= rotation_speed*0.1,  // Rotate down (around X-axis)
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

                if mouse_enabled{

                    
                    yaw += delta.0 as f32 * mouse_sensitivity;
                    pitch -= delta.1 as f32 * mouse_sensitivity;

                    // Clamp the pitch value to prevent camera flip
                    let pitch_limit = 89.0_f32.to_radians();
                    if pitch > pitch_limit {
                        pitch = pitch_limit;
                    }
                    if pitch < -pitch_limit {
                        pitch = -pitch_limit;
                    }

                    
                    *delta = (0.0, 0.0); // reset when done

                }
            }

                        // default handler:
                        
            // == // Please compute camera transforms here (exercise 2 & 3)
            let front = glm::vec3(
                yaw.cos() * pitch.cos(),
                pitch.sin(),
                yaw.sin() * pitch.cos()
            );
            camera_node.rotation = glm::normalize(&front);

            

            // Compute the view matrix
            let view = glm::look_at(
                &camera_node.position,
                &(camera_node.position + camera_node.rotation),
                &camera_up
            );

            // Compute the projection matrix
            let aspect = window_aspect_ratio;
            let fovy = 45.0; // FOV (field of view) in degrees
            let near = 1.0; // near plane
            let far = 5000.0; // far plane

            let perspective_projection: glm::Mat4 = glm::perspective(    
                aspect, 
                fovy, 
                near, 
                far
            );
            
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
                let uniform_cam_pos_location = gl::GetUniformLocation(shader_program.program_id, "camera_position\0".as_ptr() as *const i8);
                gl::Uniform3fv(uniform_cam_pos_location, 1, camera_node.position.as_ptr());
                
                
          

                // == // Issue the necessary gl:: commands to draw your scene here
                shader_program.activate();
                // Pass the 'elapsed_time' to the 'time' uniform in the shader
                
                draw_scene(&root_node, &mixed_matrix, &identity_matrix, shader_program.program_id);

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
