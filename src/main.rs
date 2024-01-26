use std::collections::HashMap;

mod cpu;
mod font;
mod render;
mod ui;

use crate::ui::{Audio, Input, Renderer};
use cpu::CPU;
use font::DEF_FONT;

const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;
const SCALE: u32 = 10;

struct Vec2 {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct Font([i32; 80]);
#[derive(Debug, Default)]
struct ProgramCounter(u16);
#[derive(Debug)]
struct Memory([u8; 4096]);
#[derive(Debug, Default)]
struct Stack(Vec<u16>);

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
    let mut cpu = CPU::init(Font::default());
    loop {
        let opcode = cpu.fetch();
        let _ = cpu.execute(opcode);
    }
}

impl Default for Memory {
    fn default() -> Self {
        Memory([0; 4096])
    }
}

impl Default for Font {
    fn default() -> Self {
        Font(DEF_FONT)
    }
}
impl Vec2 {
    fn prod(&self) -> usize {
        (self.width * self.height) as usize
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
