use rand::Rng;
use std::collections::HashMap;

mod render;
mod ui;

use crate::ui::{Audio, Input, Renderer};

const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;
const SCALE: u32 = 10;

struct CPU {
    mem: Memory,
    vram: [u8; 2048],
    pc: ProgramCounter,
    stack: Stack,
    I: u16,
    V: [u16; 16],
    delay: u16,
    sound: u16,
}
struct Vec2 {
    width: u32,
    height: u32,
}
struct Font([i32; 80]);
struct ProgramCounter(u16);
struct Memory([u8; 4096]);
struct Stack(Vec<u16>);

impl Vec2 {
    fn prod(&self) -> usize {
        (self.width * self.height) as usize
    }
}

impl CPU {
    fn init() -> Self {
        let font = Font([
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ]);
        let mem = Memory([0; 4096]);
        let pc = ProgramCounter(0);
        let stack = Stack(vec![]);
        let I = 0;
        let V = [0; 16];
        let sound = 0;
        let delay = 0;
        let mut cpu = CPU {
            mem,
            pc,
            stack,
            I,
            V,
            delay,
            sound,
            vram: [0; 2048],
        };
        cpu.frontload_font(font);
        cpu
    }

    fn frontload_font(&mut self, font: Font) {
        let start: usize = 0x0;
        for i in 0..font.0.len() {
            self.mem.0[start + i] = font.0[i] as u8;
        }
    }
}
fn main() {
    let ctx = sdl2::init().unwrap();
    let keyboard = Input::new(ctx);
    let speaker = Audio::new(ctx);
    let mut display = Renderer::new(
        ctx,
        Vec2 {
            width: 64,
            height: 32,
        },
        10,
    );
    let mut cpu = CPU::init();
    loop {
        let opcode = cpu.fetch();
        let _ = cpu.execute(opcode);
    }
}

