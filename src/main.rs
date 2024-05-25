use std::{
    env,
    io::{stdout, Error},
    path::PathBuf,
};

use audio::queue::{Queue, Speed, Track};
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use jembo::stringify_paths;
use ratatui::{
    prelude::{Color, CrosstermBackend, Modifier, Style, Terminal},
    widgets::{Block, List, ListState, StatefulWidget},
    Frame,
};

mod audio;

struct App {
    queue: Queue,
    list_state: ListState,
}

impl App {
    fn new(files: Vec<PathBuf>) -> Self {
        let tracks: Vec<Track> = files.iter().map(|file| Track::new(file)).collect();
        let queue = Queue::new(tracks);
        let mut new_app = App {
            queue,
            list_state: ListState::default(),
        };
        new_app.list_state.select(Some(0));

        new_app
    }
}

fn view(model: &mut App, frame: &mut Frame) {
    let names: Vec<String> = model
        .queue
        .get_tracks()
        .iter()
        .map(|track| track.name().to_owned())
        .collect();
    frame.render_stateful_widget(
        List::new(names)
            .block(Block::bordered().title("Your music"))
            .style(Style::default().fg(Color::Yellow))
            .highlight_style(Style::default().add_modifier(Modifier::SLOW_BLINK))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true),
        frame.size(),
        &mut model.list_state,
    )
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    let folder_path = args.get(1).unwrap();

    let path_files = jembo::get_music_files(PathBuf::from(folder_path));

    let mut app = App::new(path_files);

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    loop {
        terminal.draw(|frame| {
            view(&mut app, frame);
        })?;

        if event::poll(std::time::Duration::from_millis(5))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    let current = app
                        .list_state
                        .selected()
                        .expect(" something must be selected");
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char(' ') => {
                            app.queue.speed(Speed::Normal);
                            app.queue.play_nth(current);
                        }
                        KeyCode::Down => {
                            if app.queue.is_playing() {
                                app.queue.pause();
                            }
                            if current < app.queue.get_queue_length() - 1 {
                                app.list_state.select(Some(current + 1))
                            }
                        }
                        KeyCode::Up => {
                            if app.queue.is_playing() {
                                app.queue.pause();
                            }
                            if current > 0 {
                                app.list_state.select(Some(current - 1))
                            }
                        }
                        KeyCode::Left => {
                            app.queue.speed(Speed::Slower);
                        }
                        KeyCode::Right => app.queue.speed(Speed::Faster),
                        KeyCode::Esc => app.queue.speed(Speed::Normal),
                        _ => (),
                    };
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
