#[derive(Debug, Clone)]
pub struct Rectangle {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

impl Rectangle {
    pub fn area(&self) -> i32 {
        let width = (self.x2 - self.x1).max(0);
        let height = (self.y2 - self.y1).max(0);
        width * height
    }

    pub fn intersect(&self, other: &Rectangle) -> Option<Rectangle> {
        let x1 = self.x1.max(other.x1);
        let y1 = self.y1.max(other.y1);
        let x2 = self.x2.min(other.x2);
        let y2 = self.y2.min(other.y2);

        if x1 < x2 && y1 < y2 {
            Some(Rectangle { x1, y1, x2, y2 })
        } else {
            None
        }
    }
}

pub fn occupied_area(rects: &[Rectangle]) -> i32 {
    fn union_area(rects: &[Rectangle]) -> i32 {
        if rects.is_empty() {
            return 0;
        }
        if rects.len() == 1 {
            return rects[0].area();
        }

        let first = &rects[0];
        let rest = &rects[1..];

        let mut total = first.area() + union_area(rest);

        let mut intersections = Vec::new();
        for r in rest {
            if let Some(inter) = first.intersect(r) {
                intersections.push(inter);
            }
        }
        total -= union_area(&intersections);

        total
    }

    union_area(rects)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        let r1 = Rectangle { x1: 0, y1: 0, x2: 3, y2: 3 };
        let r2 = Rectangle { x1: 1, y1: 1, x2: 4, y2: 4 };
        let r3 = Rectangle { x1: 2, y1: 0, x2: 5, y2: 2 };

        let rects = vec![r1, r2, r3];

        // Очікувана площа покриття = 17
        assert_eq!(occupied_area(&rects), 17);
    }
}

