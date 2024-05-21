use std::{env, io::Error, path::PathBuf};

mod audio;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    let folder_path = args.get(1).unwrap();

    let files = jembo::get_music_files(PathBuf::from(folder_path));

    let mut queue = audio::queue::Queue::new();

    for file in files {
        queue.add(audio::queue::Track::new(file));
    }

    queue.play();
    println!("Now playing {}.", queue.current());

    std::thread::sleep(std::time::Duration::from_secs(30));

    queue.play_next();

    println!("Now playing {}.", queue.current());

    std::thread::sleep(std::time::Duration::from_secs(30));

    Ok(())
}
