mod point_cloud;
mod rotation;
mod vector;

use std::{env, error, io};
use std::fs::File;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let _lines = io::BufReader::new(File::open(path)?).lines();
        let _input_string = std::fs::read_to_string(path)?;

        Ok(())
    } else {
        Err("Usage: day19 INPUT_FILE_PATH".into())
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
