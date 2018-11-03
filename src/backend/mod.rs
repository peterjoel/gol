use crate::{AppAction, GameState};
use editor::{EditAction, Editor};
use grid::Grid;
use std::error;
use std::fmt::{self, Debug, Display, Formatter};

pub(crate) mod key_map;
pub(crate) mod terminal;

#[derive(Debug)]
pub struct Error {
    msg: String,
    cause: Option<Box<dyn error::Error>>,
}

impl Error {
    fn new(msg: String) -> Error {
        Error { msg, cause: None }
    }

    fn caused_by(msg: String, cause: Box<dyn error::Error>) -> Error {
        Error {
            msg,
            cause: Some(cause),
        }
    }
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.msg)?;
        Ok(())
    }
}

pub trait GameBackend {
    fn num_cols(&self) -> usize;

    fn num_rows(&self) -> usize;

    fn app_actions(&self, game_state: GameState) -> Box<dyn Iterator<Item = AppAction>>;

    fn edit_actions(&self) -> Box<dyn Iterator<Item = EditAction>>;

    fn draw_game(&self, grid: &Grid<u8>) -> Result<(), Error>;

    fn draw_editor(&self, editor: &Editor, grid: &Grid<u8>) -> Result<(), Error>;
}
