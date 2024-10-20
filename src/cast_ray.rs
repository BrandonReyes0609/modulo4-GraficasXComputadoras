use nalgebra_glm::Vec3;

// Estructura vacía para representar objetos
pub struct Object;

// Función para simular el lanzamiento de un rayo
pub fn cast_ray(origin: &Vec3, direction: &Vec3, _objects: &[Object]) -> Vec3 {
    // Para simplicidad, se devuelve un color rojo por defecto
    Vec3::new(1.0, 0.0, 0.0) // Color rojo
}
