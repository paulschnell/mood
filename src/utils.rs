use nalgebra_glm as ng;

pub fn deg_to_rad(deg: f32) -> f32 {
    deg * ng::pi::<f32>() / 180.0
}

pub struct Rect<T> {
    pub left: T,
    pub top: T,
    pub right: T,
    pub bottom: T,
}

impl<T> Rect<T> {
    pub fn new(left: T, top: T, right: T, bottom: T) -> Rect<T> {
        Rect {
            left,
            top,
            right,
            bottom,
        }
    }
}
