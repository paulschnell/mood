pub fn deg_to_rad(deg: f32) -> f32 {
    deg * nalgebra_glm::pi::<f32>() / 180.0
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

pub fn get_item<T>(list: &Vec<T>, index: isize) -> Option<&T> {
    if list.is_empty() {
        return None;
    }

    if index >= list.len() as isize {
        Some(&list[index as usize % list.len()])
    } else if index < 0 {
        Some(&list[(index % list.len() as isize + list.len() as isize) as usize])
    } else {
        Some(&list[index as usize])
    }
}

pub fn _get_item_mut<T>(list: &mut Vec<T>, index: isize) -> Option<&mut T> {
    if list.is_empty() {
        return None;
    }

    if index >= list.len() as isize {
        let len = list.len();
        Some(&mut list[index as usize % len])
    } else if index < 0 {
        let len = list.len();
        Some(&mut list[index as usize % len + len])
    } else {
        Some(&mut list[index as usize])
    }
}

pub fn index_of<T: std::cmp::PartialEq>(list: &Vec<T>, element: &T) -> Option<usize> {
    for i in 0..list.len() {
        if *element == list[i] {
            return Some(i);
        }
    }
    None
}
