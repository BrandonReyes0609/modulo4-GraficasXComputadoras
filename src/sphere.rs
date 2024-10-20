use nalgebra_glm::{dot, Vec3};
use crate::ray_intersect::RayIntersect;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl RayIntersect for Sphere {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> bool {
        // Vector desde el origen del rayo al centro de la esfera
        let oc = ray_origin - self.center;

        // Coeficientes de la ecuación cuadrática
        let a = dot(ray_direction, ray_direction);
        let b = 2.0 * dot(&oc, ray_direction);
        let c = dot(&oc, &oc) - self.radius * self.radius;

        // Discriminante
        let discriminant = b * b - 4.0 * a * c;

        // El rayo intersecta la esfera si el discriminante es mayor que cero
        discriminant > 0.0
    }
}
