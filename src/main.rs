#[macro_use]
extern crate serde_derive;
extern crate plist;

use std::fs::File;
use std::io::BufReader;

use anyhow::Error;
use directories::{UserDirs};

fn main() -> Result<(), Error> {
    if let Some(user_dirs) = UserDirs::new() {
        let audio_dir = match user_dirs.audio_dir() {
            Some(dir) => { dir }
            None => { panic!("Failed to locate audio directory"); }
        };

        let itunes_xml = audio_dir.join("iTunes\\iTunes Music Library.xml");

        let fp = File::open(itunes_xml)?;
        let fp = BufReader::new(fp);

        let plist = plist::from_reader_xml(fp)?;

        print!("{:?}", plist);
    };

    return Ok(())
}
