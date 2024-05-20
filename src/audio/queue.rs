use std::{fs::File, time::Duration};

mod audio_player;

pub struct Queue {
    audio_player: audio_player::AudioPlayer,
    queue: Vec<audio_player::Track>,
    current: usize,
}
