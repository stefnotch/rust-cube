use std::time::Duration;

use crossterm::event;
use game_loop::game_loop;

mod rust_cube;
mod terminal_renderer;

struct Game {
    pub draw_buffer: terminal_renderer::DrawBuffer,
}

impl Game {
    fn update(&mut self) {
        self.draw_buffer.update_size();
        self.draw_buffer.clear();

        self.draw_buffer.buffer[0] = 220u8;
        self.draw_buffer.buffer[1] = 200u8;
        self.draw_buffer.buffer[10] = 180u8;
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
        30,
        0.1,
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
