use nalgebra_glm::Vec3;
use crate::color::Color;
use crate::material::Material;
use crate::scene::Scene;
use crate::sphere::Sphere;
use crate::camera::Camera;
use crate::render::render;
use crate::framebuffer::Framebuffer;
use crate::light::Light;  // Importar Light
use pixels::{Pixels, SurfaceTexture};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

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
mod light;  // Asegúrate de tener este módulo

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {
    // Crear el EventLoop de winit
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Rust Graphics - Raytracer")
        .with_inner_size(winit::dpi::LogicalSize::new(WIDTH, HEIGHT))
        .build(&event_loop)
        .unwrap();

    // Crear un framebuffer donde renderizar los píxeles
    let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, &window);
    let mut pixels = Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap();

    // Crear una instancia de Framebuffer
    let mut framebuffer = Framebuffer::new(WIDTH as usize, HEIGHT as usize);

    // Inicializar la cámara
    let camera = Camera::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));

    // Inicializar los materiales y objetos
    let rubber = Material::new(
        Color::new(80.0, 0.0, 0.0),  // Color rojo
        1.0,                         // Coeficiente especular
        [0.9, 0.1, 0.0, 0.0],        // Albedo (90% difuso, 10% especular, sin reflexión ni transparencia)
        1.0                          // Índice de refracción (por ejemplo, 1.0 para aire)
    );
    // Opcional: Ejemplo de material para vidrio, si deseas añadir transparencia y refracción
    let glass = Material::new(
        Color::new(100.0, 100.0, 100.0), // Color transparente
        50.0,                            // Coeficiente especular alto
        [0.0, 0.5, 0.1, 0.9],            // Albedo con transparencia (50% especular, 10% reflexión, 90% transparencia)
        1.5                              // Índice de refracción (1.5 para vidrio)
    );

    let ivory = Material::new(
        Color::new(100.0, 100.0, 80.0),  // Color marfil
        50.0,                            // Coeficiente especular alto
        [0.6, 0.3, 0.0, 0.0],            // Albedo (60% difuso, 30% especular, sin reflexión ni transparencia)
        1.0                              // Índice de refracción (por ejemplo, 1.0 para aire o materiales no transparentes)
    );

    // Crear esferas con los materiales
    let objects = vec![
        Sphere::new(Vec3::new(0.0, 0.0, -5.0), 1.0, rubber),  // Usar material 'rubber'
        Sphere::new(Vec3::new(2.0, 0.0, -5.0), 1.0, ivory),   // Usar material 'ivory'
    ];

    // Inicializar la luz
    let light = Light::new(
        Vec3::new(5.0, 5.0, 5.0),
        Color::new(255.0, 255.0, 255.0), // Luz blanca
        1.0,
    );

    let scene = Scene::new(objects, Vec3::new(0.0, 5.0, 0.0));

    // Ejecutar el bucle de eventos para la ventana
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::RedrawRequested(_) => {
                // Renderizar la escena con el raytracer
                render(&mut framebuffer, &camera, &scene, &light);

                // Actualizar los píxeles en la ventana
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
        window.request_redraw(); // Solicitar una nueva actualización de ventana
    });
}

// Función para copiar el contenido del framebuffer al array de píxeles
fn render_framebuffer_to_pixels(framebuffer: &Framebuffer, frame: &mut [u8]) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = i % WIDTH as usize;
        let y = i / WIDTH as usize;

        // Obtener el color del framebuffer
        let color = framebuffer.get_pixel(x, y);

        // Convertir Vec3 a RGBA (se espera que el framebuffer devuelva un Vec3 con valores de 0.0 a 1.0)
        let rgba = [
            (color.x * 255.0) as u8,
            (color.y * 255.0) as u8,
            (color.z * 255.0) as u8,
            255, // Alfa
        ];

        pixel.copy_from_slice(&rgba);
    }
}
