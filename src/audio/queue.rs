use std::{fs::File, path::PathBuf};

mod audio_player;

pub struct Queue {
    audio_player: audio_player::AudioPlayer,
    queue: Vec<Track>,
    current: usize,
}

pub enum Speed {
    Faster,
    Slower,
    Normal,
}

impl Queue {
    pub fn new(tracks: Vec<Track>) -> Self {
        Self {
            audio_player: audio_player::AudioPlayer::new().expect("Unable to create audio player"),
            queue: tracks,
            current: 0,
        }
    }

    pub fn add(&mut self, track: Track) {
        self.queue.push(track);
    }

    pub fn play(&mut self) {
        self.audio_player.clear_queue();
        let file = File::open(self.queue[self.current].path()).expect("Unable to read file");
        self.audio_player
            .add_track_to_queue(file)
            .expect("Unable to add track");
    }

    pub fn pause(&mut self) {
        self.audio_player.pause();
    }

    pub fn resume(&mut self) {
        self.audio_player.resume();
    }

    pub fn play_next(&mut self) {
        self.current += 1;
        self.play();
    }

    pub fn play_previous(&mut self) {
        self.current -= 1;
        self.play();
    }

    pub fn play_nth(&mut self, slot: usize) {
        self.current = slot;
        self.play();
    }

    pub fn current(&self) -> &String {
        self.queue[self.current].name()
    }

    pub fn get_tracks(&self) -> &Vec<Track> {
        &self.queue
    }

    pub fn get_queue_length(&self) -> usize {
        self.queue.len()
    }

    pub fn is_playing(&self) -> bool {
        !self.audio_player.is_paused()
    }

    pub fn speed(&self, speed: Speed) {
        match speed {
            Speed::Faster => self
                .audio_player
                .set_speed(self.audio_player.speed() + 0.025),
            Speed::Slower => self
                .audio_player
                .set_speed(self.audio_player.speed() - 0.025),
            Speed::Normal => self.audio_player.set_speed(1.0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Track {
    name: String,
    file_path: PathBuf,
}

impl Track {
    pub fn new(path: &PathBuf) -> Self {
        let name = path
            .file_name()
            .expect("Unable to read filename.")
            .to_str()
            .expect("Unable to convert filename tu UTF-8")
            .to_owned();

        Track {
            name,
            file_path: path.clone(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn path(&self) -> &PathBuf {
        &self.file_path
    }
}
