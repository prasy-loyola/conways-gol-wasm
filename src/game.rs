#[allow(dead_code)]
extern "C" {
    pub fn fillRect(x: u32, y: u32, w: u32, h: u32, r: u8, g: u8, b: u8, a: u8);
    pub fn print(text: *const u8, len: usize);
}

#[derive(Debug, Clone)]
struct Coord {
    col: usize,
    row: usize,
}

#[derive(Debug, Clone)]
struct Cell {
    is_alive: bool,
    alive_neighbours: u8,
}

#[derive(Debug)]
pub struct Game {
    state: Vec<Vec<Cell>>,
    live_cells: Vec<Coord>,
    cell_size: u32,
    screen_width: u32,
    screen_height: u32,
    border_width: u32,
    rows: usize,
    cols: usize,
    color: u8,
}

impl Game {

    fn mark_neighbours(&mut self, row: usize, col: usize) {
        let coord = Coord { row: row, col: col };

        for row_offset in -1..2 as i32 {
            for col_offset in -1..2 as i32 {
                let row = coord.row as i32 + row_offset;
                let col = coord.col as i32 + col_offset;

                if row_offset == 0 && col_offset == 0 {
                    continue;
                }
                if col < 0 || col >= self.cols as i32 {
                    continue;
                }
                if row < 0 || row >= self.rows as i32 {
                    continue;
                }
                self.state[row as usize][col as usize].alive_neighbours += 1;
                //log_console(format!("Curr row: {}, col:{} | marking cell row: {}, col: {}", coord.row, coord.col, row, col));
            }
        }
    }

    fn next_generation(&mut self) {
        let new_alive_cells = vec![];

        //self.state = Game::new_state(self.rows, self.cols);
        self.state
            .iter_mut()
            .flat_map(|r| r.iter_mut())
            .for_each(|c| c.alive_neighbours = 0);
        for cell in self.live_cells.clone().iter() {
            self.mark_neighbours(cell.row, cell.col);
        }

        log_console(format!("before live_cells: {:?}", self.live_cells));
        self.live_cells = new_alive_cells;
        for row in 0..self.rows as i32 {
            for col in 0..self.cols as i32 {
                let alive_neighbours = self.state[row as usize][col as usize].alive_neighbours;
                //Die of loneliness
                if alive_neighbours < 2 {
                    self.state[row as usize][col as usize].is_alive = false;
                }
                //Die of overpopulation
                else if alive_neighbours > 3 {
                    self.state[row as usize][col as usize].is_alive = false
                } else if !self.state[row as usize][col as usize].is_alive {
                    //new life
                    if alive_neighbours == 3 {
                        self.state[row as usize][col as usize].is_alive = true;
                        self.live_cells.push(Coord {
                            row: row as usize,
                            col: col as usize,
                        });
                    }
                } else {
                    self.state[row as usize][col as usize].is_alive = true;
                    self.live_cells.push(Coord {
                        row: row as usize,
                        col: col as usize,
                    });
                }
            }
        }

        log_console(format!("after live_cells: {:?}", self.live_cells));
    }

    fn add_pattern(&mut self, pattern: &str, row_offset: usize, col_offset: usize) {
        //self.live_cells = vec![];
        let initial_state: Vec<Vec<u8>> = pattern
            .lines()
            .filter(|l| !(l.is_empty() || l.starts_with("!")))
            .map(|c| {
                c.chars()
                    .map(|c| if c == 'O' { self.color } else { 0 })
                    .collect()
            })
            .collect();
        //log_console(format!("Got initial state :{}, rows: {}, cols: {}", initial_state.len(), self.rows, self.cols));

        //log_console(format!("cells size rows: {}, cols: {}", self.state.len(), self.state[0].len()));
        let mut r = 0;
        for row in initial_state.iter() {
            let mut c = 0;
            for col in row.iter() {
                if r + row_offset >= self.rows || c + col_offset >= self.cols {
                    c += 1;
                    continue;
                }
                log_console(format!(
                    "updating row: {}, col: {}",
                    r + row_offset,
                    c + col_offset
                ));
                self.state[r + row_offset][c + col_offset].is_alive = *col > 0;
                if *col > 0 {
                    self.live_cells.push(Coord {
                        col: (c + col_offset),
                        row: (r + row_offset),
                    })
                }
                c += 1;
            }
            r += 1;
        }
        log_console(format!("after pattern addtion: {:?}", self.live_cells));
    }

