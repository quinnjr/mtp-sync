#[macro_use] extern crate serde_derive;
extern crate plist;

use std::fs::File;
use std::io::BufReader;

use anyhow::Error;
use directories::{UserDirs};
use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

pub mod track;
pub mod library;
pub mod playlist;

use crate::library::Library;

fn main() -> Result<(), Error> {
    if let Some(user_dirs) = UserDirs::new() {
        let audio_dir = match user_dirs.audio_dir() {
            Some(dir) => { dir }
            None => { panic!("Failed to locate audio directory"); }
        };

        let itunes_xml = audio_dir.join("iTunes\\iTunes Music Library.xml");

        let fp = File::open(itunes_xml)?;
        let fp = BufReader::new(fp);

        let plist: Library = plist::from_reader_xml(fp)?;
    };

    // let app = Application::builder()
    //     .application_id("tech.quinnjr.mtp-sync")
    //     .build();

    //     app.connect_activate(|app| {
    //         let window = ApplicationWindow::builder()
    //             .application(app)
    //             .default_width(1280)
    //             .default_height(720)
    //             .title("MTP-Sync")
    //             .build();

    //         window.show();
    //     });

    // app.run();

    return Ok(())
}
