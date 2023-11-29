use std::fs::File;
use std::io::BufReader;
use std::{fs, thread};
use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink};
use std::env;
use std::path::{Path, PathBuf};
use rand::seq::SliceRandom;

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

fn main() {
    // Find all the files in the directory to play
    let mut files:Vec<PathBuf> = scan_directory(Path::new("C:\\music"));
    //Shuffle
    files = random_music_order(files);

    // Get an output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    for e in files{
        if let Ok(current_dir) = env::current_dir() {
            let path: PathBuf = current_dir.join(e);
            let file_open = File::open(path);

            // Load a sound from a file, using a path relative to Cargo.toml
            let file = BufReader::new(file_open.unwrap());

            // Decode that sound file into a source
            let source = Decoder::new(file).unwrap();

            sink.append(source);

            // Sleep for 2 seconds
            thread::sleep(Duration::from_secs(2));
            sink.skip_one()

        } else {
            eprintln!("Failed to get the current working directory");
        }

    }





    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    thread::sleep(Duration::from_secs(5));
}
