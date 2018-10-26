use ::grid::*;

#[derive(Debug)]
pub struct Gol {
    grid: Game<u8>,
    wrapped: bool,
}

impl Gol {
    pub fn new(width: usize, height: usize, wrapped: bool) -> Gol {
        Gol { grid: Game::new(width, height), wrapped }
    }

    pub fn new_from_grid(grid: Grid<u8>, wrapped: bool) -> Gol {
        Gol { grid: grid.into(), wrapped }
    }

    pub fn init(&mut self, cells: &[(usize, usize)]) {
        for &(x, y) in cells {
            self.grid.grid_mut().set(x, y, 1);
        }
    }

    pub fn grid(&self) -> &Grid<u8> {
        self.grid.grid()
    }

    pub fn grid_mut(&mut self) -> &mut Grid<u8> {
        self.grid.grid_mut()
    }

    pub fn next_turn(&mut self) {
        self.grid.next_turn();

        let width = self.grid.grid().width();
        let height = self.grid.grid().height();

        for x in 0 .. width {
            for y in 0 .. height {
                let num_neighbours: u8 = {
                    let prev = self.grid.old_grid();
                    if self.wrapped {
                        prev.neighbours_wrapped(x, y).sum()
                    } else {
                        prev.neighbours(x, y).sum()
                    }
                };
                if self.grid.old_grid().get(x, y) == 1 {
                    if num_neighbours < 2 || num_neighbours > 3 {
                        self.grid.grid_mut().set(x, y, 0);
                    } else {
                        self.grid.grid_mut().set(x, y, 1);
                    }
                } else if num_neighbours == 3 {
                    self.grid.grid_mut().set(x, y, 1);
                } else {
                    self.grid.grid_mut().set(x, y, 0);
                }
            }
        }
    }
}
