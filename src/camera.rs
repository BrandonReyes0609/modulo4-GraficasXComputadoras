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

    // Método para orbitar la cámara (Yaw y Pitch)
    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        let radius_vector = self.eye - self.center;
        let radius = radius_vector.magnitude();

        let current_yaw = radius_vector.z.atan2(radius_vector.x);
        let radius_xz = (radius_vector.x * radius_vector.x + radius_vector.z * radius_vector.z).sqrt();
        let current_pitch = (-radius_vector.y).atan2(radius_xz);

        let new_yaw = (current_yaw + delta_yaw) % (2.0 * std::f32::consts::PI);
        let new_pitch = (current_pitch + delta_pitch).clamp(-std::f32::consts::PI / 2.0 + 0.1, std::f32::consts::PI / 2.0 - 0.1);

        let new_eye = self.center + Vec3::new(
            radius * new_yaw.cos() * new_pitch.cos(),
            -radius * new_pitch.sin(),
            radius * new_yaw.sin() * new_pitch.cos(),
        );

        self.eye = new_eye;
    }

    // Método para realizar el cambio de base (transformar un vector desde la perspectiva de la cámara)
    pub fn basis_change(&self, vector: &Vec3) -> Vec3 {
        let forward = (self.center - self.eye).normalize();     // Eje Z de la cámara (mirando hacia adelante)
        let right = forward.cross(&self.up).normalize();        // Eje X de la cámara (producto cruz)
        let up = right.cross(&forward).normalize();             // Eje Y de la cámara

        let rotated = 
            vector.x * right + 
            vector.y * up - 
            vector.z * forward;

        rotated.normalize()  // Retorna el vector rotado en el sistema de la cámara, normalizado
    }
}
