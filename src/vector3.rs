#[derive(Copy, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn cross(&self, rhs: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn dot(&self, rhs: &Vector3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn zero() -> Vector3 {
        Vector3 {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    pub fn one() -> Vector3 {
        Vector3 {
            x: 1.,
            y: 1.,
            z: 1.,
        }
    }

    /// Rotates in the X-Y-Z order around the origin
    pub fn rotate_euler(&self, euler_angles: &Vector3) -> Vector3 {
        fn to_rad(v: f64) -> f64 {
            v * std::f64::consts::PI / 180.
        }

        let rad = Vector3::new(
            to_rad(euler_angles.x),
            to_rad(euler_angles.y),
            to_rad(euler_angles.z),
        );

        let result_x = Vector3 {
            x: self.x,
            y: self.y * rad.x.cos() - self.z * rad.x.sin(),
            z: self.z * rad.x.cos() + self.y * rad.x.sin(),
        };

        let result_xy = Vector3 {
            x: result_x.x * rad.y.cos() + result_x.z * rad.y.sin(),
            y: result_x.y,
            z: result_x.z * rad.y.cos() - result_x.x * rad.y.sin(),
        };

        let result_xyz = Vector3 {
            x: result_xy.x * rad.z.cos() - result_xy.y * rad.z.sin(),
            y: result_xy.y * rad.z.cos() + result_xy.x * rad.z.sin(),
            z: result_xy.z,
        };

        result_xyz
    }
}

impl std::ops::Mul for Vector3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Vector3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl std::ops::Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
