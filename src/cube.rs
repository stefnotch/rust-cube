use crate::terminal_renderer::{DrawBuffer, RgbColor};
use crate::vector2::Vector2;
use crate::vector3::Vector3;

pub struct Cube {
    pub pos: Vector3,
    pub size: Vector3,
    pub euler_angles: Vector3,
}

impl Cube {
    pub fn unit() -> Cube {
        Cube {
            pos: Vector3::zero(),
            size: Vector3::new(1., 1., 1.),
            euler_angles: Vector3::zero(),
        }
    }

    /// `(top, sides, bottom)`
    pub fn get_faces(&self) -> [Rectangle3D; 6] {
        let a = self.pos + self.size * Vector3::new(1., 1., 1.).rotate_euler(&self.euler_angles);
        let b = self.pos + self.size * Vector3::new(-1., 1., 1.).rotate_euler(&self.euler_angles);
        let c = self.pos + self.size * Vector3::new(-1., 1., -1.).rotate_euler(&self.euler_angles);
        let d = self.pos + self.size * Vector3::new(1., 1., -1.).rotate_euler(&self.euler_angles);

        let e = self.pos + self.size * Vector3::new(1., -1., 1.).rotate_euler(&self.euler_angles);
        let f = self.pos + self.size * Vector3::new(-1., -1., 1.).rotate_euler(&self.euler_angles);
        let g = self.pos + self.size * Vector3::new(-1., -1., -1.).rotate_euler(&self.euler_angles);
        let h = self.pos + self.size * Vector3::new(1., -1., -1.).rotate_euler(&self.euler_angles);

        let faces: [Rectangle3D; 6] = [
            Rectangle3D {
                top_left: c,
                top_right: d,
                bottom_right: a,
                bottom_left: b,
            },
            Rectangle3D {
                top_left: b,
                top_right: a,
                bottom_right: h,
                bottom_left: g,
            },
            Rectangle3D {
                top_left: c,
                top_right: b,
                bottom_right: g,
                bottom_left: f,
            },
            Rectangle3D {
                top_left: d,
                top_right: c,
                bottom_right: f,
                bottom_left: e,
            },
            Rectangle3D {
                top_left: a,
                top_right: d,
                bottom_right: e,
                bottom_left: h,
            },
            Rectangle3D {
                top_left: g,
                top_right: h,
                bottom_right: e,
                bottom_left: f,
            },
        ];

        faces
    }

    pub fn render(&self, draw_buffer: &mut DrawBuffer) {
        let faces = self.get_faces();

        let forward = Vector3::new(0., 0., 1.);
        for face in faces {
            let normal = face.scaled_normal();
            // Backface culling
            if forward.dot(&normal) > 0. {
                continue;
            }

            face.render(draw_buffer);
        }
    }
}

pub struct Rectangle3D {
    top_left: Vector3,
    top_right: Vector3,
    bottom_right: Vector3,
    bottom_left: Vector3,
}

impl Rectangle3D {
    pub fn scaled_normal(&self) -> Vector3 {
        (self.bottom_left - self.top_left).cross(&(self.top_right - self.top_left))
    }

    fn render(&self, draw_buffer: &mut DrawBuffer) {
        let horizontal = self.top_right - self.top_left;
        let vertical = self.bottom_left - self.top_left;

        // Find bounding box
        let mut bounding_box_min = Vector2::new(self.top_left.x, self.top_left.y);
        let mut bounding_box_max = Vector2::new(self.top_left.x, self.top_left.y);
        for point in [self.top_right, self.bottom_right, self.bottom_left] {
            let point = Vector2::new(point.x, point.y);

            bounding_box_min = bounding_box_min.min(&point);
            bounding_box_max = bounding_box_max.max(&point);
        }

        // Viewport: -1 to 1 range, with (0,0) at the center
        // Transform to draw buffer size
        let half_size = 0.5 * (draw_buffer.width.min(draw_buffer.height) as f64);
        let half_width = (draw_buffer.width / 2) as f64;
        let half_height = (draw_buffer.height / 2) as f64;
        let project = |v: Vector2<f64>| {
            (
                (v.x * half_size + half_width) as u16,
                (v.y * half_size + half_height) as u16,
            )
        };

        let unproject = |v: (u16, u16)| Vector2 {
            x: (v.0 as f64 - half_width) / half_size,
            y: (v.1 as f64 - half_height) / half_size,
        };

        // Transformed coordinates
        let bounding_box_min_2d = project(bounding_box_min);
        let bounding_box_max_2d = project(bounding_box_max);

        let top_left_2d = Vector2::new(self.top_left.x, self.top_left.y);
        let horizontal_2d = Vector2::new(horizontal.x, horizontal.y);
        let vertical_2d = Vector2::new(vertical.x, vertical.y);

        let horizontal_2d_direction = horizontal_2d.normalized();
        let vertical_2d_direction = vertical_2d.normalized();

        let horizontal_2d_length = horizontal_2d.length();
        let vertical_2d_length = vertical_2d.length();

        // Iterate over draw buffer
        for column in bounding_box_min_2d.0..bounding_box_max_2d.0 {
            for row in bounding_box_min_2d.1..bounding_box_max_2d.1 {
                let point = unproject((column, row));
                let uv = (
                    (point - top_left_2d).dot(&horizontal_2d_direction) / horizontal_2d_length,
                    (point - top_left_2d).dot(&vertical_2d_direction) / vertical_2d_length,
                );

                if uv.0 < 0. || uv.1 < 0. || uv.0 >= 1. || uv.1 >= 1. {
                    continue;
                }

                // Draw this point
                draw_buffer.set_color(
                    column,
                    row,
                    RgbColor {
                        r: (uv.0 * 255.) as u8,
                        g: (uv.1 * 255.) as u8,
                        b: 0,
                    },
                )
            }
        }
    }
}
