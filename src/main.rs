use std::fs::File;
use std::error::Error;

extern crate gif;
use gif::{Encoder, Frame, Repeat};
use std::borrow::Cow;

use cca::*;

fn main() -> Result<(), Box<dyn Error>>{
    let (width, height) = (300, 300);
    let mut automaton = CyclicAutomaton::new(width as usize, height as usize, 4, 1);
    automaton.randomize(1234);
    let color_map = &[0x00, 0x00, 0xFF,  0xFF, 0, 0, 0xFF, 0xFF, 0x00 ];

    let mut image = File::create("examples/first.gif").unwrap();
    let mut encoder = Encoder::new(&mut image, width, height, color_map).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();
    for _ in 0..100 {
        let mut frame = Frame::default();
        frame.width = width;
        frame.height = height;
        frame.buffer = Cow::Borrowed(&automaton.grid);
        encoder.write_frame(&frame).unwrap();
        automaton.next_generation();
    }
    
    Ok(())
}
