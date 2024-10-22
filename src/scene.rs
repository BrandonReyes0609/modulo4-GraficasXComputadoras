use crate::sphere::Sphere;
use crate::ray_intersect::RayIntersect;  // Importa el rasgo RayIntersect
use crate::intersect::Intersect;
use nalgebra_glm::Vec3;

pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub light_position: Vec3,
}

impl Scene {
    pub fn new(spheres: Vec<Sphere>, light_position: Vec3) -> Self {
        Self { spheres, light_position }
    }

    pub fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect> {
        let mut closest_intersection: Option<Intersect> = None;

        for sphere in &self.spheres {
            if let Some(intersection) = sphere.ray_intersect(ray_origin, ray_direction) {
                if closest_intersection.is_none() || intersection.distance < closest_intersection.unwrap().distance {
                    closest_intersection = Some(intersection);
                }
            }
        }

        closest_intersection
    }
}
