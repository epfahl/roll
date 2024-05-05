use clap::Parser;
use rand::Rng;
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
struct Die {
    sides: u64,
}

impl Die {
    fn roll(&self) -> u64 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..(self.sides + 1))
    }
}

impl FromStr for Die {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s_lower = s.to_lowercase();
        let mut chars = s_lower.chars();
        let err_string = format!("What kind of die is '{s}'? Try d10 or d20 or similar.");

        if let Some('d') = chars.next() {
            let sides = chars.as_str().parse::<u64>().map_err(|_| err_string)?;

            Ok(Self { sides })
        } else {
            Err(err_string)
        }
    }
}

#[derive(Debug, Parser)]
struct Args {
    #[arg(value_parser = clap::value_parser!(Die))]
    die: Die,
}

fn play_audio() {
    // Copied from rodio crate docs
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open("audio/dice-142528.mp3").unwrap());
    let source = Decoder::new(file).unwrap();
    let _ = stream_handle.play_raw(source.convert_samples());
    std::thread::sleep(std::time::Duration::from_millis(1100));
}

fn main() {
    let args = Args::parse();
    let _ = play_audio();
    println!("You rolled {}!", args.die.roll())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn die_parser() {
        assert_eq!(Ok(Die { sides: 10 }), Die::from_str("d10"));
        assert!(Die::from_str("f22").is_err());
    }
}
