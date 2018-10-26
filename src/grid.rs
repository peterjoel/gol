use std::mem;

#[derive(Clone, Debug)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

static NEIGHBOUR_POSITIONS: [(isize, isize); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize) -> Grid<T>
    where 
        T: Default + Clone,
    {
        let data = vec![T::default(); width * height];
        Grid { width, height, data }
    }

    pub fn with_data(width: usize, height: usize, data: Vec<T>) -> Grid<T> {
        assert!(data.len() == width * height, "invalid data size: {}, w={}, h={}", data.len(), width, height);
        Grid { width, height, data }
    }

    pub fn get(&self, x: usize, y: usize) -> T
    where 
        T: Copy,
    {
        debug_assert!(x < self.width, "w = {}, x = {}", self.width, x);
        debug_assert!(y < self.height, "h = {}, y = {}", self.height, y);
        self.data[y * self.width + x]
    }

    pub fn get_ref(&self, x: usize, y: usize) -> &T {
        debug_assert!(x < self.width, "w = {}, x = {}", self.width, x);
        debug_assert!(y < self.height, "h = {}, y = {}", self.height, y);
        &self.data[y * self.width + x]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        debug_assert!(x < self.width, "w = {}, x = {}", self.width, x);
        debug_assert!(y < self.height, "h = {}, y = {}", self.height, y);
        &mut self.data[y * self.width + x]
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        *self.get_mut(x, y) = value;
    }

    pub fn set_all(&mut self, value: T)
    where
        T: Clone,
    {
        for cell in self.data.iter_mut() {
            *cell = value.clone();
        }
    }

    pub fn neighbours(&self, x: usize, y: usize) -> impl Iterator<Item = T> + '_
    where
        T: Copy,
    {
        let (x, y) = (x as isize, y as isize);
        let (w, h) = (self.width as isize, self.height as isize);
        NEIGHBOUR_POSITIONS.iter()
            .flat_map(move |(dx, dy)| {
                let new_x = x + dx;
                let new_y = y + dy;
                if new_x >= 0 && new_y >= 0 && new_x < w && new_y < h {
                    Some(self.get(new_x as usize, new_y as usize))
                } else {
                    None
                }
            })
    }

    pub fn neighbours_wrapped(&self, x: usize, y: usize) -> impl Iterator<Item = T> + '_
    where
        T: Copy,
    {
        let (x, y) = (x as isize, y as isize);
        let (w, h) = (self.width as isize, self.height as isize);
        NEIGHBOUR_POSITIONS.iter()
            .flat_map(move |(dx, dy)| {
                let new_x = (x + dx + w) % w;
                let new_y = (y + dy + h) % h;
                Some(self.get(new_x as usize, new_y as usize))
            })
    }
}

#[derive(Debug)]
pub struct Game<T> {
    old_grid: Grid<T>,
    grid: Grid<T>,
}

impl<T> From<Grid<T>> for Game<T>
where
    T: Clone,
{
    fn from(grid: Grid<T>) -> Game<T> {
        Game { old_grid: grid.clone(), grid }
    }
}

impl<T: Default + Clone> Game<T> {
    pub fn new(width: usize, height: usize) -> Game<T> {
        let grid = Grid::new(width, height);
        let old_grid = Grid::new(width, height);
        Game { old_grid, grid }
    }

    pub fn grid(&self) -> &Grid<T> {
        &self.grid
    }

    pub fn grid_mut(&mut self) -> &mut Grid<T> {
        &mut self.grid
    }

    pub fn old_grid(&self) -> &Grid<T> {
        &self.old_grid
    }

    pub fn next_turn(&mut self) {
        mem::swap(&mut self.grid, &mut self.old_grid);
    }
}
