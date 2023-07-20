use std::collections::HashMap;

use crate::render::Render;

mod render;

fn main() {
    // let mut mem = Memory::default();
    let mut pc = ProgramCounter(0);
    let mut stack = Stack(vec![]);
    loop {
        fetch(&pc)
    }
}

struct ProgramCounter(isize);
struct IndexRegister(u16);
#[derive(Debug, Default)]
struct Display(Vec<Pixel>);

#[derive(Debug, Default, Clone)]
struct Pixel {
    p: isize,
}
// #[derive(Default)]
struct Memory([isize; 4096]);
struct Stack(Vec<isize>);

fn fetch(pc: &ProgramCounter) {}

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
