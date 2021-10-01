use crossterm::{
    cursor::{CursorShape, Hide, MoveTo, SetCursorShape},
    event, execute, queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal, ExecutableCommand, Result,
};
use std::io::{stdout, Write};

pub use Color::Rgb as RgbColor;

pub struct DrawBuffer {
    pub buffer: Vec<u8>,
    pub width: u16,
    pub height: u16,
}

impl DrawBuffer {
    pub fn new() -> DrawBuffer {
        let terminal_size = get_terminal_size();
        let buffer_size = DrawBuffer::get_buffer_size(terminal_size);
        let mut buffer = Vec::with_capacity(buffer_size);
        buffer.resize(buffer_size, 0);
        DrawBuffer {
            buffer,
            width: terminal_size.0,
            height: terminal_size.1,
        }
    }

    pub fn update_size(self: &mut DrawBuffer) {
        let terminal_size = get_terminal_size();

        if self.width != terminal_size.0 || self.height != terminal_size.1 {
            self.buffer
                .resize(DrawBuffer::get_buffer_size(terminal_size), 0);
            self.width = terminal_size.0;
            self.height = terminal_size.1;
        }
    }

    pub fn clear(self: &mut DrawBuffer) {
        self.buffer.fill(0);
    }

    pub fn get_color(&self, column: u16, row: u16) -> Color {
        let pos: usize = ((row * self.width + column) * 3).into();
        RgbColor {
            r: self.buffer[pos],
            g: self.buffer[pos + 1],
            b: self.buffer[pos + 2],
        }
    }

    fn get_buffer_size(terminal_size: (u16, u16)) -> usize {
        (terminal_size.0 * terminal_size.1 * 3).into()
    }
}
/// `(columns, rows)`
fn get_terminal_size() -> (u16, u16) {
    terminal::size().unwrap_or((1, 1))
}

pub fn render(buffer: &DrawBuffer) -> Result<()> {
    let mut stdout = stdout();
    let terminal_size = get_terminal_size();

    queue!(stdout, Hide)?;

    for row in 0..terminal_size.1 {
        queue!(stdout, MoveTo(0, row))?;

        for column in 0..terminal_size.0 {
            let color = buffer.get_color(column, row);

            // TODO: Only change background color if it has changed
            queue!(stdout, SetBackgroundColor(color))?;

            queue!(stdout, Print(" "))?;
        }
    }

    stdout.flush()?;

    Ok(())
}
