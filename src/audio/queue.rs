use std::{fs::File, path::PathBuf};

mod audio_player;

pub struct Queue {
    audio_player: audio_player::AudioPlayer,
    queue: Vec<Track>,
    current: usize,
}

impl Queue {
    pub fn new() -> Self {
        Self {
            audio_player: audio_player::AudioPlayer::new(),
            queue: Vec::new(),
            current: 0,
        }
    }

    pub fn add(&mut self, track: Track) {
        self.queue.push(track);
    }

    pub fn play(&mut self) {
        self.audio_player.clear_queue();
        let file = File::open(self.queue[self.current].path()).expect("Unable to read file");
        self.audio_player.add_track_to_queue(file);
    }

    pub fn pause(&mut self) {
        self.audio_player.pause();
    }

    pub fn play_next(&mut self) {
        self.current += 1;
        self.play();
    }

    pub fn play_previous(&mut self) {
        self.current -= 1;
        self.play();
    }

    pub fn current(&self) -> &String {
        self.queue[self.current].name()
    }
}

#[derive(Debug, Clone)]
pub struct Track {
    name: String,
    file_path: PathBuf,
}

impl Track {
    pub fn new(path: PathBuf) -> Self {
        let name = path
            .file_name()
            .expect("Unable to read filename.")
            .to_str()
            .expect("Unable to convert filename tu UTF-8")
            .to_owned();

        Track {
            name,
            file_path: path,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn path(&self) -> &PathBuf {
        &self.file_path
    }
}
