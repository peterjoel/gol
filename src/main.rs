#![feature(drain_filter)]

extern crate rustty;

use std::sync::mpsc::{channel, Receiver, SendError};
use std::sync::{Arc, Mutex};

mod backend;
mod editor;
mod game;
mod grid;
mod presets;
mod runner;

use backend::terminal::Term;
use backend::GameBackend;
use editor::{EditAction, Editor};
use game::Gol;
use runner::Runner;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AppAction {
    Quit,
    TogglePause,
    EditMode,
    EditDone,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    Running,
    Paused,
    Editing,
}

impl GameState {
    fn toggle_paused(self) -> GameState {
        match self {
            GameState::Paused => GameState::Running,
            GameState::Running => GameState::Paused,
            s => s,
        }
    }
}

#[derive(Debug)]
enum Error {
    Runner(runner::Error),
    SendEditAction(SendError<editor::EditAction>),
    UI(backend::Error),
}

impl From<backend::Error> for Error {
    fn from(other: backend::Error) -> Error {
        Error::UI(other)
    }
}

impl From<runner::Error> for Error {
    fn from(other: runner::Error) -> Error {
        Error::Runner(other)
    }
}

impl From<SendError<editor::EditAction>> for Error {
    fn from(other: SendError<editor::EditAction>) -> Error {
        Error::SendEditAction(other)
    }
}

fn main() -> Result<(), Error> {
    let mut state = GameState::Paused;
    // TODO: handle resizing the terminal window
    let mut ui = Term::new();
    let game = Arc::new(Mutex::new(Gol::new(ui.num_cols(), ui.num_rows(), true)));
    let editor = Arc::new(Mutex::new(Editor::new()));
    let (edit_actions, editor_recv) = channel();

    let game_runner = run_game(Arc::clone(&game));
    let editor_runner = run_editor(Arc::clone(&game), Arc::clone(&editor), editor_recv);

    loop {
        let mut new_state = state;
        for action in ui.app_actions(state) {
            new_state = match action {
                AppAction::Quit => {
                    editor_runner.finish()?;
                    game_runner.finish()?;
                    return Ok(());
                }
                AppAction::EditDone => GameState::Paused,
                AppAction::TogglePause => state.toggle_paused(),
                AppAction::EditMode => GameState::Editing,
            };
        }
        if state == GameState::Editing {
            for action in ui.edit_actions() {
                edit_actions.send(action)?;
            }
        }

        if new_state != state {
            match state {
                GameState::Editing => {
                    editor_runner.pause()?;
                }
                GameState::Running => {
                    game_runner.pause()?;
                }
                _ => (),
            }
            state = new_state;
            match state {
                GameState::Editing => {
                    editor_runner.start()?;
                }
                GameState::Running => {
                    game_runner.start()?;
                }
                _ => (),
            }
        }

        draw_current_state(state, &Arc::clone(&game), &Arc::clone(&editor), &mut ui)?;
    }
}

fn run_editor(
    game: Arc<Mutex<Gol>>,
    editor: Arc<Mutex<Editor>>,
    recv: Receiver<EditAction>,
) -> Runner {
    Runner::new(move || {
        // await action before locking anything else
        if let Ok(action) = recv.recv() {
            let mut game = game.lock().unwrap();
            let grid = game.grid_mut();
            let mut editor = editor.lock().unwrap();
            editor.apply_action(action, grid);
        }
    })
}

fn run_game(game: Arc<Mutex<Gol>>) -> Runner {
    Runner::new(move || {
        let mut game = game.lock().unwrap();
        game.next_turn();
    })
}

fn draw_current_state<Ui: GameBackend>(
    state: GameState,
    game: &Mutex<Gol>,
    editor: &Mutex<Editor>,
    ui: &mut Ui,
) -> Result<(), Error> {
    match state {
        GameState::Running => {
            let game = game.lock().unwrap();
            ui.draw_game(game.grid())?;
        }
        GameState::Editing => {
            let game = game.lock().unwrap();
            let editor = editor.lock().unwrap();
            ui.draw_editor(&editor, game.grid())?;
        }
        _ => {}
    }
    Ok(())
}
