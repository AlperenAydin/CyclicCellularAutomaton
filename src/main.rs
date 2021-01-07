use std::fs::File;
use std::error::Error;

extern crate gif;
use gif::{Encoder, Frame, Repeat};
use std::borrow::Cow;
use std::env;

use cca::*;

fn main() -> Result<(), Box<dyn Error>>{    
    cca::run(env::args())
}
