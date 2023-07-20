use crate::Display;
use crate::Pixel;

#[derive(Debug, Default)]
pub struct Render {
    scale: usize,
    row: usize,
    col: usize,
    display: Display,
}

impl Render {
    fn new(&self, scale: usize, row: usize, col: usize) -> Self {
        Render {
            scale,
            row,
            col,
            display: Display::default(),
        }
    }

    fn constructor(&mut self) -> (usize, usize) {
        let width = self.col * self.scale;
        let height = self.row * self.scale;
        self.display = Display {
            0: vec![Pixel::default(); width * height],
        };

        return (width, height);
    }
    fn new_display(&mut self) {
        let width = self.col * self.scale;
        let height = self.row * self.scale;
        self.display = Display {
            0: vec![Pixel::default(); width * height],
        };
    }
    fn set_pixel(&mut self, mut x: usize, mut y: usize) -> isize {
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

    fn clear(&mut self) {
        self.constructor();
    }

    fn render(&self) {
        for (i, px) in self.display.0.iter().enumerate() {
            let x = (i % self.col) * self.scale;
            let y = (i / self.col) * self.scale;

            if px.p == 1 {
                // set ui color correspondingly at this pixel
                unimplemented!()
            }
        }
    }
}