    fn new_state(rows: usize, cols: usize) -> Vec<Vec<Cell>> {
        let mut state = vec![vec![]];
        for row in 0..rows {
            state.push(vec![]);
            for _col in 0..cols {
                state[row].push(Cell {
                    is_alive: false,
                    alive_neighbours: 0,
                });
            }
        }
        log_console(format!(
            "state rows: {} cols: {}",
            state.len(),
            state[0].len()
        ));
        return state;
    }
    fn reset(&mut self) {
        self.state = Game::new_state(self.rows, self.cols);
        self.live_cells = vec![];
    }

    fn change_cell_size(&mut self, cell_size: u32) {
        self.cell_size = cell_size;
        self.rows = (self.screen_height / cell_size) as usize;
        self.cols = (self.screen_width / cell_size) as usize;
        self.state = Game::new_state(self.rows, self.cols);
        self.live_cells = vec![];
    }

    fn new(width: u32, height: u32, cell_size: u32, border_width: u32) -> Game {
        let color = 230;
        let screen_height: u32 = height;
        let screen_width: u32 = width;
        let rows = (screen_height / cell_size) as usize;
        let cols = (screen_width / cell_size) as usize;

        let game = Game {
            state: Game::new_state(rows, cols),
            live_cells: vec![],
            color: color,
            cell_size: cell_size,
            border_width: border_width,
            screen_height: screen_height,
            screen_width: screen_width,
            rows: rows,
            cols: cols,
        };
        return game;
    }

    fn render(&self) {
        self.live_cells.iter().for_each(|c| unsafe {
            fillRect(
                (c.col as u32 * self.cell_size) as u32 + self.border_width,
                (c.row as u32 * self.cell_size) as u32 + self.border_width,
                self.cell_size - self.border_width,
                self.cell_size - self.border_width,
                self.color,
                0,
                0,
                255,
            );
        });
    }
}

#[no_mangle]
pub fn init(width: u32, height: u32, cell_size: u32, border_width: u32) -> u32 {
    let game = Game::new(width, height, cell_size, border_width);
    draw_background(&game);
    return Box::into_raw(Box::new(game)) as u32;
}

fn draw_background(game: &Game) {
    unsafe {
        fillRect(0, 0, game.screen_width, game.screen_height, 56, 56, 56, 255);
    }
}

#[no_mangle]
pub fn render(game: *mut Game) {
    let game = get_game(game);
    draw_background(&game);
    game.render();
}

fn log_console(_message: String) {
    return;
    //unsafe {
        //print(_message.as_str().as_ptr(), _message.len());
    //}
}

#[no_mangle]
pub fn add_pattern(game: *mut Game, pattern: &str, row_offset: usize, col_offset: usize) {
    log_console(format!("game: {:?}", game));
    let game = get_game(game);

    log_console(format!("got game: {:?}", game.cols));
    game.add_pattern(pattern, row_offset, col_offset);
}

#[no_mangle]
pub fn reset(game: *mut Game) {
    let game = get_game(game);
    game.reset();
    draw_background(&game);
}

#[no_mangle]
pub fn change_cell_size(game: *mut Game, cell_size: u32) {
    let game = get_game(game);
    game.change_cell_size(cell_size);
    draw_background(&game);
}

#[no_mangle]
pub fn update(game: *mut Game) {
    let game = get_game(game);
    game.next_generation();
}

pub fn get_game(game: *mut Game) -> &'static mut Game {
    let game = unsafe {
        let ref_mut: &mut Game = &mut *game;
        let game = &mut *ref_mut;
        game
    };
    return game;
}
