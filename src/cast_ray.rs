use nalgebra_glm::Vec3;
use crate::sphere::Sphere;
use crate::intersect::Intersect;

pub fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Sphere]) -> Intersect {
    let mut closest_intersect = Intersect::empty();
    let mut closest_distance = f32::MAX;

    for object in objects.iter() { // Uso de `.iter()` para iterar sobre los objetos
        let intersect = object.ray_intersect(ray_origin, ray_direction);
        if intersect.is_intersecting && intersect.distance < closest_distance {
            closest_distance = intersect.distance;
            closest_intersect = intersect;
        }
    }

    closest_intersect
}
