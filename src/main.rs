extern crate rustty;
extern crate game_grid;

use std::io;
use std::time::Duration;
use std::mem;

mod game;
use game::Gol;


enum AppState {
    Paused, Editing, Running,
}

impl AppState {
    fn toggle_paused(&mut self) {
        match self {
            AppState::Paused => { mem::replace(self, AppState::Running); },
            AppState::Running => { mem::replace(self, AppState::Paused); },
            _ => ()
        }
    }
}

struct App {
    term: rustty::Terminal,
    game: Gol,
    state: AppState,
    frame_delay: u64,
    edit_cursor: (usize, usize),
}

impl App {
    fn new() -> Result<App, io::Error> {
        let term = rustty::Terminal::new()?;
        let game = Gol::new(term.cols(), term.rows(), true);
        Ok(App { term, game, state: AppState::Running, edit_cursor: (0, 0), frame_delay: 5 })
    }

    fn run(&mut self) -> Result<(), io::Error> {
        loop {
            if let Some(evt) = self.term.get_event(Duration::from_millis(self.frame_delay))? {
                match self.state {
                    AppState::Editing => {
                        match evt {
                            rustty::Event::Key('q') => break,
                            rustty::Event::Key(key) => self.editing(key),
                        }
                    },
                    _ => {
                        match evt {
                            rustty::Event::Key('q') => break,
                            rustty::Event::Key('c') => self.clear_grid(),
                            rustty::Event::Key('p') => self.state.toggle_paused(),
                            rustty::Event::Key('e') => { self.state = AppState::Editing },
                            rustty::Event::Key(_) => (),
                        }
                    },
                }
            }

            match self.state {
                AppState::Running => {
                    self.game_step();
                },
                _ => ()
            }

            self.term.swap_buffers()?;
        }
        Ok(())
    }

    fn game_step(&mut self) {
        self.game.next_turn();
        self.draw_game();
    }
    
    fn draw_game(&mut self) {
        let grid = self.game.grid();
        let width = self.term.cols();
        let height = self.term.rows();
        for x in 0 .. width {
            for y in 0 .. height {
                if grid.get(x, y) == 1 {
                    self.term[(x, y)].set_fg(rustty::Color::Yellow);
                    self.term[(x, y)].set_bg(rustty::Color::Red);
                } else {
                    self.term[(x, y)].set_fg(rustty::Color::Black);
                    self.term[(x, y)].set_bg(rustty::Color::Black);
                }
            }
        }
    }

    fn editing(&mut self, key: char) {
        match key {
            'i' => self.move_edit_cursor(0, -1),
            'k' => self.move_edit_cursor(0, 1),
            'j' => self.move_edit_cursor(-1, 0),
            'l' => self.move_edit_cursor(1, 0),
            'g' => self.glider_gun(),
            '1' => self.glider(1),
            '2' => self.glider(2),
            '3' => self.glider(3),
            '4' => self.glider(4),
            'c' => self.clear_grid(),
            ' ' => self.toggle_edit_cell(),
            '\r' => self.state = AppState::Running,
            _ => (),
        }
    }

    fn clear_grid(&mut self) {
        let width = self.term.cols();
        let height = self.term.rows();
        for x in 0 .. width {
            for y in 0 .. height {
                self.game.grid_mut().set(x, y, 0);
            }
        }
    }

    fn glider_gun(&mut self) {
        let glider_gun = include_bytes!("presets/glider_gun.txt");
        self.draw_from_ascii(glider_gun);
    }

    fn glider(&mut self, variant: u8) {
        let shape: &[u8] = match variant {
            1 => include_bytes!("presets/glider_1.txt"),
            2 => include_bytes!("presets/glider_2.txt"),
            3 => include_bytes!("presets/glider_3.txt"),
            4 => include_bytes!("presets/glider_4.txt"),
            _ => b"",
        };
        self.draw_from_ascii(shape);
    }

    fn draw_from_ascii(&mut self, data: &[u8]) {
        let (x, y) = self.edit_cursor;
        let (w, h) = (self.term.cols(), self.term.rows());
        let lines: Vec<Vec<u8>> = data.splitn(usize::max_value(), |&c| c == b'\n')
            .map(|line| line.into_iter().cloned().collect())
            .collect();
        for j in 0 .. lines.len() {
            let row = &lines[j];
            for i in 0 .. row.len() {
                self.edit_cursor = ((x + i + w) % w, (y + j + h) % h);
                if lines[j][i] != b' ' {
                    self.toggle_edit_cell();
                }
            }
        }
        self.draw_game();
    }

    fn move_edit_cursor(&mut self, by_horiz: isize, by_vert: isize) {
        let (x, y) = self.edit_cursor;
        let grid = self.game.grid();
        let width = self.term.cols();
        let height = self.term.rows();
        if grid.get(x, y) == 0 {
            self.term[(x, y)].set_bg(rustty::Color::Black);
        } else {
            self.term[(x, y)].set_bg(rustty::Color::Red);
        }
        let (x, y) = (((x + width) as isize + by_horiz) as usize % width, 
                    ((y + height) as isize + by_vert) as usize % height);
        self.edit_cursor = (x, y);
        self.term[(x, y)].set_bg(rustty::Color::White);
    }

    fn toggle_edit_cell(&mut self) {
        let (x, y) = self.edit_cursor;
        let grid = self.game.grid_mut();
        let current = grid.get(x, y);
        if current == 0 {
            grid.set(x, y, 1);
            self.term[(x, y)].set_bg(rustty::Color::Green);
        } else {
            grid.set(x, y, 0);
            self.term[(x, y)].set_bg(rustty::Color::White);
        }
    }
}

fn main() -> Result<(), io::Error> {
    let mut app = App::new()?;
    app.run()
}
