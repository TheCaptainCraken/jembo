use std::io::BufReader;
use std::{env, fs, io::Error, path::PathBuf};

mod audio;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    let folder_path = args.get(1).unwrap();

    let files = jembo::get_music_files(PathBuf::from(folder_path));

    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let file1 = std::fs::File::open(files[1].clone()).unwrap();
    let file2 = std::fs::File::open(files[1].clone()).unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file1)).unwrap());
    sink.append(rodio::Decoder::new(BufReader::new(file2)).unwrap());
    std::thread::sleep(std::time::Duration::from_secs(5));
    sink.pause();
    std::thread::sleep(std::time::Duration::from_secs(5));
    sink.play();
    sink.sleep_until_end();

    Ok(())
}
