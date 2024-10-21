use nalgebra_glm::Vec3;
use crate::color::Color;
use crate::material::Material;
use crate::scene::Scene;
use crate::sphere::Sphere;
use crate::camera::Camera;
use crate::render::render;
use crate::framebuffer::Framebuffer;

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

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);
    let camera = Camera::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));

    let red_material = Material::new(Color::new(255.0, 0.0, 0.0));
    let green_material = Material::new(Color::new(0.0, 255.0, 0.0));

    let objects = vec![
        Sphere::new(Vec3::new(0.0, 0.0, -5.0), 1.0, red_material),
        Sphere::new(Vec3::new(2.0, 0.0, -5.0), 1.0, green_material),
    ];

    let scene = Scene::new(objects, Vec3::new(0.0, 5.0, 0.0));
    render(&mut framebuffer, &camera, &scene);
}
