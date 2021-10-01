pub struct Vector2<T> {
    x: T,
    y: T,
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

    // Hmmmmmmm, I'm really starting to reconsider this code.
    // Maybe generics weren't that good of a idea here.
    pub fn normalized(&self, zero: T) -> Vector2<T> {
        let length_squared = self.x * self.x + self.y * self.y;
        if length_squared > zero {
            Vector2 {
                x: self.x / length_squared,
                y: self.y / length_squared,
            }
        } else {
            Vector2 { x: zero, y: zero }
        }
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
