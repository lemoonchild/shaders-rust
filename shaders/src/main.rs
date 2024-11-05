use nalgebra_glm::{Vec3, Mat4, look_at, perspective};
use minifb::{Key, Window, WindowOptions};
use std::{f32::consts::PI, time::Instant};

mod framebuffer;
mod triangle;
mod vertex;
mod obj;
mod color;
mod fragment;
mod shaders;
mod camera;

use framebuffer::Framebuffer;
use vertex::Vertex;
use obj::Obj;
use camera::Camera;
use triangle::triangle;
use shaders::{vertex_shader, fragment_shader};
use fastnoise_lite::{FastNoiseLite, NoiseType, FractalType};


pub struct Uniforms {
    model_matrix: Mat4,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    viewport_matrix: Mat4,
    time: u32,
    noise: FastNoiseLite,
    cloud_noise: FastNoiseLite, 
    band_noise: FastNoiseLite, 
    current_shader: u8, 
}

fn create_noise(current_shader: u8) -> FastNoiseLite {
    match current_shader {
        1 => create_earth_noise(),
        2 => create_mars_noise(),
        3 => create_mercury_noise(),
        4 => FastNoiseLite::new(),
        5 => create_jupiter_noise(),
        6 => create_urano_noise(),
        8 => create_moon_noise(),
        9 => FastNoiseLite::new(),
        _ => create_earth_noise(),  
    }
}

fn create_earth_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(1337);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2S));
    noise.set_fractal_type(Some(FractalType::Ridged));
    noise.set_fractal_octaves(Some(5)); // Octavas para mayor detalle
    noise.set_fractal_lacunarity(Some(3.0)); // Lacunaridad para escalado de frecuencia
    noise.set_fractal_gain(Some(0.5)); // Ganancia para el escalado de amplitud
    noise.set_frequency(Some(0.5)); 

    noise
}

fn create_cloud_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(40);  
    noise.set_noise_type(Some(NoiseType::Perlin)); 
    noise.set_fractal_type(Some(FractalType::FBm));
    noise.set_fractal_octaves(Some(2));  // Menos octavas para menos detalles
    noise.set_fractal_lacunarity(Some(3.0));
    noise.set_fractal_gain(Some(0.5));
    noise.set_frequency(Some(0.01));  // Baja frecuencia para nubes grandes y suaves
    noise
}

fn create_mars_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(1234);
    noise.set_noise_type(Some(NoiseType::Perlin));
    noise.set_fractal_type(Some(FractalType::Ridged));
    noise.set_fractal_octaves(Some(4));
    noise.set_fractal_lacunarity(Some(2.0));
    noise.set_fractal_gain(Some(0.5));
    noise.set_frequency(Some(1.5)); 
    noise
}

fn create_moon_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(4321);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    noise.set_fractal_type(Some(FractalType::PingPong));
    noise.set_fractal_octaves(Some(2));
    noise.set_fractal_lacunarity(Some(2.0));
    noise.set_fractal_gain(Some(0.5));
    noise.set_frequency(Some(3.0));  
    noise
}

fn create_mercury_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(4321);
    noise.set_noise_type(Some(NoiseType::Perlin));
    noise.set_fractal_type(Some(FractalType::PingPong));
    noise.set_fractal_octaves(Some(5));
    noise.set_fractal_lacunarity(Some(2.0));
    noise.set_fractal_gain(Some(1.0));
    noise.set_frequency(Some(5.0));  
    noise
}

fn create_jupiter_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(5678); // Puedes elegir cualquier semilla
    noise.set_noise_type(Some(NoiseType::OpenSimplex2)); // OpenSimplex2 produce un ruido más suave
    noise.set_fractal_type(Some(FractalType::DomainWarpProgressive)); // Añade complejidad fractal
    noise.set_fractal_octaves(Some(6)); // Más octavas para más detalle
    noise.set_fractal_lacunarity(Some(2.0)); // Lacunaridad estándar
    noise.set_fractal_gain(Some(0.5)); // Ganancia menor para detalles finos
    noise.set_frequency(Some(2.0)); // Ajusta la escala del ruido
    noise
}

fn create_jupiter_band_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(7890); // Nueva semilla
    noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    noise.set_frequency(Some(1.0));
    noise.set_fractal_type(Some(FractalType::FBm));
    noise
}

