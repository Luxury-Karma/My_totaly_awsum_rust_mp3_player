use std::fs::File;
use std::io::{BufReader, stdout};
use std::io;
use std::{fs, thread};
use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink};
use std::env;
use std::path::{Path, PathBuf};
use rand::seq::SliceRandom;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers,read};
use crossterm::event::KeyEventKind;

use crossterm::execute;

fn scan_directory(directory_path: &Path) -> Vec<PathBuf> {
    let mut data:Vec<_> = fs::read_dir(directory_path)
        .into_iter()
        .flatten()
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let path = e.path();
                path.file_name()
                    .and_then(|file_name| file_name.to_str().map(str::to_string))
            })
        })
        .filter(|file_name_str| file_name_str.ends_with(".mp3"))
        .collect();
    let mut full_path:Vec<PathBuf> = Vec::new() ;
    for e in data{
        full_path.push(directory_path.join(e))
    }
    return full_path;
}

fn random_music_order(files:Vec<PathBuf>) -> Vec<PathBuf> {
    let mut rng = rand::thread_rng();
    let mut shuffled_files:Vec<PathBuf> = files.clone();
    shuffled_files.shuffle(&mut rng);
    return shuffled_files;
}


fn key_dection_system(){
    match crossterm::event::read().unwrap()
    {
        Event::Key(KeyEvent
                   {
                       code: KeyCode::Char('h'),
                       kind : KeyEventKind::Press,
                       modifiers: KeyModifiers::CONTROL, ..
                   }) => execute!(stdout, print!("Hello world!")).unwrap(),
        _ => {}
    }
}

fn main() {
    // Find all the files in the directory to play
    let mut files:Vec<PathBuf> = scan_directory(Path::new("C:\\music"));
    //Shuffle
    files = random_music_order(files);

    // Get an output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();


    let mut i: usize = 0;
    let music_size:usize = files.len();
    while i < music_size{
        if let Ok(current_dir) = env::current_dir() {
            let path: PathBuf = current_dir.join(&files[i]);
            let file_open = File::open(path);

            // Load a sound from a file, using a path relative to Cargo.toml
            let file = BufReader::new(file_open.unwrap());

            // Decode that sound file into a source
            let source = Decoder::new(file).unwrap();

            sink.append(source);


        } else {
            eprintln!("Failed to get the current working directory");
        }
    }


    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    thread::sleep(Duration::from_secs(5));
}
