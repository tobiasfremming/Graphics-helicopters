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
use std::thread;
use std::sync::{Mutex, Arc, RwLock};
use std::time::Instant;


mod shader;
mod util;

use glm::{normalize_dot, Mat4, Vec3};
use glutin::event::ElementState;
use glutin::event::{Event, WindowEvent, DeviceEvent, KeyboardInput, ElementState::{Pressed, Released}, VirtualKeyCode::{self, *}};
use glutin::event_loop::ControlFlow;



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
unsafe fn create_vao(vertices: &Vec<f32>, indices: &Vec<u32>, colors: &Vec<f32>) -> u32 {
    

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
    

    // Enable the VAP
    
    gl::EnableVertexAttribArray(1);


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

        /*
        let simple_shader = unsafe {
            shader::ShaderBuilder::new()
                .attach_file("./path/to/simple/shader.file")
                .link()
        };
        */

        // let vertices: Vec<f32> = vec![
        //     // Triangle 1 
        //     -0.8,  0.5, 0.0,   // Top left
        //     -0.5,  0.5, 0.0,   // Top right
        //     -0.8,  1.0, 0.0,   // Bottom left
        //     // -0.2, -0.4, -0.2,   // Bottom left 
        //     // -0.3, -1.0, 0.0,    // Bottom right 
        //     // 0.1, -0.5, 0.0,    // Top left 

        //     // Triangle 2
        //     -0.1, -0.4, -0.1,   // Bottom left 
        //     -0.2, -1.0, -0.1,    // Bottom right 
        //     0.1, -0.5, -0.1,    // Top left 

        //     // Triangle 3 
        //     -0.8, -0.8, -1.0,   // Bottom left
        //     -0.5, -1.0, -1.0,   // Bottom right
        //     -0.2, -0.5, -1.0,   // Top left

        //     // Triangle 4 
        //     0.7, -0.7, 0.0,    // Bottom far right
        //     0.7, -0.3, 0.0,    // Bottom right
        //     0.4, -0.5, 0.0,    // Far bottom right

        //     // Triangle 5 
        //     0.0, 0.1, 0.0,     // Center
        //     0.4, 0.1, 0.0,     // Right of center
        //     0.2, 0.5, 0.0,     // Above center
        // ];
        let  vertices: Vec<f32> = create_box((-0.5, -0.5, -0.5), 1.0, 1.0, 1.0);
        // let mut vertices: Vec<f32> = vec![];

        //let billboard_vertices: Vec<f32> = create_square(0.1, 0.0, 0.0, 0.0);
        let mut billboard_vertices: Vec<f32> = vec![];
        let point0 = [-1.0, -1.0, -2.0];
        let point1 = [1.0, -1.0, -2.0];
        let point2 = [-1.0, 1.0, -2.0];
        let point3 = [1.0, 1.0, -2.0];

        billboard_vertices.extend_from_slice(&point0);
        billboard_vertices.extend_from_slice(&point1);
        billboard_vertices.extend_from_slice(&point2);

        billboard_vertices.extend_from_slice(&point1);
        billboard_vertices.extend_from_slice(&point3);
        billboard_vertices.extend_from_slice(&point2);


        let indices: Vec<u32> = vec![
            0, 1, 2,  // Triangle 1
            3, 4, 5,  // Triangle 2
            6, 7, 8,  // Triangle 3
            9, 10, 11, // Triangle 4
            12, 13, 14, // Triangle 5
            15, 16, 17, // Triangle 6
            18, 19, 20, // Triangle 7
            21, 22, 23, // Triangle 8
            24, 25, 26, // Triangle 9
            27, 28, 29, // Triangle 10
            30, 31, 32, // Triangle 11
            33, 34, 35, // Triangle 12
            36, 37, 38, // Triangle 13
            39, 40, 41, // Triangle 14
            42, 43, 44, // Triangle 15
            45, 46, 47, // Triangle 16
            48, 49, 50, // Triangle 17
            51, 52, 53, // Triangle 18
            54, 55, 56, // Triangle 19
            57, 58, 59, // Triangle 20
            60, 61, 62, // Triangle 21
            63, 64, 65, // Triangle 22
            66, 67, 68, // Triangle 23
            69, 70, 71, // Triangle 24

        ];

        let colors: Vec<f32> = vec![
            


            // 0.0, 1.0, 0.0, 0.5, // Green
            // 0.2, 0.8, 0.0, 0.5, // Yellowish Green
            // 0.0, 0.6, 0.2, 0.5, // Blueish Green

            // 1.0, 0.0, 0.0, 0.5, // Red
            // 1.0, 0.3, 0.0, 0.5, // Orange
            // 0.8, 0.0, 0.2, 0.5, // Reddish Purple
            
            // 0.0, 0.0, 1.0, 0.5, // Blue
            // 0.2, 0.2, 1.0, 0.5, // Light Blue
            // 0.0, 0.2, 0.8, 0.5, // Blueish Cyan

            // 1.0, 1.0, 0.0, 1.0, // Yellow
            // 1.0, 0.8, 0.2, 1.0, // Yellowish Orange
            // 0.8, 1.0, 0.2, 1.0, // Greenish Yellow

            // 0.0, 1.0, 1.0, 1.0, // Cyan
            // 0.2, 0.8, 1.0, 1.0, // Light Blue Cyan
            // 0.0, 1.0, 0.8, 1.0, // Greenish Cyan

            1.0, 1.0, 0.0, 1.0, // Yellow 0
            0.0, 1.0, 1.0, 1.0, // Cyan   1
            1.0, 0.0, 0.0, 1.0, // Green  2
            0.0, 1.0, 1.0, 1.0, // Cyan   1
            0.0, 0.0, 1.0, 1.0, // Blue   3
            1.0, 0.0, 0.0, 1.0, // Green  2

            1.0, 1.0, 0.0, 1.0, // Yellow 0
            0.0, 1.0, 1.0, 1.0, // Cyan   1
            1.0, 0.0, 0.0, 1.0, // Green  2
            0.0, 1.0, 1.0, 1.0, // Cyan   1
            0.0, 0.0, 1.0, 1.0, // Blue   3
            1.0, 0.0, 0.0, 1.0, // Green  2

            1.0, 1.0, 0.0, 1.0, // Yellow 0
            0.0, 1.0, 1.0, 1.0, // Cyan   1
            1.0, 0.0, 0.0, 1.0, // Green  2
            0.0, 1.0, 1.0, 1.0, // Cyan   1
            0.0, 0.0, 1.0, 1.0, // Blue   3
            1.0, 0.0, 0.0, 1.0, // Green  2

            1.0, 1.0, 0.0, 1.0, // Yellow 0
            0.0, 1.0, 1.0, 1.0, // Cyan   1
            1.0, 0.0, 0.0, 1.0, // Green  2
            0.0, 1.0, 1.0, 1.0, // Cyan   1
            0.0, 0.0, 1.0, 1.0, // Blue   3
            1.0, 0.0, 0.0, 1.0, // Green  2

            1.0, 1.0, 0.0, 1.0, // Yellow 0
            0.0, 1.0, 1.0, 1.0, // Cyan   1
            1.0, 0.0, 0.0, 1.0, // Green  2
            0.0, 1.0, 1.0, 1.0, // Cyan   1
            0.0, 0.0, 1.0, 1.0, // Blue   3
            1.0, 0.0, 0.0, 1.0, // Green  2

            1.0, 1.0, 0.0, 1.0, // Yellow 0
            0.0, 1.0, 1.0, 1.0, // Cyan   1
            1.0, 0.0, 0.0, 1.0, // Green  2
            0.0, 1.0, 1.0, 1.0, // Cyan   1
            0.0, 0.0, 1.0, 1.0, // Blue   3
            1.0, 0.0, 0.0, 1.0, // Green  2

            1.0, 1.0, 0.0, 1.0, // Yellow 0
            0.0, 1.0, 1.0, 1.0, // Cyan   1
            1.0, 0.0, 0.0, 1.0, // Green  2
            0.0, 1.0, 1.0, 1.0, // Cyan   1
            0.0, 0.0, 1.0, 1.0, // Blue   3
            1.0, 0.0, 0.0, 1.0, // Green  2

            1.0, 1.0, 0.0, 1.0, // Yellow 0
            0.0, 1.0, 1.0, 1.0, // Cyan   1
            1.0, 0.0, 0.0, 1.0, // Green  2
            0.0, 1.0, 1.0, 1.0, // Cyan   1
            0.0, 0.0, 1.0, 1.0, // Blue   3
            1.0, 0.0, 0.0, 1.0, // Green  2

            


            


            
            0.0, 1.0, 0.0, 1.0, // Green
            0.0, 0.0, 1.0, 1.0, // Blue
            1.0, 1.0, 0.0, 1.0, // Yellow
            0.0, 1.0, 1.0, 1.0, // Cyan
            1.0, 1.0, 0.0, 1.0, // Yellow
            0.0, 1.0, 1.0, 1.0, // Cyan
            1.0, 0.0, 0.0, 1.0, // Red
            0.0, 1.0, 0.0, 1.0, // Green
            0.0, 0.0, 1.0, 1.0, // Blue
            1.0, 1.0, 0.0, 1.0, // Yellow
            0.0, 1.0, 1.0, 1.0, // Cyan
            1.0, 1.0, 0.0, 1.0, // Yellow
            0.0, 1.0, 1.0, 1.0, // Cyan
            1.0, 0.0, 0.0, 1.0, // Red
            0.0, 1.0, 0.0, 1.0, // Green
            0.0, 0.0, 1.0, 1.0, // Blue
            1.0, 1.0, 0.0, 1.0, // Yellow
            0.0, 1.0, 1.0, 1.0, // Cyan
            1.0, 0.0, 0.0, 1.0, // Red
            0.0, 1.0, 0.0, 1.0, // Green
            0.0, 0.0, 1.0, 1.0, // Blue
            1.0, 1.0, 0.0, 1.0, // Yellow
            0.0, 1.0, 1.0, 1.0, // Cyan
            1.0, 0.0, 0.0, 1.0, // Red
            0.0, 1.0, 0.0, 1.0, // Green
            0.0, 0.0, 1.0, 1.0, // Blue
            1.0, 1.0, 0.0, 1.0, // Yellow
            0.0, 1.0, 1.0, 1.0, // Cyan


            
        ];
        // Create the VAO
        let vao = unsafe {
            create_vao(&vertices, &indices, &colors)
        };

        // Draw the VAO
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawElements(
                gl::TRIANGLES,
                indices.len() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null()
            );
        }
        
        // set up camera
        //let mut camera_motion = CameraMotion::new();
        
        let mut camera_transformation_matrix: Mat4 = glm::identity();
        let mut pitch: f32 = 0.0; // rotation around x-axis
        let mut yaw:f32 = 0.0; // rotation around y-axis
        let mut roll:f32  = 0.0; // rotation around z-axis

        let mut camera_position = glm::vec3(0.0, 0.0, -5.0);
        let mut camera_front: Vec<f32> = vec![0.0, 0.0, 0.0];

        


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

            let forward = Vec3::new(
                camera_front[0].to_radians().cos() * pitch.to_radians().cos(),
                camera_front[1].to_radians().sin(),
                camera_front[2].to_radians().sin() * pitch.to_radians().cos()
            ).normalize();

            let speed: f32 = 5.0 * delta_time;  
            let rotation_speed: f32 = 1.0 * delta_time;  

            
            

            // Handle keyboard input
            if let Ok(keys) = pressed_keys.lock() {
                for key in keys.iter() {
                    // if key is pressed, move the camera



                    match key {
                        VirtualKeyCode::W => camera_position.z += speed,  // Move forward
                        VirtualKeyCode::S => camera_position.z -= speed,  // Move backward
                        VirtualKeyCode::A => camera_position.x += speed,  // Move left
                        VirtualKeyCode::D => camera_position.x -= speed,  // Move right
                        VirtualKeyCode::Space => camera_position.y -= speed,  // Move up
                        VirtualKeyCode::LShift => camera_position.y += speed,  // Move down
                        
                        // Rotation
                        VirtualKeyCode::Up => camera_front[0] -= rotation_speed,  // Rotate up (around X-axis)
                        VirtualKeyCode::Down => camera_front[0] += rotation_speed,  // Rotate down (around X-axis)
                        VirtualKeyCode::Left => camera_front[1] += rotation_speed,  // Rotate left (around Y-axis)
                        VirtualKeyCode::Right => camera_front[1] -= rotation_speed,  // Rotate right (around Y-axis)
                        // The `VirtualKeyCode` enum is defined here:
                        //    https://docs.rs/winit/0.25.0/winit/event/enum.VirtualKeyCode.html

                        // VirtualKeyCode::A => {
                        //     _arbitrary_number += delta_time;
                        // }
                        // VirtualKeyCode::D => {
                        //     _arbitrary_number -= delta_time;
                        // }

_ => { }
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


            // Compute the projection matrix
            let aspect = window_aspect_ratio;
            let fovy = 45.0; // FOV (field of view) in degrees
            let near = 1.0; // near plane
            let far = 100.0; // far plane

            let perspective_projection: glm::Mat4 = glm::perspective(    
                aspect, 
                fovy, 
                near, 
                far
            );

            

            // Compute the view matrix
            let translation = Mat4::new_translation(&camera_position);
            let rotation_x = glm::rotation(camera_front[0], &glm::vec3(1.0, 0.0, 0.0));
            let rotation_y = glm::rotation(camera_front[1], &glm::vec3(0.0, 1.0, 0.0));
            camera_transformation_matrix = translation * rotation_x * rotation_y;
            
            let mut identity_matrix: glm::Mat4 = glm::identity();  
            let mut mixed_matrix = perspective_projection * camera_transformation_matrix * identity_matrix;

            unsafe {
                // Clear the color and depth buffers
                gl::ClearColor(0.035, 0.046, 0.078, 1.0); // night sky
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);


                //transformation_matrix = glm::translate(&transformation_matrix, &glm::vec3(f32::sin(elapsed*0.5), f32::sin(elapsed*0.5), 0.0));
                // Pass the 'transformation_matrix' to the 'transformation_matrix' uniform in the shader
                let transformation_matrix_location: i32 = gl::GetUniformLocation(shader_program.program_id, "transformation_matrix\0".as_ptr() as *const i8);
                gl::UniformMatrix4fv(transformation_matrix_location, 1, gl::FALSE, mixed_matrix.as_ptr());

                // Pass the 'elapsed_time' to the 'time' uniform in the shader
                let time_uniform_location = gl::GetUniformLocation(shader_program.program_id, "time".as_ptr() as *const i8);
                gl::Uniform1f(time_uniform_location, elapsed);
                // == // Issue the necessary gl:: commands to draw your scene here
                shader_program.activate();
                // Pass the 'elapsed_time' to the 'time' uniform in the shader
                

                // Bind the VAO
                gl::BindVertexArray(vao);

                // Draw the elements
                gl::DrawElements(
                    gl::TRIANGLES,
                    indices.len() as i32,
                    gl::UNSIGNED_INT,
                    std::ptr::null()
                );

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