fn create_urano_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(2021);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    noise.set_fractal_type(Some(FractalType::Ridged));
    noise.set_fractal_octaves(Some(4));
    noise.set_fractal_lacunarity(Some(2.0));
    noise.set_fractal_gain(Some(0.4));
    noise.set_frequency(Some(0.2));
    noise
}

fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    let rotation_matrix_x = Mat4::new(
        1.0,  0.0,    0.0,   0.0,
        0.0,  cos_x, -sin_x, 0.0,
        0.0,  sin_x,  cos_x, 0.0,
        0.0,  0.0,    0.0,   1.0,
    );

    let rotation_matrix_y = Mat4::new(
        cos_y,  0.0,  sin_y, 0.0,
        0.0,    1.0,  0.0,   0.0,
        -sin_y, 0.0,  cos_y, 0.0,
        0.0,    0.0,  0.0,   1.0,
    );

    let rotation_matrix_z = Mat4::new(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z,  cos_z, 0.0, 0.0,
        0.0,    0.0,  1.0, 0.0,
        0.0,    0.0,  0.0, 1.0,
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    let transform_matrix = Mat4::new(
        scale, 0.0,   0.0,   translation.x,
        0.0,   scale, 0.0,   translation.y,
        0.0,   0.0,   scale, translation.z,
        0.0,   0.0,   0.0,   1.0,
    );

    transform_matrix * rotation_matrix
}


fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    look_at(&eye, &center, &up)
}

fn create_perspective_matrix(window_width: f32, window_height: f32) -> Mat4 {
    let fov = 45.0 * PI / 180.0;
    let aspect_ratio = window_width / window_height;
    let near = 0.1;
    let far = 1000.0;

    perspective(fov, aspect_ratio, near, far)
}

fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0, 0.0, 0.0, width / 2.0,
        0.0, -height / 2.0, 0.0, height / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex], time: u32) {
    // Vertex Shader Stage
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    // Primitive Assembly Stage
    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    // Rasterization Stage
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    // Fragment Processing Stage
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;
        if x < framebuffer.width && y < framebuffer.height {
            // Apply fragment shader
            let shaded_color = fragment_shader(&fragment, &uniforms, time);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}

fn main() {
    let window_width = 680;
    let window_height = 800;
    let framebuffer_width = 680;
    let framebuffer_height = 800;

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Rust Graphics - Planet Shader - Press 1-7 to switch",
        window_width,
        window_height,
        WindowOptions::default(),
    )
        .unwrap();

    window.set_position(500, 500);
    window.update();

    framebuffer.set_background_color(0x333355);

    // model position
    let translation = Vec3::new(0.0, 0.0, 0.0);
    let rotation = Vec3::new(0.0, 0.0, 0.0);
    let scale = 1.0f32;

    // camera parameters
    let mut camera = Camera::new(
        Vec3::new(0.0, 5.0, 10.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 10.0, 0.0)
    );

    let obj = Obj::load("assets/models/sphere.obj").expect("Failed to load obj");
    let moon = Obj::load("assets/models/moon.obj").expect("Failed to load obj");
    let ring_obj = Obj::load("assets/models/ring.obj").expect("Failed to load ring model");

    let vertex_arrays = obj.get_vertex_array(); 
    let moon_vertex_array = moon.get_vertex_array();
    let ring_vertex_array = ring_obj.get_vertex_array();

    let mut last_frame_time = Instant::now();
    let mut time = 0;

    // Lunas de los planetas rocosos
    let moon_scale = 0.5; // Escala de la luna respecto al planeta
    let moon_distance = 2.5; // Distancia de la luna al planeta
    let moon_orbit_speed = 0.001; // Velocidad orbital de la luna

    let projection_matrix = create_perspective_matrix(window_width as f32, window_height as f32);
    let viewport_matrix = create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);
    let mut uniforms = Uniforms { 
        model_matrix: Mat4::identity(), 
        view_matrix: Mat4::identity(), 
        projection_matrix, 
        viewport_matrix, 
        time: 0, 
        noise: create_noise(1),
        cloud_noise: create_cloud_noise(),
        band_noise: create_jupiter_band_noise(), 
        current_shader: 1,
    };

    let mut current_planet = 1; 

    while window.is_open() && !window.is_key_down(Key::Escape) {

        let delta_time = last_frame_time.elapsed();
        last_frame_time = Instant::now();
        time += delta_time.as_millis() as u32;

        let keys = window.get_keys_pressed(minifb::KeyRepeat::No);
        for key in keys {
            match key {
                Key::Key1 => {
                    current_planet = 1;
                }
                Key::Key2 => {
                    current_planet = 2;
                }
                Key::Key3 => {
                    current_planet = 3;
                }
                Key::Key4 => {
                    current_planet = 4;
                }
                Key::Key5 => {
                    current_planet = 5;
                }
                Key::Key6 => {
                    current_planet = 6;
                }
                Key::Key7 => {
                    current_planet = 7;
                }
                _ => {}
            }
        }

        handle_input(&window, &mut camera);
        framebuffer.clear();
        
        uniforms.current_shader = current_planet;
        uniforms.noise = create_noise(uniforms.current_shader);

        uniforms.view_matrix = create_view_matrix(camera.eye, camera.center, camera.up);
        uniforms.time = time as u32;

        if current_planet == 2 {
            // Renderizar Marte
            uniforms.current_shader = 2;
            uniforms.model_matrix = create_model_matrix(translation, scale, rotation);
            render(&mut framebuffer, &uniforms, &vertex_arrays, time as u32);
        
            // Calcular y renderizar la luna de Marte
            let moon_angle = time as f32 * moon_orbit_speed;
            let moon_x = moon_distance * moon_angle.cos();
            let moon_z = moon_distance * moon_angle.sin();
        
            let moon_translation = Vec3::new(moon_x, 0.0, moon_z);
            let moon_model_matrix = create_model_matrix(moon_translation, moon_scale, Vec3::new(0.0, 0.0, 0.0));
            uniforms.model_matrix = moon_model_matrix;
        
            let moon_shader_id = 8;
            uniforms.current_shader = moon_shader_id;
            render(&mut framebuffer, &uniforms, &moon_vertex_array, time as u32);
        
        } else if current_planet == 4 {
            // Renderizar Saturno
            uniforms.current_shader = 4;  // Shader para Saturno
            uniforms.model_matrix = create_model_matrix(translation, scale, rotation);
            render(&mut framebuffer, &uniforms, &vertex_arrays, time as u32);
        
            // Renderizar los anillos de Saturno
            uniforms.current_shader = 9;  // Shader para los anillos
            let ring_translation = translation;  // Posición de los anillos
            let ring_scale = scale * 1.5;  // Tamaño de los anillos (más grande que el planeta)
            uniforms.model_matrix = create_model_matrix(ring_translation, ring_scale, Vec3::new(0.0, 0.0, 0.0));
            render(&mut framebuffer, &uniforms, &ring_vertex_array, time as u32);  // Reutiliza `ring_vertex_array` para la geometría de los anillos
        
        } else {
            // Renderizar otros planetas sin lunas
            uniforms.model_matrix = create_model_matrix(translation, scale, rotation);
            render(&mut framebuffer, &uniforms, &vertex_arrays, time as u32);
        }
        
        uniforms.model_matrix = create_model_matrix(translation, scale, rotation);
        framebuffer.set_current_color(0xFFDDDD);

        window
        .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
        .unwrap();
    }
}

fn handle_input(window: &Window, camera: &mut Camera) {
    let movement_speed = 1.0;
    let rotation_speed = PI/50.0;
    let zoom_speed = 0.1;

    //  camera orbit controls
    if window.is_key_down(Key::Left) {
        camera.orbit(rotation_speed, 0.0);
    }
    if window.is_key_down(Key::Right) {
        camera.orbit(-rotation_speed, 0.0);
    }
    if window.is_key_down(Key::W) {
        camera.orbit(0.0, -rotation_speed);
    }
    if window.is_key_down(Key::S) {
        camera.orbit(0.0, rotation_speed);
    }

    // Camera movement controls
    let mut movement = Vec3::new(0.0, 0.0, 0.0);
    if window.is_key_down(Key::A) {
        movement.x -= movement_speed;
    }
    if window.is_key_down(Key::D) {
        movement.x += movement_speed;
    }
    if window.is_key_down(Key::Q) {
        movement.y += movement_speed;
    }
    if window.is_key_down(Key::E) {
        movement.y -= movement_speed;
    }
    if movement.magnitude() > 0.0 {
        camera.move_center(movement);
    }

    // Camera zoom controls
    if window.is_key_down(Key::Up) {
        camera.zoom(zoom_speed);
    }
    if window.is_key_down(Key::Down) {
        camera.zoom(-zoom_speed);
    }
}