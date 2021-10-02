use std::{io::stdout, time::Duration};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, MouseButton},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};
use cube::{Cube, Rectangle3D};
use game_loop::game_loop;
use vector3::Vector3;

mod cube;
mod terminal_renderer;
mod vector2;
mod vector3;

struct Game {
    pub draw_buffer: terminal_renderer::DrawBuffer,
    pub rotation: Vector3,
    pub mouse_down_pos: (u16, u16),
}

impl Game {
    fn update(&mut self, seconds: f64, rotation: Vector3) {
        self.draw_buffer.update_size();
        self.draw_buffer.clear();

        let cube = Cube {
            pos: Vector3::zero(),
            size: Vector3::new(0.5, 0.5, 0.5),
            euler_angles: rotation, //Vector3::new(10. * seconds, 0. * seconds, 0. * seconds),
        };

        // TODO: Move to draw function
        cube.render(&mut self.draw_buffer);

        // let rect = Rectangle3D {
        //     top_left: Vector3::new(-0.6, 0.25, 0.5),
        //     top_right: Vector3::new(-0.25, -0.6, 0.5),
        //     bottom_right: Vector3::new(0.6, -0.25, 0.5),
        //     bottom_left: Vector3::new(0.25, 0.6, 0.5),
        // };

        // let rect = Rectangle3D {
        //     top_left: Vector3::new(-0.6, 0.25, -0.5),
        //     top_right: Vector3::new(-0.25, -0.6, -0.5),
        //     bottom_right: Vector3::new(-0.25, -0.6, 0.5),
        //     bottom_left: Vector3::new(-0.6, 0.25, 0.5),
        // };

        // rect.render(&mut self.draw_buffer, 0);
    }

    // TODO: Make immutable self
    fn render(&mut self) {
        // TODO: Maybe don't ignore all errors?
        match terminal_renderer::render(&self.draw_buffer) {
            Ok(_) => {}
            Err(_) => {}
        }
    }
}

fn main() -> Result<()> {
    let game = Game {
        draw_buffer: terminal_renderer::DrawBuffer::new(),
        rotation: Vector3::zero(),
        mouse_down_pos: (0, 0),
    };

    //enable_raw_mode()?;
    execute!(stdout(), EnableMouseCapture)?;

    game_loop(
        game,
        10,
        0.5,
        |g| {
            let has_event = event::poll(Duration::from_secs(0)).unwrap_or(false);
            if has_event {
                match event::read() {
                    Ok(ev) => match ev {
                        event::Event::Key(k) => {
                            if k.code == event::KeyCode::Esc {
                                g.exit();
                            }
                        }
                        event::Event::Mouse(m) => match m.kind {
                            event::MouseEventKind::Down(button) => {
                                if button == MouseButton::Left {
                                    g.game.mouse_down_pos = (m.column, m.row);
                                }
                            }
                            event::MouseEventKind::Drag(button) => {
                                if button == MouseButton::Left {
                                    let delta = (
                                        m.column as i16 - g.game.mouse_down_pos.0 as i16,
                                        m.row as i16 - g.game.mouse_down_pos.1 as i16,
                                    );
                                    g.game.mouse_down_pos = (m.column, m.row);
                                    g.game.rotation.y += delta.0 as f64;
                                    g.game.rotation.x -= delta.1 as f64;
                                }
                            }
                            event::MouseEventKind::Up(button) => {
                                if button == MouseButton::Left {
                                    let delta = (
                                        m.column as i16 - g.game.mouse_down_pos.0 as i16,
                                        m.row as i16 - g.game.mouse_down_pos.1 as i16,
                                    );
                                    g.game.mouse_down_pos = (m.column, m.row);
                                    g.game.rotation.y += delta.0 as f64;
                                    g.game.rotation.x -= delta.1 as f64;
                                }
                            }
                            _ => {}
                        },
                        _ => {}
                    },
                    Err(_) => {}
                }
            }

            g.game.update(g.running_time(), g.game.rotation);
        },
        |g| {
            g.game.render();
        },
    );

    execute!(stdout(), DisableMouseCapture)?;
    // disable_raw_mode()?;

    Ok(())
}
