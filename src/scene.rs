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

    pub fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        let mut closest_intersection = Intersect::empty();
        for sphere in &self.spheres {
            let intersection = sphere.ray_intersect(ray_origin, ray_direction);
            if intersection.is_intersecting && intersection.distance < closest_intersection.distance {
                closest_intersection = intersection;
            }
        }
        closest_intersection
    }
}
