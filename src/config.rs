// There are better tools to do this, but I want to try to do it myself once
use std::default::Default;
use std::env;

use crate::CyclicAutomaton;

pub enum Config {
    GifRequest(GifConfig),
    HelpRequest,
}

pub static HELP_MESSAGE: &str =
    "<command> gif <fname> <width> <height> <seed> <num_states> <threshold> <generations>";

pub fn parse_args(mut args: env::Args) -> Result<Config, &'static str> {
    args.next();
    match args.next() {
        Some(ref s) => match &s[..] {
            "gif" => GifConfig::parse_args(args),
            _ => Ok(Config::HelpRequest),
        },
        None => return Ok(Config::HelpRequest),
    }
}

#[derive(Debug)]
pub struct GifConfig {
    pub fname: String,
    pub width: usize,
    pub height: usize,
    pub seed: u64,
    pub num_states: u8,
    pub threshold: usize,
    pub generations: u32,
}

impl GifConfig {
    fn parse_args(mut args: env::Args) -> Result<Config, &'static str> {
        let mut config = GifConfig::default();

        config.fname = args.next().expect("No filename!"); // TODO: Check if string is valid path
        config.width = args
            .next()
            .map_or(config.width, |s| s.parse().expect("Cannot parse width"));
        config.height = args
            .next()
            .map_or(config.height, |s| s.parse().expect("Cannot parse height"));
        config.seed = args
            .next()
            .map_or(config.seed, |s| s.parse().expect("Cannot parse seed"));
        config.num_states = args
            .next()
            .map_or(config.num_states, |s| s.parse().expect("Cannot parse num_states"));

        config.threshold = args
            .next()
            .map_or(config.threshold, |s| s.parse().expect("Cannot parse threshold"));
        config.generations = args
            .next()
            .map_or(config.generations, |s| s.parse().expect("Cannot parse generations"));

        Ok(Config::GifRequest(config))
    }

    pub fn get_automaton(&self) -> CyclicAutomaton
    {
        CyclicAutomaton::new(self.width, self.height, self.num_states, self.threshold)
    }
}

impl Default for GifConfig {
    fn default() -> Self {
        GifConfig {
            fname: String::from("default.gif"),
            width: 300,
            height: 300,
            seed: 1,
            num_states: 5,
            threshold: 1,
            generations: 50,
        }
    }
}
