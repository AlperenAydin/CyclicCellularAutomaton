extern crate gif;

use std::env;
use std::error::Error;

use cca::run;

fn main() -> Result<(), Box<dyn Error>>{    
    run(env::args())
}
