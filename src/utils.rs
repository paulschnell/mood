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
        let m0 = (self.y1 - self.y0) / (self.x1 - self.x0);
        let b0 = self.y1 - m0 * self.x1;

        let m1 = (other.y1 - other.y0) / (other.x1 - other.x0);
        let b1 = other.y1 - m1 * other.x1;

        if m0 == m1 {
            return false;
        }

        if self.x1 == self.x0 {
            let ys = m1 * self.x1 + b1;
            if (other.y0 < ys && ys < other.y1) || (other.y0 > ys && ys > other.y1) {
                return true;
            }
        } else if other.x1 == other.x0 {
            let ys = m0 * other.x1 + b0;
            if (self.y0 < ys && ys < self.y1) || (self.y0 > ys && ys > self.y1) {
                return true;
            }
        } else {
            let xs = (b0 - b1) / (m1 - m0);
            let ys = m0 * xs + b0;
            if ((other.x0 < xs && xs < other.x1) || (other.x0 > xs && xs > other.x1))
                && ((other.y0 < ys && ys < other.y1) || (other.y0 > ys && ys > other.y1))
                && ((self.x0 < xs && xs < self.x1) || (self.x0 > xs && xs > self.x1))
                && ((self.y0 < ys && ys < self.y1) || (self.y0 > ys && ys > self.y1))
            {
                return true;
            }
        }

        false
    }
}
