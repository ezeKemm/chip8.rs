use std::collections::HashMap;
use rand::Rng;
use crate::render::Render;

mod render;
struct CPU {
    mem: Memory,
    pc: ProgramCounter,
    stack: Stack,
    I: u16,
    V: [u16; 16],
    font: Font,
    km: Keyboard,
    renderer: Render,
    delay: u16,
    sound: u16,
}

struct Font([i32; 80]);
// struct Address(usize);
// struct VarReg(HashMap<Address, usize>);
// struct VarReg([u16; 16]);
struct ProgramCounter(u16);
// struct IndexRegister(u16);
struct Memory([u8; 4096]);
struct Stack(Vec<u16>);

impl CPU {
    fn init(renderer: Render) -> Self {
        let mem = Memory([0; 4096]);
        let pc = ProgramCounter(0);
        let stack = Stack(vec![]);
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
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F 
        ]);
        let km = Keyboard::default();
        let I = 0 as u16;
        let V = [0 as u16; 16];
        let sound = 0;
        let delay = 0;
        CPU { mem, pc, stack, I, V, font, km, renderer, delay, sound }
    }

    fn frontload_font(&mut self) {
        let start: usize = 0x0;
        for i in 0..self.font.0.len() {
            self.mem.0[start + i] = self.font.0[i] as u8;
        }
    }
}
fn main() {
    let mut renderer = Render::default();
    let mut cpu = CPU::init(renderer);
    cpu.frontload_font();
    loop {
        let opcode = cpu.fetch();
        let _ = cpu.execute(opcode);
    }
}

impl CPU {

    fn fetch(&mut self) -> u16 {
        let i = self.pc.0 as usize;
        let opcode = self.mem.0[i] << 8 | self.mem.0[i+1];
        self.pc.increment(2);
        opcode as u16
    }
    fn execute(&mut self, opcode: u16) {

        let x = (opcode & 0x0F00 >> 8) as usize;  // the 2nd nibble
        let y = (opcode & 0x00F0 >> 4) as usize;  // the 3rd nibble
    
        match opcode & 0xF000 {
            0x00E0 => { self.renderer.clear() }, // CLS -- clear screen
            0x00EE => { self.pc.0 = self.stack.pop(0) as u16 }, // RET
            0x0000 => {}, // SYS // IGNORE 
            0x1000 => { self.pc.0 = opcode & 0xFFF }, // JMP 
            0x2000 => {    // CALL
                self.stack.push(self.pc.0);
                self.pc.0 = opcode & 0xFFF
            },
            0x3000 => {    // SE Vx, byte  
                if self.V[x] == 0xFF {
                    self.pc.increment(2);
                }
            },  
            0x4000 => {    // SNE Vx, byte
                if self.V[x] != 0xFF {
                    self.pc.increment(2);
                }
            },
            0x5000 => {    // SE Vx, Vy
                if self.V[x] == self.V[y] {
                    self.pc.increment(2);
                }
            },
            0x6000 => {    // LD Vx, byte
                self.V[x] = opcode & 0xFF;
            },
            0x7000 => {    // ADD Vx, byte
                self.V[x] += opcode & 0xFF;
            },
            0x8000 => {
                match opcode & 0xF {
                    0x0 => { self.V[x] = self.V[y] }, // LD 
                    0x1 => { self.V[x] = self.V[x] | self.V[y] }, // OR 
                    0x2 => { self.V[x] = self.V[x] & self.V[y] }, // AND
                    0x3 => { self.V[x] = self.V[x] ^ self.V[y] }, // XOR
                    0x4 => {    // ADD Vx, Vy 
                        self.V[0xF] = 0;
                        let sum = self.V[x] + self.V[y];
                        if sum > 0xFF {  // if Vx + Vy > 255
                            self.V[0xF] = 1;
                        }
                        self.V[x] = (sum & 0xFF) >> 8;  // get the lowest 8 bits
                    },
                    0x5 => {    // SUB Vx, Vy
                        self.V[0xF] = 0;
                        let sum = self.V[y] - self.V[x];
                        if self.V[x] > self.V[y] {
                            self.V[0xF] = 1;
                        }
                        self.V[x] = sum;
                    },
                    0x6 => {
                        self.V[0xF] = 0;
                        if self.V[y] & 0xF == 1 {
                            self.V[0xF] = 1;
                        }
                        self.V[x] /= 2;
                    }, // SHR
                    0x7 => {
                        self.V[0xF] = 0;
                        if self.V[y] > self.V[x] {
                            self.V[0xF] = 1;
                        }
                        self.V[x] = self.V[y] - self.V[x];
                    }, // SUBN 
                    0xE => {
                        self.V[0xF] = 0;
                        if self.V[x] & 0xF000 == 1 {
                            self.V[0xF] = 1;
                        }
                        self.V[x] *= 2;
                    }, // SHL
                    _ => {}
                } 
            },
            0x9000 => {
                if self.V[x] != self.V[y] {
                    self.pc.increment(2);
                }
            }, // SNE 
            0xA000 => {
                self.I = opcode & 0xFFF;
            }, // LD 
            0xB000 => {
                self.pc.0 = (opcode & 0xFFF) + self.V[0];
            }, // JMP 
            0xC000 => {
                let mut rng = rand::thread_rng();
                let r: u16 = rng.gen_range(0..255);
                self.V[x] = r & (opcode & 0xFF);
             }, // RND 
            0xD000 => {

            }, // DRW 
            0xE000 => {
                match opcode & 0xFF {
                    0x9E => {

                    }, // SKP 
                    0xA1 => {}, // SKNP 
                    _ => {}
                }
            }, 
            0xF000 => {
                match opcode & 0xFF {
                    0x07 => {}, // LD  
                    0x0A => {}, // LD 
                    0x15 => {}, // LD 
                    0x18 => {}, // LD 
                    0x1E => {}, // ADD 
                    0x29 => {}, // LD 
                    0x33 => {}, // LD 
                    0x55 => {}, // LD 
                    0x65 => {}, // LD 
                    _ => {}
                }
            }, 
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
        keymap.insert(82, 0xD);
        keymap.insert(65, 0x7);
        keymap.insert(83, 0x8);
        keymap.insert(68, 0x9);
        keymap.insert(70, 0xE);
        keymap.insert(90, 0xA);
        keymap.insert(88, 0x0);
        keymap.insert(67, 0xB);
        keymap.insert(86, 0xF);
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
