use game_grid::*;

pub struct Gol {
    world: Game<u8>,
    wrapped: bool,
}

impl Gol {

    pub fn new(width: usize, height: usize, wrapped: bool) -> Gol {
        Gol { world: Game::new(width, height), wrapped }
    }

    pub fn init(&mut self, cells: &[(usize, usize)]) {
        for &(x, y) in cells {
            self.world.grid_mut().set(x, y, 1);
        }
    }

    pub fn grid(&self) -> &Grid<u8> {
        self.world.grid()
    }

    pub fn grid_mut(&mut self) -> &mut Grid<u8> {
        self.world.grid_mut()
    }

    pub fn next_turn(&mut self) {
        self.world.next_turn();

        let width = self.world.grid().width();
        let height = self.world.grid().height();

        for x in 0 .. width {
            for y in 0 .. height {
                let mut neighbours = [0; 8];
                let num_neighbours: u8 = {
                    let prev = self.world.old_grid();
                    if self.wrapped {
                        prev.get_neighbours_wrapped(x, y, &mut neighbours);
                    } else {
                        prev.get_neighbours(x, y, &mut neighbours);
                    }
                    neighbours.into_iter().sum()
                };
                if self.world.old_grid().get(x, y) == 1 {
                    if num_neighbours < 2 || num_neighbours > 3 {
                        self.world.grid_mut().set(x, y, 0);
                    } else {
                        self.world.grid_mut().set(x, y, 1);
                    }
                } else if num_neighbours == 3 {
                    self.world.grid_mut().set(x, y, 1);
                } else {
                    self.world.grid_mut().set(x, y, 0);
                }
            }
        }
    }
}
