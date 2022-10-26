use std::ops::*;

static mut WIDTH: Option<usize> = None;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Point {
    pub x: usize, // →
    pub y: usize, // ↑
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }

    pub fn set_width(width: usize) {
        unsafe {
            WIDTH = Some(width);
        }
    }
}

type Value = usize; // 適宜変える
pub trait Mat {
    fn set(&mut self, p: &Point, value: Value);
    fn get(&self, p: &Point) -> Value;
}

impl Mat for Vec<Vec<Value>> {
    fn set(&mut self, p: &Point, value: Value) {
        self[p.y][p.x] = value;
    }

    fn get(&self, p: &Point) -> Value {
        self[p.y][p.x]
    }
}

impl Add for Point {
    type Output = Result<Point, &'static str>;
    fn add(self, rhs: Self) -> Self::Output {
        let (x, y) = if cfg!(debug_assertions) {
            // debugではオーバーフローでpanic発生するため、オーバーフローの溢れを明確に無視する(※1.60場合。それ以外は不明)
            (self.x.wrapping_add(rhs.x), self.y.wrapping_add(rhs.y))
        } else {
            (self.x + rhs.x, self.y + rhs.y)
        };

        unsafe {
            if let Some(width) = WIDTH {
                if x >= width || y >= width {
                    return Err("out of range");
                }
            }
        }

        Ok(Point { x, y })
    }
}

pub trait Swap<T> {
    fn swap(&mut self, p1: T, p2: T);
}

impl<T> Swap<Point> for Vec<Vec<T>>
where
    T: Copy,
{
    fn swap(&mut self, p1: Point, p2: Point) {
        let tmp = self[p1.y][p1.x];
        self[p1.y][p1.x] = self[p2.y][p2.x];
        self[p2.y][p2.x] = tmp;
    }
}

impl<T> Swap<&Point> for Vec<Vec<T>>
where
    T: Copy,
{
    fn swap(&mut self, p1: &Point, p2: &Point) {
        let tmp = self[p1.y][p1.x];
        self[p1.y][p1.x] = self[p2.y][p2.x];
        self[p2.y][p2.x] = tmp;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mat() {
        let n = 4;
        let mut mat = vec![vec![0; n]; n];
        for y in 0..n {
            for x in 0..n {
                let p = Point::new(x, y);
                mat.set(&p, n * y + x);
            }
        }
        let p1 = Point::new(0, 0);
        let p2 = Point::new(3, 3);
        let p3 = Point::new(0, 1);
        let p4 = Point::new(2, 2);

        assert_eq!(mat.get(&p1), 0);
        assert_eq!(mat.get(&p2), 15);
        mat.swap(&p1, &p2);
        assert_eq!(mat.get(&p1), 15);
        assert_eq!(mat.get(&p2), 0);
        assert_eq!(mat.get(&p3), 4);
        assert_eq!(mat.get(&p4), 10);
    }

    #[test]
    fn test_point_add_non_width() {
        let p1 = Point::new(1, 2);
        let p2 = Point::new(4, 5);
        let p = p1 + p2;
        let p = p.unwrap();
        assert_eq!(p.x, 5);
        assert_eq!(p.y, 7);

        Point::set_width(4);
        let p = p1 + p2;
        assert_eq!(p.is_ok(), false);
        assert_eq!(p.is_err(), true);

        Point::set_width(8);
        let p = p1 + p2;
        assert_eq!(p.is_ok(), true);
        assert_eq!(p.is_err(), false);
        let p = p.unwrap();
        assert_eq!(p.x, 5);
        assert_eq!(p.y, 7);
    }

    #[test]
    fn test_point_add_with_width() {
        let p1 = Point::new(1, 2);
        let p2 = Point::new(4, 5);

        Point::set_width(4);
        let p = p1 + p2;
        assert_eq!(p.is_ok(), false);
        assert_eq!(p.is_err(), true);

        Point::set_width(8);
        let p = p1 + p2;
        assert_eq!(p.is_ok(), true);
        assert_eq!(p.is_err(), false);
        let p = p.unwrap();
        assert_eq!(p.x, 5);
        assert_eq!(p.y, 7);
    }
}
