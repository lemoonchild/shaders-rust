use nalgebra_glm::{dot, mat4_to_mat3, normalize, Mat3, Vec2, Vec3, Vec4};
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
      2 => mars_planet_shader(fragment, uniforms),
      3 => mercury_shader(fragment, uniforms),
      8 => moon_shader(fragment, uniforms),
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

fn mars_planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let noise_value = uniforms.noise.get_noise_2d(fragment.vertex_position.x, fragment.vertex_position.y);
  
  let dark_red = Color::from_float(0.4, 0.1, 0.1); // Color oscuro para áreas en sombra
  let bright_orange = Color::from_float(0.8, 0.4, 0.1); // Color brillante para áreas iluminadas
  let terracotta = Color::from_float(0.6, 0.3, 0.1); // Color intermedio, típico de Marte

  // Usar lerp para mezclar colores basado en el valor del ruido
  let lerp_factor = noise_value.clamp(0.0, 1.0); // Asegurar que esté entre 0 y 1
  let base_color = if lerp_factor < 0.5 {
    dark_red.lerp(&terracotta, lerp_factor * 2.0) // Interpola entre rojo oscuro y terracotta
  } else {
    terracotta.lerp(&bright_orange, (lerp_factor - 0.5) * 2.0) // Interpola entre terracotta y naranja brillante
  };

  // Definir la posición y dirección de la luz
  let light_pos = Vec3::new(0.0, 8.0, 9.0);  // Posición de la fuente de luz
  let light_dir = (light_pos - fragment.vertex_position).normalize(); // Dirección de la luz desde la posición del fragmento

  // Normalizar la normal del fragmento
  let normal = fragment.normal.normalize();

  // Calcular la intensidad de la luz difusa
  let diffuse_intensity = normal.dot(&light_dir).max(0.0);

  // Modificar el color final basado en la intensidad de la luz
  let lit_color = base_color * diffuse_intensity;  // Modula el color por la intensidad de la luz

  // Añadir un término ambiental para evitar que las partes no iluminadas sean completamente oscuras
  let ambient_intensity = 0.15;  // Intensidad de luz ambiental, ajusta según necesites
  let ambient_color = base_color * ambient_intensity;

  // Suma del componente ambiental y difuso
  let combined_color = ambient_color + lit_color;

  combined_color
}

pub fn moon_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // Base y detalles de color más distintos
  let base_color = Color::from_float(0.8, 0.8, 0.8); // Gris base
  let detail_color = Color::from_float(0.3, 0.3, 0.3); // Gris más oscuro para detalles

  // Genera variaciones en la superficie
  let noise_value = uniforms.noise.get_noise_2d(fragment.vertex_position.x, fragment.vertex_position.y);

  // Normaliza el valor del ruido a [0, 1]
  let normalized_noise = (noise_value + 1.0) * 0.5; // Ajusta según el rango real de tu generador de ruido
  let surface_variation = base_color.lerp(&detail_color, normalized_noise.clamp(0.0, 1.0));

  // Iluminación simple
  let light_position = Vec3::new(10.0, 10.0, 10.0);
  let light_direction = (light_position - fragment.vertex_position).normalize();
  let normal = fragment.normal.normalize();
  let diffuse = normal.dot(&light_direction).max(0.0);

  // Combinar color de superficie con iluminación
  surface_variation * (0.3 + 0.7 * diffuse) 
}

pub fn mercury_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // Colores base para la superficie de Mercurio
  let gray_light = Color::from_float(0.7, 0.7, 0.7);
  let gray_dark = Color::from_float(0.4, 0.4, 0.4);
  let brown = Color::from_float(0.5, 0.4, 0.3);
  let blue_tint = Color::from_float(0.3, 0.3, 0.7);
  let yellow_light = Color::from_float(0.8, 0.7, 0.4);

  // Genera ruido para variaciones de color
  let noise_value1 = uniforms.noise.get_noise_2d(fragment.vertex_position.x, fragment.vertex_position.y);
  let noise_value2 = uniforms.noise.get_noise_2d(fragment.vertex_position.x * 2.0, fragment.vertex_position.y * 2.0); // Ajustar frecuencia
  let noise_value3 = uniforms.noise.get_noise_2d(fragment.vertex_position.x * 0.5, fragment.vertex_position.y * 0.5); // Baja frecuencia

  // Normaliza los valores de ruido
  let lerp_factor1 = (noise_value1 + 1.0) * 0.5; // Normalizar a [0, 1]
  let lerp_factor2 = (noise_value2 + 1.0) * 0.5;
  let lerp_factor3 = (noise_value3 + 1.0) * 0.5;

  // Mezcla de colores usando `lerp`
  let color_mix1 = gray_light.lerp(&gray_dark, lerp_factor1);
  let color_mix2 = color_mix1.lerp(&brown, lerp_factor2 * 2.5);
  let color_mix3 = color_mix2.lerp(&blue_tint, lerp_factor2 * 1.5);
  let final_color = color_mix3.lerp(&yellow_light, lerp_factor3);

  // Iluminación para dar más realismo
  let light_position = Vec3::new(0.0, 8.0, 9.0);
  let light_direction = (light_position - fragment.vertex_position).normalize();
  let normal = fragment.normal.normalize();
  let diffuse = normal.dot(&light_direction).max(0.0);

  // Combinación de la iluminación con el color
  let ambient_intensity = 0.15;
  let ambient_color = final_color * ambient_intensity;
  let lit_color = final_color * diffuse;

  // Suma del componente ambiental y difuso
  ambient_color + lit_color
}






