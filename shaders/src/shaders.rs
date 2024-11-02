use nalgebra_glm::{Vec3, Vec4, Mat3, mat4_to_mat3, dot, normalize};
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::fragment::Fragment;
use crate::color::Color;


pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
  // Transform position
  let position = Vec4::new(
    vertex.position.x,
    vertex.position.y,
    vertex.position.z,
    1.0
  );
  let transformed = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

  // Perform perspective division
  let w = transformed.w;
  let ndc_position = Vec4::new(
    transformed.x / w,
    transformed.y / w,
    transformed.z / w,
    1.0
  );

  // apply viewport matrix
  let screen_position = uniforms.viewport_matrix * ndc_position;

  // Transform normal
  let model_mat3 = mat4_to_mat3(&uniforms.model_matrix); 
  let normal_matrix = model_mat3.transpose().try_inverse().unwrap_or(Mat3::identity());

  let transformed_normal = normal_matrix * vertex.normal;

  // Create a new Vertex with transformed attributes
  Vertex {
    position: vertex.position,
    normal: vertex.normal,
    tex_coords: vertex.tex_coords,
    color: vertex.color,
    transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
    transformed_normal,
  }
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms, time: u32) -> Color {
  match uniforms.current_shader {
      1 => earth_shader(fragment, uniforms, time),
      2 => cloud_shader(fragment, uniforms),
      // Continúa con otros shaders según sea necesario
      _ => Color::new(0, 0, 0), // Color por defecto si no hay un shader definido
  }
}

fn earth_shader(fragment: &Fragment, uniforms: &Uniforms, time: u32) -> Color {
  let zoom = 100.0;  // to move our values 
  let ox = 100.0; // offset x in the noise map
  let oy = 100.0;
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;
  let t = time as f32 * 0.1;

  let base_noise_value = uniforms.noise.get_noise_2d(x, y);
  let cloud_noise_value = uniforms.cloud_noise.get_noise_2d(
      x * zoom + ox +t, y * zoom + oy
  );

  // Colores base para el agua y la tierra
  let water_color_1 = Color::from_float(0.0, 0.1, 0.6); // Azul oscuro
  let water_color_2 = Color::from_float(0.0, 0.3, 0.7); // Azul claro
  let land_color_1 = Color::from_float(0.1, 0.5, 0.0); // Verde oscuro
  let land_color_2 = Color::from_float(0.2, 0.8, 0.2); // Verde claro
  let cloud_color = Color::from_float(0.9, 0.9, 0.9); // Color casi blanco para las nubes

  let land_threshold = 0.3; // Umbral para tierra

  // Decidir si el fragmento es agua o tierra
  let base_color = if base_noise_value > land_threshold {
      land_color_1.lerp(&land_color_2, (base_noise_value - land_threshold) / (1.0 - land_threshold))
  } else {
      water_color_1.lerp(&water_color_2, base_noise_value / land_threshold)
  };

  // Iluminación más dramática
  let light_position = Vec3::new(1.0, 1.0, 3.0); // Posición de la luz ajustada para mayor contraste
  let light_dir = normalize(&(light_position - fragment.vertex_position)); // Dirección de la luz ajustada
  let normal = normalize(&fragment.normal); // Normalizar la normal
  let diffuse = dot(&normal, &light_dir).max(0.0); // Cálculo de la componente difusa

  let lit_color = base_color * (0.1 + 0.9 * diffuse); 

  let cloud_threshold = 0.1; // Umbral para la aparición de nubes
  let cloud_opacity = 0.3 + 0.2 * ((time as f32 / 1000.0) * 0.3).sin().abs(); 
  if cloud_noise_value > cloud_threshold {
      let cloud_intensity = ((cloud_noise_value - cloud_threshold) / (1.0 - cloud_threshold)).clamp(0.0, 1.0);
      lit_color.blend_add(&(cloud_color * (cloud_intensity * cloud_opacity)))
  } else {
      lit_color
  }
}

fn cloud_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let zoom = 100.0;  // to move our values 
  let ox = 100.0; // offset x in the noise map
  let oy = 100.0;
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;
  let t = uniforms.time as f32 * 0.5;

  let noise_value = uniforms.noise.get_noise_2d(x * zoom + ox + t, y * zoom + oy);

  // Define cloud threshold and colors
  let cloud_threshold = 0.5; // Adjust this value to change cloud density
  let cloud_color = Color::new(255, 255, 255); // White for clouds
  let sky_color = Color::new(30, 97, 145); // Sky blue

  // Determine if the pixel is part of a cloud or sky
  let noise_color = if noise_value > cloud_threshold {
    cloud_color
  } else {
    sky_color
  };

  noise_color * fragment.intensity
}
