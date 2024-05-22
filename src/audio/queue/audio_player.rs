use rodio;
use std::{fs::File, io::BufReader, time::Duration};

pub struct AudioPlayer {
    stream: rodio::OutputStream,
    stream_handle: rodio::OutputStreamHandle,
    sink: rodio::Sink,
}

impl AudioPlayer {
    pub fn new() -> Result<Self, AudioPlayerError> {
        let (stream, stream_handle) = match rodio::OutputStream::try_default() {
            Ok(output_stream) => output_stream,
            Err(e) => return Err(AudioPlayerError::new(e.to_string())),
        };
        let sink = match rodio::Sink::try_new(&stream_handle) {
            Ok(sink) => sink,
            Err(e) => return Err(AudioPlayerError::new(e.to_string())),
        };

        Ok(AudioPlayer {
            stream,
            stream_handle,
            sink,
        })
    }

    pub fn volume(&self) -> f32 {
        self.sink.volume()
    }

    pub fn set_volume(&self, value: f32) {
        self.sink.set_volume(value);
    }

    pub fn add_track_to_queue(&self, file: File) -> Result<(), AudioPlayerError> {
        let dc = match rodio::Decoder::new(BufReader::new(file)) {
            Ok(dc) => dc,
            Err(e) => return Err(AudioPlayerError::new(e.to_string())),
        };
        self.sink.append(dc);

        Ok(())
    }

    pub fn resume(&self) {
        self.sink.play();
    }

    pub fn pause(&self) {
        self.sink.pause();
    }

    pub fn seek(&self, position: Duration) -> Result<(), AudioPlayerError> {
        match self.sink.try_seek(position) {
            Ok(_) => Ok(()),
            Err(e) => Err(AudioPlayerError::new(e.to_string())),
        }
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

#[derive(Debug)]
pub struct AudioPlayerError {
    message: String,
}

impl AudioPlayerError {
    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn new(message: String) -> Self {
        AudioPlayerError { message }
    }
}
