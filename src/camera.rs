use nalgebra_glm::Vec3;

pub struct Camera {
    pub eye: Vec3,     // Posición de la cámara en el espacio 3D
    pub center: Vec3,  // Punto que la cámara está observando
    pub up: Vec3,      // Vector que representa "arriba" para la cámara
}

impl Camera {
    // Constructor de la cámara
    pub fn new(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        Camera {
            eye,
            center,
            up,
        }
    }

    // Método para cambiar la base: transformar la dirección de un vector desde la perspectiva de la cámara
    pub fn basis_change(&self, vector: &Vec3) -> Vec3 {
        let forward = (self.center - self.eye).normalize();     // Eje Z de la cámara (mirando hacia adelante)
        let right = forward.cross(&self.up).normalize();        // Eje X de la cámara (producto cruz)
        let up = right.cross(&forward).normalize();             // Eje Y de la cámara (producto cruz del eje X y Z)

        let rotated = 
            vector.x * right + 
            vector.y * up - 
            vector.z * forward;

        rotated.normalize()  // Retorna el vector rotado en el sistema de la cámara, normalizado
    }
}
