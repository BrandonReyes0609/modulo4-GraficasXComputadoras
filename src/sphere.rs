use crate::ray_intersect::RayIntersect;
use crate::intersect::Intersect;
use crate::material::Material;
use nalgebra_glm::{Vec3, dot};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl RayIntersect for Sphere {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect> {
        let oc = ray_origin - self.center;
        let a = dot(ray_direction, ray_direction);
        let b = 2.0 * dot(&oc, ray_direction);
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        
        if discriminant < 0.0 {
            // Si no hay intersección, devuelve `None`
            return None;
        }
        
        let t = (-b - discriminant.sqrt()) / (2.0 * a);
        if t > 0.0 {
            // Si hay intersección, calcula el punto de impacto y la normal
            let hit_point = ray_origin + ray_direction * t;
            let normal = (hit_point - self.center).normalize();
            return Some(Intersect {
                distance: t,
                point: hit_point,
                normal,
                material: self.material.clone(),
                is_intersecting: true,
            });
        }
        
        // Devuelve `None` si no se encuentra intersección
        None
    }
}
