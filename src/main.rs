use nalgebra_glm::Vec3;
use crate::color::Color;
use crate::material::Material;
use crate::scene::Scene;
use crate::sphere::Sphere;
use crate::camera::Camera;
use crate::render::render;
use crate::framebuffer::Framebuffer;
use crate::light::Light;
use pixels::{Pixels, SurfaceTexture};
use winit::event::{Event, WindowEvent, MouseButton, ElementState};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit::dpi::PhysicalPosition;

mod camera;
mod color;
mod framebuffer;
mod material;
mod ray_intersect;
mod render;
mod scene;
mod sphere;
mod intersect;
mod cast_ray;
mod light;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Rust Graphics - Raytracer")
        .with_inner_size(winit::dpi::LogicalSize::new(WIDTH, HEIGHT))
        .build(&event_loop)
        .unwrap();

    let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, &window);
    let mut pixels = Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap();
    let mut framebuffer = Framebuffer::new(WIDTH as usize, HEIGHT as usize);

    let rubber = Material::new(
        Color::new(80.0, 0.0, 0.0),
        1.0,
        [0.9, 0.1, 0.0, 0.0],
        1.0,
    );

    let ivory = Material::new(
        Color::new(100.0, 100.0, 80.0),
        50.0,
        [0.6, 0.3, 0.0, 0.0],
        1.0,
    );

    let objects = vec![
        Sphere::new(Vec3::new(0.0, 0.0, -5.0), 1.0, rubber),
        Sphere::new(Vec3::new(2.0, 0.0, -5.0), 1.0, ivory),
    ];

    let light = Light::new(
        Vec3::new(5.0, 5.0, 5.0),
        Color::new(255.0, 255.0, 255.0),
        1.0,
    );

    let scene = Scene::new(objects, Vec3::new(0.0, 5.0, 0.0));

    // Variables para el control de la cámara
    let mut camera_distance = 5.0;
    let mut camera_yaw: f32 = 0.0;
    let mut camera_pitch: f32 = 0.0;
    let rotation_speed: f32 = 0.005;

    let mut is_left_mouse_button_pressed = false;
    let mut last_cursor_position: Option<PhysicalPosition<f64>> = None;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::MouseInput { button: MouseButton::Left, state, .. } => {
                    is_left_mouse_button_pressed = state == ElementState::Pressed;
                }
                WindowEvent::CursorMoved { position, .. } => {
                    if is_left_mouse_button_pressed {
                        if let Some(last_pos) = last_cursor_position {
                            let dx = (position.x - last_pos.x) as f32;
                            let dy = (position.y - last_pos.y) as f32;

                            // Actualizamos el ángulo de la cámara en función del movimiento del mouse
                            camera_yaw += dx * rotation_speed;
                            camera_pitch = (camera_pitch + dy * rotation_speed)
                                .clamp(-std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2);
                        }
                        last_cursor_position = Some(position);
                    } else {
                        last_cursor_position = Some(position);
                    }
                }
                _ => {}
            },
            Event::RedrawRequested(_) => {
                // Convertir el ángulo en posición de cámara
                let eye_x = camera_distance * camera_yaw.cos() * camera_pitch.cos();
                let eye_y = camera_distance * camera_pitch.sin();
                let eye_z = camera_distance * camera_yaw.sin() * camera_pitch.cos();

                let camera = Camera::new(
                    Vec3::new(eye_x, eye_y, eye_z),
                    Vec3::new(0.0, 0.0, -5.0),
                    Vec3::new(0.0, 1.0, 0.0),
                );

                render(&mut framebuffer, &camera, &scene, &light);
                render_framebuffer_to_pixels(&mut framebuffer, pixels.frame_mut());

                if pixels
                    .render()
                    .map_err(|e| eprintln!("pixels.render() failed: {}", e))
                    .is_err()
                {
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => {}
        }
        window.request_redraw();
    });
}

// Función para copiar el contenido del framebuffer al array de píxeles
fn render_framebuffer_to_pixels(framebuffer: &Framebuffer, frame: &mut [u8]) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = i % WIDTH as usize;
        let y = i / WIDTH as usize;

        // Obtener el color del framebuffer
        let color = framebuffer.get_pixel(x, y);

        // Convertir Vec3 a RGBA
        let rgba = [
            (color.x * 255.0) as u8,
            (color.y * 255.0) as u8,
            (color.z * 255.0) as u8,
            255, // Alfa
        ];

        pixel.copy_from_slice(&rgba);
    }
}