use nalgebra_glm::Vec3;
use crate::sphere::Sphere;
use crate::ray_intersect::RayIntersect;

pub fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Sphere]) -> u32 {
    for object in objects {
        if object.ray_intersect(ray_origin, ray_direction) {
            return 0xFFFFFF; // Color blanco si hay intersección
        }
    }
    0x000000 // Color negro si no hay intersección
}
