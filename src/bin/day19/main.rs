mod point_cloud;
mod rotation;
mod vector;

use std::fs::File;
use std::io::BufRead;
use std::{env, error, io};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let _point_clouds = point_cloud::from_lines(
            io::BufReader::new(File::open(path)?)
                .lines()
                .map(|line| line.unwrap()),
        );

        Ok(())
    } else {
        Err("Usage: day19 INPUT_FILE_PATH".into())
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
