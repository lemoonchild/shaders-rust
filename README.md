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
https://github.com/user-attachments/assets/52096288-fbf0-41cf-9051-830a224695cc

### Mars
https://github.com/user-attachments/assets/c7cd7352-0a35-4326-ab73-2ab833656c67

### Mercury 
https://github.com/user-attachments/assets/3ebd3214-7643-4e06-b3e9-71ca7a336e79

### Saturn
https://github.com/user-attachments/assets/7105e67e-1089-44ea-8338-f166d05884c1

### Jupiter 
https://github.com/user-attachments/assets/36037b2d-74c2-4d88-bc1a-d5e8f9fe0a69

### Urano
https://github.com/user-attachments/assets/b2d9c89b-f4b2-42fd-8c9f-4c9b5a74643d

### Sun
https://github.com/user-attachments/assets/285bb9e3-295e-4175-95c1-0e4a47f95bce

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
