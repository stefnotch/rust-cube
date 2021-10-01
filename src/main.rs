use std::time::Duration;

use crossterm::event;
use cube::Cube;
use game_loop::game_loop;
use vector3::Vector3;

mod cube;
mod terminal_renderer;
mod vector2;
mod vector3;

struct Game {
    pub draw_buffer: terminal_renderer::DrawBuffer,
}

impl Game {
    fn update(&mut self) {
        self.draw_buffer.update_size();
        self.draw_buffer.clear();

        let cube = Cube {
            pos: Vector3::zero(),
            size: Vector3::new(0.5, 0.5, 0.5),
        };

        // TODO: Move down to the render function
        cube.render(&mut self.draw_buffer);
    }

    fn render(&self) {
        // TODO: Maybe don't ignore all errors?
        match terminal_renderer::render(&self.draw_buffer) {
            Ok(_) => {}
            Err(_) => {}
        }
    }
}

fn main() {
    let game = Game {
        draw_buffer: terminal_renderer::DrawBuffer::new(),
    };

    game_loop(
        game,
        15,
        0.2,
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
                        _ => {}
                    },
                    Err(_) => {}
                }
            }

            g.game.update();
        },
        |g| {
            g.game.render();
        },
    );
}