impl CPU {
    fn fetch(&mut self) -> u16 {
        let i = self.pc.0 as usize;
        let opcode = self.mem.0[i] << 8 | self.mem.0[i + 1];
        self.pc.increment(2);
        opcode as u16
    }
    fn execute(&mut self, opcode: u16) {
        let x = (opcode & 0x0F00 >> 8) as usize; // the 2nd nibble
        let y = (opcode & 0x00F0 >> 4) as usize; // the 3rd nibble
        let n = (opcode & 0x0FFF) as usize;

        match opcode & 0xF000 {
            0x00E0 => {
                for bit in self.vram.iter_mut() {
                    *bit = 0;
                }
            } // CLS -- clear screen
            0x00EE => self.pc.0 = self.stack.pop(0) as u16, // RET
            0x0000 => {}                                    // SYS // IGNORE
            0x1000 => self.pc.0 = n as u16,                 // JMP
            0x2000 => {
                // CALL
                self.stack.push(self.pc.0);
                self.pc.0 = opcode & 0xFFF
            }
            0x3000 => {
                // SE Vx, byte
                if self.V[x] == 0xFF {
                    self.pc.increment(2);
                }
            }
            0x4000 => {
                // SNE Vx, byte
                if self.V[x] != 0xFF {
                    self.pc.increment(2);
                }
            }
            0x5000 => {
                // SE Vx, Vy
                if self.V[x] == self.V[y] {
                    self.pc.increment(2);
                }
            }
            0x6000 => {
                // LD Vx, byte
                self.V[x] = opcode & 0xFF;
            }
            0x7000 => {
                // ADD Vx, byte
                self.V[x] += opcode & 0xFF;
            }
            0x8000 => {
                match opcode & 0xF {
                    0x0 => self.V[x] = self.V[y],             // LD
                    0x1 => self.V[x] = self.V[x] | self.V[y], // OR
                    0x2 => self.V[x] = self.V[x] & self.V[y], // AND
                    0x3 => self.V[x] = self.V[x] ^ self.V[y], // XOR
                    0x4 => {
                        // ADD Vx, Vy
                        self.V[0xF] = 0;
                        let sum = self.V[x] + self.V[y];
                        if sum > 0xFF {
                            // if Vx + Vy > 255
                            self.V[0xF] = 1;
                        }
                        self.V[x] = (sum & 0xFF) >> 8; // get the lowest 8 bits
                    }
                    0x5 => {
                        // SUB Vx, Vy
                        self.V[0xF] = 0;
                        let sum = self.V[y] - self.V[x];
                        if self.V[x] > self.V[y] {
                            self.V[0xF] = 1;
                        }
                        self.V[x] = sum;
                    }
                    0x6 => {
                        self.V[0xF] = 0;
                        if self.V[y] & 0xF == 1 {
                            self.V[0xF] = 1;
                        }
                        self.V[x] /= 2;
                    } // SHR
                    0x7 => {
                        self.V[0xF] = 0;
                        if self.V[y] > self.V[x] {
                            self.V[0xF] = 1;
                        }
                        self.V[x] = self.V[y] - self.V[x];
                    } // SUBN
                    0xE => {
                        self.V[0xF] = 0;
                        if self.V[x] & 0xF000 == 1 {
                            self.V[0xF] = 1;
                        }
                        self.V[x] *= 2;
                    } // SHL
                    _ => {}
                }
            }
            0x9000 => {
                if self.V[x] != self.V[y] {
                    self.pc.increment(2);
                }
            } // SNE
            0xA000 => {
                self.I = opcode & 0xFFF;
            } // LD
            0xB000 => {
                self.pc.0 = (opcode & 0xFFF) + self.V[0];
            } // JMP
            0xC000 => {
                let mut rng = rand::thread_rng();
                let r: u16 = rng.gen_range(0..255);
                self.V[x] = r & (opcode & 0xFF);
            } // RND
            0xD000 => {} // DRW
            0xE000 => {
                match opcode & 0xFF {
                    0x9E => {} // SKP
                    0xA1 => {} // SKNP
                    _ => {}
                }
            }
            0xF000 => {
                match opcode & 0xFF {
                    0x07 => {} // LD
                    0x0A => {} // LD
                    0x15 => {} // LD
                    0x18 => {} // LD
                    0x1E => {} // ADD
                    0x29 => {} // LD
                    0x33 => {} // LD
                    0x55 => {} // LD
                    0x65 => {} // LD
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
fn load_program(mem: &mut Memory, rom: Vec<u8>) {
    for (i, bit) in rom.iter().enumerate() {
        mem.0[0x200 + i] = *bit;
    }
}
struct Keyboard(HashMap<usize, isize>);
impl Default for Keyboard {
    fn default() -> Self {
        let mut keymap: HashMap<usize, isize> = HashMap::new();
        keymap.insert(49, 0x1); // 1
        keymap.insert(50, 0x2); // 2
        keymap.insert(51, 0x3); // 3
        keymap.insert(52, 0xc); // 4
        keymap.insert(81, 0x4); // Q
        keymap.insert(87, 0x5); // W
        keymap.insert(69, 0x6); // E
        keymap.insert(82, 0xD); // R
        keymap.insert(65, 0x7); // A
        keymap.insert(83, 0x8); // S
        keymap.insert(68, 0x9); // D
        keymap.insert(70, 0xE); // F
        keymap.insert(90, 0xA); // Z
        keymap.insert(88, 0x0); // X
        keymap.insert(67, 0xB); // C
        keymap.insert(86, 0xF); // V
        Keyboard(keymap)
    }
}

impl ProgramCounter {
    fn increment(&mut self, n: u16) -> u16 {
        self.0 += n;
        self.0
    }
}

impl Stack {
    fn pop(&mut self, i: usize) -> u16 {
        self.0.remove(i)
    }
    fn push(&mut self, el: u16) {
        self.0.insert(0, el)
    }
}
