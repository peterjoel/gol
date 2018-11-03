use backend::key_map::*;
use backend::{Error, GameBackend};
use crate::{AppAction, GameState};
use editor::{EditAction, Editor};
use grid::Grid;
use rustty::*;
use std::cell::RefCell;
use std::time::Duration;

pub struct Term {
    term: RefCell<Terminal>,
    key_presses: RefCell<Vec<char>>,
}

impl Term {
    pub fn new() -> Term {
        Term {
            term: RefCell::new(Terminal::new().unwrap()),
            key_presses: RefCell::new(Vec::new()),
        }
    }

    fn render_game(&self, grid: &Grid<u8>) {
        let width = grid.width();
        let height = grid.height();
        for x in 0..width {
            for y in 0..height {
                let col = if grid.get(x, y) == 1 {
                    Color::Red
                } else {
                    Color::Black
                };
                self.term.borrow_mut()[(x, y)].set_bg(col);
            }
        }
    }

    fn render_editor(&self, editor: &Editor, grid: &Grid<u8>) {
        self.render_game(grid);
        let (x, y) = editor.get_cursor();
        let col = if grid.get(x, y) == 1 {
            Color::Green
        } else {
            Color::White
        };
        self.term.borrow_mut()[(x, y)].set_bg(col);
    }

    fn filter_remove_actions<F, T>(&self, filter: F) -> Box<dyn Iterator<Item = T>>
    where
        F: Fn(char) -> Option<T> + 'static,
        T: Copy + 'static,
    {
        let mut presses = self.key_presses.borrow_mut();
        let presses: Vec<_> = presses.drain_filter(|key| filter(*key).is_some()).collect();

        if presses.is_empty() {
            let press = self
                .term
                .borrow_mut()
                .get_event(Duration::from_millis(20))
                .ok();
            Box::new(
                press
                    .into_iter()
                    .flatten()
                    .flat_map(move |Event::Key(k)| filter(k)),
            )
        } else {
            Box::new(presses.into_iter().flat_map(filter))
        }
    }
}

impl GameBackend for Term {
    fn num_cols(&self) -> usize {
        self.term.borrow().cols()
    }

    fn num_rows(&self) -> usize {
        self.term.borrow().rows()
    }

    fn app_actions(&self, game_state: GameState) -> Box<dyn Iterator<Item = AppAction>> {
        self.filter_remove_actions(move |key| map_key_to_global_action(game_state, key))
    }

    fn edit_actions(&self) -> Box<dyn Iterator<Item = EditAction>> {
        self.filter_remove_actions(map_key_to_edit_action)
    }

    fn draw_game(&self, grid: &Grid<u8>) -> Result<(), Error> {
        self.render_game(grid);
        self.term
            .borrow_mut()
            .swap_buffers()
            .map_err(|err| Error::caused_by("Error drawing game".to_owned(), Box::new(err)))
    }

    fn draw_editor(&self, editor: &Editor, grid: &Grid<u8>) -> Result<(), Error> {
        self.render_editor(editor, grid);
        self.term
            .borrow_mut()
            .swap_buffers()
            .map_err(|err| Error::caused_by("Error drawing editor".to_owned(), Box::new(err)))
    }
}
