use rodio;
use std::{fs::File, io::BufReader, path::PathBuf, time::Duration};

pub struct AudioPlayer {
    stream: rodio::OutputStream,
    stream_handle: rodio::OutputStreamHandle,
    sink: rodio::Sink,
}

impl AudioPlayer {
    pub fn new() -> Self {
        let (_stream, handle) =
            rodio::OutputStream::try_default().expect("Impossible to create audio player.");
        let sink = rodio::Sink::try_new(&handle).expect("Impossible to create audio player");
        AudioPlayer {
            stream: _stream,
            stream_handle: handle,
            sink: sink,
        }
    }

    pub fn volume(&self) -> f32 {
        self.sink.volume()
    }

    pub fn set_volume(&self, value: f32) {
        self.sink.set_volume(value);
    }

    pub fn add_track_to_queue(&self, file: File) {
        self.sink.append(
            rodio::Decoder::new(BufReader::new(file)).expect("Impossible to decode the file."),
        );
    }

    pub fn resume(&self) {
        self.sink.play();
    }

    pub fn pause(&self) {
        self.sink.pause();
    }

    pub fn seek(&self, position: Duration) {
        self.sink
            .try_seek(position)
            .expect("Unable to seek to this position.");
    }

    pub fn clear_queue(&self) {
        self.sink.clear();
        self.sink.play();
    }

    pub fn is_empty(&self) -> bool {
        self.sink.empty()
    }

    pub fn is_paused(&self) -> bool {
        self.sink.is_paused()
    }

    pub fn go_next(&self) {
        self.sink.skip_one();
    }
}

pub struct AudioPlayerError {
    message: String,
}

impl AudioPlayerError {
    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn new(message: &str) -> Self {
        AudioPlayerError {
            message: message.to_string(),
        }
    }
}
