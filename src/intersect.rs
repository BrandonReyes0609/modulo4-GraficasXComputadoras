use crate::material::Material;

#[derive(Debug, Clone, Copy)]
pub struct Intersect {
    pub distance: f32,
    pub is_intersecting: bool,
    pub material: Material,
}

impl Intersect {
    pub fn new(distance: f32, material: Material) -> Self {
        Intersect {
            distance,
            is_intersecting: true,
            material,
        }
    }

    pub fn empty() -> Self {
        Intersect {
            distance: 0.0,
            is_intersecting: false,
            material: Material::black(),
        }
    }
}
