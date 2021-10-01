use crossterm::{
    cursor::{Hide, MoveTo},
    event, execute, queue,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
    terminal, Result,
};
use std::io::{stdout, Write};

pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<RgbColor> for crossterm::style::Color {
    fn from(item: RgbColor) -> Self {
        Color::Rgb {
            r: item.r,
            g: item.g,
            b: item.b,
        }
    }
}

const HEIGHT_SCALE: u16 = 2; // TODO: This is totally a hack;

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

    pub fn get_color(&self, column: u16, row: u16) -> RgbColor {
        if column >= self.width || row >= self.height {
            return RgbColor { r: 0, g: 0, b: 0 };
        }

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

    pub(crate) fn set_color(&mut self, column: u16, row: u16, color: RgbColor) {
        if column >= self.width || row >= self.height {
            return;
        }

        let pos: usize = ((row * self.width + column) * 3).into();
        self.buffer[pos] = color.r;
        self.buffer[pos + 1] = color.g;
        self.buffer[pos + 2] = color.b;
    }
}
/// `(columns, rows)`
fn get_terminal_size() -> (u16, u16) {
    let size = terminal::size().unwrap_or((1, 1));
    (size.0, size.1 * HEIGHT_SCALE)
}

pub fn render(buffer: &DrawBuffer) -> Result<()> {
    let mut stdout = stdout();
    let terminal_size = get_terminal_size();

    queue!(stdout, Hide)?;

    for row in (0..terminal_size.1).step_by(HEIGHT_SCALE.into()) {
        queue!(stdout, MoveTo(0, row / 2))?;

        for column in 0..terminal_size.0 {
            let color = buffer.get_color(column, row);

            // TODO: Only change background color if it has changed
            queue!(stdout, SetBackgroundColor(color.into()))?;

            if HEIGHT_SCALE >= 2 {
                let bottom_color = buffer.get_color(column, row + 1);
                queue!(stdout, SetForegroundColor(bottom_color.into()))?;
                queue!(stdout, Print("_"))?; // TODO: Print "Lower half block"
            } else {
                queue!(stdout, Print(" "))?;
            }
        }
    }

    stdout.flush()?;

    Ok(())
}
