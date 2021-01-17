// Re-arraning the modules
mod cyclic_automaton;
pub use cyclic_automaton::*;

mod config;
pub use config::*;

mod colormap;
pub use colormap::*;

use std::env; 
use std::error::Error;
use std::fs::File;

use gif::{Encoder, Frame, Repeat};
use std::borrow::Cow;

pub fn run(mut args: env::Args) -> Result<(), Box<dyn Error>>
{
    let config = config::parse_args(args).unwrap();
    match config
    {
        Config::HelpRequest => {
            println!("{}", config::HELP_MESSAGE);
            Ok(())
        }
        Config::GifRequest(c) => run_gif(c)
    }
}

pub fn run_gif(c: GifConfig) -> Result<(), Box<dyn Error>>
{
    let mut automaton = c.get_automaton();
    automaton.randomize(1234);
    let color_map = get_colormap(c.num_states);

    let mut image = File::create(&c.fname).unwrap();
    let mut encoder = Encoder::new(&mut image, c.width as u16, c.height as u16, &color_map).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();
    for _ in 0..c.generations {
        let mut frame = Frame::default();
        get_frame(&automaton, &mut frame);
        encoder.write_frame(&frame).unwrap();
        automaton.next_generation();
    }
    println!("{:?}", c);
    Ok(())
}

pub fn get_frame<'a>(automaton: &'a CyclicAutomaton, frame: &mut Frame<'a>)
{
    frame.width = automaton.width as u16;
    frame.height = automaton.height as u16;
    frame.buffer = Cow::Borrowed(&automaton.grid);
}