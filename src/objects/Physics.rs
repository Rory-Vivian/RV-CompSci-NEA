pub(crate) enum PhysicsType {
    Static,
    Dynamic,
    Kinematic,
}

pub struct Material {
    mass: f32,
    area: f32,
    density: f32,
}

impl Material {
    pub(crate) fn new(mass: f32, area: f32) -> Material {
        Material{
            mass,
            area,
            density: mass/area,
        }
    }
}



