#[allow(dead_code)]
pub struct Material {
    mass: f32,
    area: f32,
    density: f32,
}

#[allow(dead_code)]
impl Material {
    pub(crate) fn new(mass: f32, area: f32) -> Material {
        Material{
            mass,
            area,
            density: mass/area,
        }
    }
}

