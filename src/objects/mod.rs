pub(crate) mod square;
use square::*;

pub(crate) trait Render {
    fn render(&self);
}

pub fn render_objects(objects: Vec<Box<dyn Render>>) {
    for object in objects {
        object.render();
    }
}