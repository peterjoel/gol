extern crate rustty;

use std::fmt::Debug;
use std::time::Duration;
use std::sync::{Mutex, Arc};
use std::sync::mpsc::{channel, Sender, Receiver, RecvError, TryRecvError};

mod grid;
mod game;   
mod editor;
mod presets;
mod runner;

use grid::Grid;
use game::Gol;
use editor::{Editor, EditAction};
use runner::Runner;

#[derive(Copy, Clone, Debug, PartialEq)]
enum AppAction {
    Quit,
    TogglePause,
    EditMode,
    EditDone,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum GameState {
    Running,
    Paused,
    Editing,
}

fn main() {
    let mut state = GameState::Paused;
    // TODO: handle resizing the terminal window
    let mut term = rustty::Terminal::new().unwrap();
    let game = Arc::new(Mutex::new(
        Gol::new(term.cols(), term.rows(), true)
    ));
    let editor = Arc::new(Mutex::new(Editor::new()));
    let (edit_actions, editor_recv) = channel();

    let game_runner = run_game(Arc::clone(&game));
    let editor_runner = run_editor(Arc::clone(&game), Arc::clone(&editor), editor_recv);

    loop {
        if let Some(rustty::Event::Key(key)) = term.get_event(Duration::from_millis(50)).unwrap() {
            let new_state = if let Some(action) = map_key_to_global_action(&state, key) {
                match action {
                    AppAction::Quit => { 
                        editor_runner.finish();
                        game_runner.finish();
                        return; 
                    },
                    AppAction::EditDone => GameState::Paused,
                    AppAction::TogglePause => {
                        if let GameState::Paused = state {
                            GameState::Running
                        } else if let GameState::Running = state {
                            GameState::Paused
                        } else {
                            state
                        }
                    },
                    AppAction::EditMode => {
                        GameState::Editing
                    },
                    AppAction::EditDone => {
                        GameState::Paused
                    },
                }
            } else {
                if state == GameState::Editing {
                    for action in map_key_to_edit_action(key) {
                        edit_actions.send(action);
                    }
                }
                state
            };

            if new_state != state {
                match state {
                    GameState::Editing => { editor_runner.pause() },
                    GameState::Running => { game_runner.pause() },
                    _ => (),
                }
                state = new_state;
                match state {
                    GameState::Editing => { editor_runner.start() },
                    GameState::Running => { game_runner.start() },
                    _ => (),
                }
            }
        }

        draw_current_state(state, Arc::clone(&game), Arc::clone(&editor), &mut term);
    }
}

fn run_editor(game: Arc<Mutex<Gol>>, editor: Arc<Mutex<Editor>>, recv: Receiver<EditAction>) -> Runner {
    Runner::new(move || {
        // await action before locking anything else
        let action = recv.recv().ok();
        let mut game = game.lock().unwrap();
        let grid = game.grid_mut();
        let mut editor = editor.lock().unwrap();
        action.map(|action| editor.apply_action(action, grid)); 
    })
}

fn run_game(game: Arc<Mutex<Gol>>) -> Runner {
    Runner::new(move || {
        let mut game = game.lock().unwrap();
        game.next_turn();
    })
}

fn map_key_to_global_action(state: &GameState, key: char) -> Option<AppAction> {
    match key {
        'q' => return Some(AppAction::Quit),
        _ => (),
    }
    match state {
        GameState::Editing => {
            match key {
                '\r' => Some(AppAction::EditDone),
                _ => None,
            }
        },
        // Paused or Running:
        _ => {
            match key {
                '\r' => Some(AppAction::TogglePause),
                'e' => Some(AppAction::EditMode),
                _ => None,
            }
        },
    }
}

fn map_key_to_edit_action(key: char) -> Option<EditAction> {
    match key {
        'c' => Some(EditAction::Clear),
        'i' => Some(EditAction::MoveCursorBy { x: 0, y: -1 }),
        'k' => Some(EditAction::MoveCursorBy { x: 0, y: 1 }),
        'j' => Some(EditAction::MoveCursorBy { x: -1, y: 0 }),
        'l' => Some(EditAction::MoveCursorBy { x: 1, y: 0 }),
        ' ' => Some(EditAction::ToggleCell),
        n if n.is_digit(10) => Some(EditAction::AddPreset { index: n.to_string().parse().unwrap() }),
        _ => None,
    }
}

fn draw_current_state(state: GameState, game: Arc<Mutex<Gol>>, editor: Arc<Mutex<Editor>>, term: &mut rustty::Terminal) {
    match state {
        GameState::Running => {
            let game = game.lock().unwrap();
            draw_game(term, game.grid());
        },
        GameState::Editing => {
            let game = game.lock().unwrap();
            let editor = editor.lock().unwrap();
            draw_editor(term, &editor, game.grid());
        },
        _ => { },
    }
    term.swap_buffers().unwrap();   
}

fn draw_game(term: &mut rustty::Terminal, grid: &Grid<u8>) {
    let width = grid.width();
    let height = grid.height();
    for x in 0 .. width {
        for y in 0 .. height {
            if grid.get(x, y) == 1 {
                term[(x, y)].set_bg(rustty::Color::Red);
            } else {
                term[(x, y)].set_bg(rustty::Color::Black);
            }
        }
    }
}

fn draw_editor(term: &mut rustty::Terminal, editor: &Editor, grid: &Grid<u8>) {
    draw_game(term, grid);
    let (x, y) = editor.get_cursor();
    if grid.get(x, y) == 1 {
        term[(x, y)].set_bg(rustty::Color::Green);
    } else {
        term[(x, y)].set_bg(rustty::Color::White);
    }
}
