use ::grid::*;
use ::presets;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EditAction {
    MoveCursorBy { x: isize, y: isize },
    ToggleCell,
    Clear,
    AddPreset { index: u8 },
}

pub struct Editor {
    cursor: (usize, usize)
}

impl Editor {
    pub fn new() -> Editor {
        Editor { cursor: (0, 0) }
    }

    pub fn set_cursor(&mut self, x: usize, y: usize) {
        self.cursor = (x, y);
    }

    pub fn get_cursor(&self) -> (usize, usize) {
        self.cursor
    }

    pub fn edit<'a>(&'a mut self, grid: &'a mut Grid<u8>) -> EditSteps<'a> {
        EditSteps { editor: self, grid }
    }

    pub fn apply_action(&mut self, action: EditAction, grid: &mut Grid<u8>) {
        let mut edit_steps = self.edit(grid);
        match action {
            EditAction::Clear => edit_steps.clear_all(),
            EditAction::ToggleCell => edit_steps.toggle_current(),
            EditAction::MoveCursorBy { x, y } => edit_steps.move_cursor_by(x, y),
            EditAction::AddPreset { index } => edit_steps.add_preset(presets::get_preset(index)),
        } 
    }
}

pub struct EditSteps<'a> {
    editor: &'a mut Editor,
    grid: &'a mut Grid<u8>,
}

impl<'a> EditSteps<'a> {

    pub fn toggle_at(&mut self, x: usize, y: usize) {
        let val = if self.grid.get(x, y) == 0 {
            1
        } else {
            0
        };
        self.grid.set(x, y, val);
    }

    pub fn toggle_current(&mut self) {
        let (x, y) = self.editor.cursor;
        self.toggle_at(x, y);
    }

    pub fn add_preset(&mut self, cells: impl Iterator<Item = (usize, usize)>) {
        let (x, y) = self.editor.get_cursor();
        let (w, h) = (self.grid.width(), self.grid.height());
        for (i, j) in cells {
            self.grid.set((x + i + w) % w, (y + j + h) % h, 1);
        }
    }   

    pub fn clear_all(&mut self) {
        self.grid.set_all(0);
    }

    pub fn move_cursor_by(&mut self, by_x: isize, by_y: isize) {
        let (mut x, mut y) = self.editor.cursor;
        let (w, h) = (self.grid.width(), self.grid.height());
        x = (x as isize + by_x + w as isize ) as usize % w;
        y = (y as isize + by_y + h as isize ) as usize % h;
        self.editor.set_cursor(x, y);
    }

}
