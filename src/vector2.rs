#[derive(Copy, Clone)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<
        T: std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>
            + std::cmp::PartialOrd<T>
            + std::ops::Add<Output = T>
            + Copy,
    > Vector2<T>
{
    pub fn new(x: T, y: T) -> Vector2<T> {
        Vector2 { x, y }
    }

    pub fn dot(&self, rhs: &Vector2<T>) -> T {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl Vector2<f64> {
    pub fn min(&self, rhs: &Vector2<f64>) -> Vector2<f64> {
        Vector2 {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
        }
    }

    pub fn max(&self, rhs: &Vector2<f64>) -> Vector2<f64> {
        Vector2 {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
        }
    }

    pub fn normalized(&self) -> Vector2<f64> {
        let length_squared = self.x * self.x + self.y * self.y;
        if length_squared > f64::EPSILON {
            let length = length_squared.sqrt();
            Vector2 {
                x: self.x / length,
                y: self.y / length,
            }
        } else {
            Vector2 { x: 0., y: 0. }
        }
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl<T: std::ops::Mul<Output = T>> std::ops::Mul for Vector2<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T: std::ops::Add<Output = T>> std::ops::Add for Vector2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: std::ops::Sub<Output = T>> std::ops::Sub for Vector2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
