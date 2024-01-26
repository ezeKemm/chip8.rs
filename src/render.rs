use cursive::{
    event::{Event, EventResult, Key},
    theme::{BaseColor, Color, ColorStyle},
    views::Printer,
};

#[derive(Debug, Default)]
struct Display(Vec<Pixel>);

#[derive(Debug, Default, Clone)]
struct Pixel {
    p: isize,
}
#[derive(Debug, Default)]
pub struct Render {
    scale: usize,
    row: usize,
    col: usize,
    display: Display,
}

impl Render {
    pub fn new(scale: usize, row: usize, col: usize) -> Self {
        let mut render = Render {
            scale,
            row,
            col,
            display: Display::default(),
        };
        render.constructor();
        render
    }

    fn constructor(&mut self) {
        let width = self.col * self.scale;
        let height = self.row * self.scale;
        self.display = Display {
            0: vec![Pixel::default(); width * height],
        };

        // return (width, height);
    }
    pub fn set_pixel(&mut self, mut x: usize, mut y: usize) -> isize {
        if x > self.col {
            x -= self.col;
        } else if x < 0 {
            x += self.col;
        }

        if y > self.row {
            y -= self.row;
        } else if y < 0 {
            y += self.col;
        }

        let i = x + y * self.col;

        self.display.0[i].p ^= 1;

        !self.display.0[i].p
    }

    pub fn clear(&mut self) {
        self.constructor();
    }

    pub fn render(&mut self) {
        for i in 0..self.display.0.len() {
            let x = (i % self.col) * self.scale;
            let y = (i / self.col) * self.scale;

            if self.display.0[i].p == 1 {
                // set ui color correspondingly at this pixel
                self.set_pixel(x, y);
            }
        }
    }
}

impl cursive::view::View for Render {
    fn draw(&self, printer: &Printer) {
        for i in 0..self.display.0.len() {
            let x = (i % self.col) * self.scale;
            let y = (i / self.col) * self.scale;

            let mut color: Color;
            if self.display.0[i].p == 1 {
                // set ui color correspondingly at this pixel
                color = Color::RgbLowRes(0, 0, 0);
                self.set_pixel(x, y);
            }
            printer.with_color(
                ColorStyle::new(Color::Dark(BaseColor::Black), color),
                |printer| printer.print((x, y), ""),
            );
        }
    }
    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Char(char) => {}
            _ => {}
        }
        let c = 'a';
        let p = c.to_ascii_uppercase();
        EventResult::Ignored
    }
}
