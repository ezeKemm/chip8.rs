use crate::{Font, Memory, ProgramCounter, Stack};
use rand::Rng;

#[derive(Debug)]
pub struct CPU {
    mem: Memory,
    vram: [u8; 2048],
    pc: ProgramCounter,
    stack: Stack,
    I: u16,
    V: [u16; 16],
    delay: u16,
    sound: u16,
}

impl Default for CPU {
    fn default() -> Self {
        CPU {
            mem: Memory::default(),
            vram: [0; 2048],
            pc: ProgramCounter::default(),
            stack: Stack::default(),
            I: 0,
            V: [0; 16],
            delay: 0,
            sound: 0,
        }
    }
}
impl CPU {
    pub fn init(font: Font) -> Self {
        CPU::default().frontload_font(font)
    }

    fn frontload_font(mut self, font: Font) -> Self {
        let start: usize = 0x0;
        for i in 0..font.0.len() {
            self.mem.0[start + i] = font.0[i] as u8;
        }
        self
    }
}

impl CPU {
    pub fn fetch(&mut self) -> u16 {
        let i = self.pc.0 as usize;
        let opcode = self.mem.0[i] << 8 | self.mem.0[i + 1];
        self.pc.increment(2);
        opcode as u16
    }
    pub fn execute(&mut self, opcode: u16) {
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
