# Planetary Shader Project

## Project Overview
The Planetary Shader Project is an exploration into procedural rendering and shader development in Rust, focusing on creating visually compelling celestial bodies without using textures or pre-built materials. The primary objective of this project is to practice generating complex visual effects purely through color manipulation, utilizing parameters available in a custom software renderer. By layering and combining mathematical functions and procedural noise, each celestial body is brought to life with unique characteristics, from dynamic cloud formations and rough planetary surfaces to atmospheric gradients and emissive glow.

## Features
- **Dynamic Celestial Body Rendering**: Simulates multiple types of celestial bodies, including a sun, rocky planets, and gas giants.
- **Shader-Based Effects**: Uses complex shaders to render atmospheric effects, surface textures, color gradation, and luminous emissions.
- **Bloom and Gaussian Blur**: Applies bloom and Gaussian blur effects to create realistic glows around emissive objects like the sun.
- **Interactive Controls**: Switch between celestial bodies using keyboard inputs (keys 1-7).
- **Noise-Based Texturing**: Implements noise-based texturing for realistic surface and atmospheric patterns without using external textures.

## Getting Started
### Prerequisites
- **Rust**: Install Rust from [rust-lang.org](https://www.rust-lang.org/) if you haven't already.

### Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/lemoonchild/shaders-rust.git 
2. Build the project
    ```bash
    cd shaders
3. Build the project
    ```bash
    cargo build --release
4. Build the project
    ```bash
    cargo run --release  
## Usage
1. Use the keys 1 to 7 to switch between different celestial bodies.
2. Arrow keys allow for camera orbit controls.
3. W and S: Move the camera up and down.
4. A and D: Move the camera left and right.
5. Q and E: Move the camera forward and backward.
6. Up and Down arrows: Zoom in and out.

## Implemented Celestial Bodies

### The earth
<video controls src="videos\20241105-1928-37.4232512.mp4" title="Earth"></video>

### Mars
<video controls src="videos\20241105-1929-21.5232669.mp4" title="Mars"></video>

### Mercury 
![Mercury](videos\image.png)

### Saturn
<video controls src="videos\20241105-1930-33.0115835.mp4" title="Saturn"></video>

### Jupiter 
![Jupiter](videos\image-1.png)

### Urano
<video controls src="videos\20241105-1932-01.0689104.mp4" title="Urano"></video>

### Sun
<video controls src="videos\20241105-1932-30.0620717.mp4" title="Sun"></video>

## Shader Techniques

### Noise Generation and Application
Noise functions are used to simulate terrain, atmospheric clouds, and color variation. Each planet utilizes different noise configurations to achieve unique effects.

### Bloom and Gaussian Blur
Gaussian blur is applied to the emissive buffer of the sun to create a realistic glow. The bloom effect is achieved by combining blurred pixels with the main image buffer, making the sun appear bright and radiant.

### Lighting and Shading
Each celestial body uses a basic lighting model with diffuse lighting. The lighting position is fixed, casting realistic shadows and highlights based on the surface’s orientation.

### Emissive Materials
The sun's shader has an emissive material component that generates a glowing effect. The emission falls off smoothly with distance, simulating a natural glow.

## Dependencies
This project relies on the following dependencies:

1. `fastnoise-lite (1.1.1):` Used for procedural noise generation to create textures and simulate terrain and atmospheric details.
2. `minifb (0.27.0):` Provides a framebuffer-based windowing library for displaying the rendered images.
3. `nalgebra-glm (0.19.0):` A linear algebra library for handling vector and matrix operations.
4. `rand (0.8.5):` Generates random numbers used for noise and variation.
5. `tobj (4.0.2):` A library for loading OBJ models, allowing for model import in the rendering pipeline.

## License
This project is licensed under the MIT License.