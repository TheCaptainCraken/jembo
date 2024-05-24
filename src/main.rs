use std::{
    env,
    io::{stdout, Error},
    path::PathBuf,
};

use audio::queue::{Queue, Track};
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
    tracks: Vec<Track>,
    queue: Queue,
    current: usize,
    list_state: ListState,
}

impl App {
    fn new(files: Vec<PathBuf>) -> Self {
        let tracks: Vec<Track> = files.iter().map(|file| Track::new(file)).collect();
        let queue = Queue::new(tracks.clone());
        let mut new_app = App {
            tracks,
            queue,
            current: 0,
            list_state: ListState::default(),
        };
        new_app.list_state.select(Some(0));

        new_app
    }
}

fn view(model: &mut App, frame: &mut Frame) {
    let names: Vec<String> = model
        .tracks
        .iter()
        .map(|track| track.name().to_owned())
        .collect();
    frame.render_stateful_widget(
        List::new(names)
            .block(Block::bordered().title("List"))
            .style(Style::default().fg(Color::LightCyan))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
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

        if event::poll(std::time::Duration::from_millis(8))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char(' ') => app.queue.play_nth(match app.list_state.selected() {
                            Some(nth) => nth,
                            None => 1,
                        }),
                        KeyCode::Down => app.list_state.select(Some(
                            app.list_state
                                .selected()
                                .expect("something must be selected")
                                + 1,
                        )),
                        KeyCode::Up => app.list_state.select(Some(
                            app.list_state
                                .selected()
                                .expect("something must be selected")
                                - 1,
                        )),
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
