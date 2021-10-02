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
        let a = self.pos + (self.size * Vector3::new(1., 1., 1.)).rotate_euler(&self.euler_angles);
        let b = self.pos + (self.size * Vector3::new(-1., 1., 1.)).rotate_euler(&self.euler_angles);
        let c =
            self.pos + (self.size * Vector3::new(-1., 1., -1.)).rotate_euler(&self.euler_angles);
        let d = self.pos + (self.size * Vector3::new(1., 1., -1.)).rotate_euler(&self.euler_angles);

        let e = self.pos + (self.size * Vector3::new(1., -1., 1.)).rotate_euler(&self.euler_angles);
        let f =
            self.pos + (self.size * Vector3::new(-1., -1., 1.)).rotate_euler(&self.euler_angles);
        let g =
            self.pos + (self.size * Vector3::new(-1., -1., -1.)).rotate_euler(&self.euler_angles);
        let h =
            self.pos + (self.size * Vector3::new(1., -1., -1.)).rotate_euler(&self.euler_angles);

        let faces: [Rectangle3D; 6] = [
            Rectangle3D {
                top_left: d,
                top_right: c,
                bottom_right: b,
                bottom_left: a,
            },
            Rectangle3D {
                top_left: a,
                top_right: b,
                bottom_right: f,
                bottom_left: e,
            },
            Rectangle3D {
                top_left: b,
                top_right: c,
                bottom_right: g,
                bottom_left: f,
            },
            Rectangle3D {
                top_left: c,
                top_right: d,
                bottom_right: h,
                bottom_left: g,
            },
            Rectangle3D {
                top_left: d,
                top_right: a,
                bottom_right: e,
                bottom_left: h,
            },
            Rectangle3D {
                top_left: e,
                top_right: f,
                bottom_right: g,
                bottom_left: h,
            },
        ];

        faces
    }

    pub fn render(&self, draw_buffer: &mut DrawBuffer) {
        let faces = self.get_faces();

        let forward = Vector3::new(0., 0., 1.);
        for (index, face) in faces.iter().enumerate() {
            let normal = face.scaled_normal();
            // Backface culling
            if forward.dot(&normal) > 0. {
                continue;
            }

            face.render(draw_buffer, index);
        }
    }
}

pub struct Rectangle3D {
    pub top_left: Vector3,
    pub top_right: Vector3,
    pub bottom_right: Vector3,
    pub bottom_left: Vector3,
}

const RAINBOW: [RgbColor; 6] = [
    RgbColor { r: 255, g: 0, b: 0 },
    RgbColor { r: 0, g: 255, b: 0 },
    RgbColor {
        r: 255,
        g: 255,
        b: 0,
    },
    RgbColor { r: 0, g: 0, b: 255 },
    RgbColor {
        r: 255,
        g: 0,
        b: 255,
    },
    RgbColor {
        r: 0,
        g: 255,
        b: 255,
    },
];

impl Rectangle3D {
    pub fn scaled_normal(&self) -> Vector3 {
        (self.bottom_left - self.top_left).cross(&(self.top_right - self.top_left))
    }

    pub fn render(&self, draw_buffer: &mut DrawBuffer, index: usize) {
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

        let top_left_2d: Vector2<f64> = self.top_left.into();
        let top_right_2d: Vector2<f64> = self.top_right.into();
        let bottom_left_2d: Vector2<f64> = self.bottom_left.into();
        let bottom_right_2d: Vector2<f64> = self.bottom_right.into();

        let horizontal_2d = Vector2::new(horizontal.x, horizontal.y);
        let vertical_2d = Vector2::new(vertical.x, vertical.y);

        let horizontal_2d_direction = horizontal_2d.normalized();
        let vertical_2d_direction = vertical_2d.normalized();

        let horizontal_2d_length = horizontal_2d.length();
        let vertical_2d_length = vertical_2d.length();

        // Iterate over draw buffer
        for column in bounding_box_min_2d.0..=bounding_box_max_2d.0 {
            for row in bounding_box_min_2d.1..=bounding_box_max_2d.1 {
                let point = unproject((column, row));

                // TODO: Fix those bad UVs
                let uv = (
                    (point - top_left_2d).dot(&horizontal_2d_direction)
                        / horizontal_2d_length.max(0.1),
                    (point - top_left_2d).dot(&vertical_2d_direction) / vertical_2d_length.max(0.1),
                );

                let b = (point - top_left_2d).wedge_product(&(top_right_2d - top_left_2d)) >= 0.
                    && (point - top_right_2d).wedge_product(&(bottom_right_2d - top_right_2d))
                        >= 0.
                    && (point - bottom_right_2d).wedge_product(&(bottom_left_2d - bottom_right_2d))
                        >= 0.
                    && (point - bottom_left_2d).wedge_product(&(top_left_2d - bottom_left_2d))
                        >= 0.;
                if b {
                    let col = &RAINBOW[index % RAINBOW.len()];

                    draw_buffer.set_color(
                        column,
                        row,
                        &RgbColor {
                            r: col.r / 2,
                            g: col.g / 2,
                            b: col.b / 2,
                        },
                    );

                    /*
                    if uv.0 < 0.1 {
                        // Draw the outline
                        draw_buffer.set_color(
                            column,
                            row,
                            &RgbColor {
                                r: 255,
                                g: 255,
                                b: 255,
                            },
                        )
                    } else {
                        // Draw this point
                        draw_buffer.set_color(
                            column,
                            row,
                            &RgbColor {
                                r: (uv.0 * 255.) as u8,
                                g: (uv.1 * 255.) as u8,
                                b: (index * 40) as u8,
                            },
                        )
                    }*/
                }
            }
        }

        {
            let tl = project(Vector2::new(self.top_left.x, self.top_left.y));
            let tr = project(Vector2::new(self.top_right.x, self.top_right.y));
            let bl = project(Vector2::new(self.bottom_left.x, self.bottom_left.y));
            let br = project(Vector2::new(self.bottom_right.x, self.bottom_right.y));
            draw_buffer.set_color(tl.0, tl.1, &RAINBOW[index % RAINBOW.len()]);
            draw_buffer.set_color(tr.0, tr.1, &RAINBOW[index % RAINBOW.len()]);
            draw_buffer.set_color(bl.0, bl.1, &RAINBOW[index % RAINBOW.len()]);
            draw_buffer.set_color(br.0, br.1, &RAINBOW[index % RAINBOW.len()]);
        }
        return;
    }
}
