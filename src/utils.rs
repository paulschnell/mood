pub fn deg_to_rad(deg: f64) -> f64 {
    deg * nalgebra_glm::pi::<f64>() / 180.0
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

pub struct Line {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
}

impl Line {
    pub fn new(x0: f64, y0: f64, x1: f64, y1: f64) -> Self {
        Line { x0, y0, x1, y1 }
    }

    pub fn new_tuples(pos0: (f64, f64), pos1: (f64, f64)) -> Self {
        Line {
            x0: pos0.0,
            y0: pos0.1,
            x1: pos1.0,
            y1: pos1.1,
        }
    }

    pub fn crosses(&self, other: &Line) -> bool {
        let line0 = self.clone();
        let line1 = other.clone();
        let m0 = (line0.y1 - line0.y0) / (line0.x1 - line0.x0);
        let b0 = line0.y1 - m0 * line0.x1;

        let m1 = (line1.y1 - line1.y0) / (line1.x1 - line1.x0);
        let b1 = line1.y1 - m1 * line1.x1;

        if m0 == m1 {
            return false;
        }

        if line0.x1 == line0.x0 {
            let ys = m1 * line0.x1 + b1;
            if (line1.y0 < ys && ys < line1.y1) || (line1.y0 > ys && ys > line1.y1) {
                return true;
            }
        } else if line1.x1 == line1.x0 {
            let ys = m0 * line1.x1 + b0;
            if (line0.y0 < ys && ys < line0.y1) || (line0.y0 > ys && ys > line0.y1) {
                return true;
            }
        } else {
            let xs = (b1 - b0) / (m0 - m1);
            if (line1.x0 < xs && xs < line1.x1) || (line1.x0 > xs && xs > line1.x1) {
                return true;
            }
        }

        false
    }
}
