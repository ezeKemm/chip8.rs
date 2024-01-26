use crate::Vec2;
use sdl2::{
    audio::{AudioCallback, AudioDevice, AudioSpecDesired},
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::Canvas,
    video::Window,
};

enum Pixel {
    Off,
    On,
}

pub struct Renderer {
    canvas: Canvas<Window>,
    size: Vec2,
    scale: u32,
}

impl Renderer {
    pub fn new(ctx: sdl2::Sdl, size: Vec2, scale: u32) -> Self {
        let mut vidsub = ctx.video().unwrap();
        let mut window = vidsub
            .window("Chip-8 Emulator", size.width * scale, size.height * scale)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        Renderer {
            canvas,
            size,
            scale,
        }
    }

    pub fn draw(&mut self, display: [u8; 2048], clr: bool) {
        let width = self.size.width as usize;
        let scale = self.scale;
        for (i, &px) in display.iter().enumerate() {
            let x = (i % width) as i32 * scale as i32;
            let y = (i / width) as i32 * scale as i32;
            let color = if px == 0 || clr {
                Color::RGB(0, 0, 0)
            } else {
                Color::RGB(0, 250, 0)
            };
            self.canvas.set_draw_color(color);
            self.canvas.fill_rect(Rect::new(x, y, scale, scale));
        }
        self.canvas.present();
    }
}

pub struct Input {
    events: sdl2::EventPump,
}

impl Input {
    pub fn new(ctx: sdl2::Sdl) -> Self {
        Input {
            events: ctx.event_pump().unwrap(),
        }
    }

    pub fn poll(&mut self) -> Result<[bool; 16], ()> {
        for event in self.events.poll_iter() {
            if let Event::Quit { .. } = event {
                return Err(());
            };
        }

        let keys: Vec<Keycode> = self
            .events
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        let mut chipkeys = [false; 16];

        for key in keys {
            let index = match key {
                Keycode::Num1 => Some(0x1),
                Keycode::Num2 => Some(0x2),
                Keycode::Num3 => Some(0x3),
                Keycode::Num4 => Some(0xc),
                Keycode::Q => Some(0x4),
                Keycode::W => Some(0x5),
                Keycode::E => Some(0x6),
                Keycode::R => Some(0xd),
                Keycode::A => Some(0x7),
                Keycode::S => Some(0x8),
                Keycode::D => Some(0x9),
                Keycode::F => Some(0xe),
                Keycode::Z => Some(0xa),
                Keycode::X => Some(0x0),
                Keycode::C => Some(0xb),
                Keycode::V => Some(0xf),
                _ => None,
            };
            if let Some(i) = index {
                chipkeys[i] = true;
            }
        }
        Ok(chipkeys)
    }
}

pub struct Audio {
    audio: AudioDevice<SquareWave>,
}

impl Audio {
    pub fn new(ctx: sdl2::Sdl) {
        let audsub = ctx.audio().unwrap();
        let device = audsub.open_playback(
            None,
            &AudioSpecDesired {
                freq: Some(44100),
                channels: Some(1),
                samples: None,
            },
            |spec| SquareWave {
                phase_inc: 240.0 / spec.freq as f32,
                phase: 0.0,
                vol: 0.25,
            },
        );
    }
}
struct SquareWave {
    phase: f32,
    vol: f32,
    phase_inc: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [Self::Channel]) {
        for x in out.iter_mut() {
            *x = self.vol * if self.phase < 0.5 { 1.0 } else { -1.0 };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}
fn main() {}
