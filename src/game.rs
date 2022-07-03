extern "C" {
    fn alert(s: &str);
    fn fillRect(x: u32, y: u32, w: u32, h: u32, r: u8, g: u8, b: u8, a: u8);
}
const CELL_SIZE: u32 = 30;
const SCREEN_WIDTH: u32 = 1200;
const SCREEN_HEIGHT: u32 = 900;
const BORDER_WIDTH: u32 = 2;
const ROWS: usize = (SCREEN_HEIGHT / CELL_SIZE) as usize;
const COLS: usize = (SCREEN_WIDTH / CELL_SIZE) as usize;
#[derive(Debug)]
pub struct Game {
    name: String,
    state: Vec<Vec<u8>>,
}

impl Game {
    fn is_alive(&self, row: i32, col: i32) -> bool {
        if row < 0 || col < 0 || col >= COLS as i32 || row >= ROWS as i32 {
            return false;
        }

        if self.state[row as usize][col as usize] > 0 {
            return true;
        }
        return false;
    }

    fn next_day(&mut self) {
        let mut next_state = self.state.clone();

        for row in 0..ROWS as i32 {
            for col in 0..COLS as i32 {
                let mut alive_neighbours = 0;
                if self.is_alive(row - 1, col) {
                    alive_neighbours += 1;
                }
                if self.is_alive(row - 1, col - 1) {
                    alive_neighbours += 1;
                }
                if self.is_alive(row - 1, col + 1) {
                    alive_neighbours += 1;
                }
                if self.is_alive(row + 1, col) {
                    alive_neighbours += 1;
                }
                if self.is_alive(row + 1, col - 1) {
                    alive_neighbours += 1;
                }
                if self.is_alive(row + 1, col + 1) {
                    alive_neighbours += 1;
                }
                if self.is_alive(row, col - 1) {
                    alive_neighbours += 1;
                }
                if self.is_alive(row, col + 1) {
                    alive_neighbours += 1;
                }
                if alive_neighbours > 0 {
                    unsafe {
                        //alert(
                        //    format!(
                        //        " row: {}, col:{},alive neighbours {}",
                        //        row, col, alive_neighbours
                        //    )
                        //    .as_str(),
                        //);
                    }
                }
                if alive_neighbours < 2 {
                    next_state[row as usize][col as usize] = 0;
                } else if alive_neighbours > 3 {
                    next_state[row as usize][col as usize] = 0;
                } else if self.state[row as usize][col as usize] == 0 {
                    if alive_neighbours == 3 {
                        next_state[row as usize][col as usize] = 200;
                    }
                }
            }
        }
        self.state = next_state;
    }
}

#[no_mangle]
pub fn init() -> u32 {
    let mut game = Game {
        name: "New Game".to_string(),
        state: vec![vec![0; COLS]; ROWS],
    };
    unsafe {
        fillRect(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT, 56, 56, 56, 255);
    }
    game.state[1][10] = 200;
    game.state[2][10] = 200;
    game.state[3][10] = 200;
    game.state[3][9] = 200;

    game.state[2][8] = 200;
    unsafe {
        alert(format!("{:?}", game).as_str());
    }

    return Box::into_raw(Box::new(game)) as u32;
}

#[no_mangle]
pub fn render(game: *mut Game) {
    unsafe {
        fillRect(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT, 56, 56, 23, 10);
    }
    let mut game = unsafe {
        let ref_mut: &mut Game = &mut *game;
        let mut game = &mut *ref_mut;
        game
    };

    let mut r = 0;

    for row in 0..ROWS as u32 {
        for col in 0..COLS as u32 {
            unsafe {
                fillRect(
                    (col * CELL_SIZE) + BORDER_WIDTH,
                    (row * CELL_SIZE) + BORDER_WIDTH,
                    CELL_SIZE - BORDER_WIDTH,
                    CELL_SIZE - BORDER_WIDTH,
                    game.state[row as usize][col as usize],
                    0,
                    0,
                    255,
                );
            }
        }
    }
}

#[no_mangle]
pub fn update(game: *mut Game) {
    let mut game = unsafe {
        let ref_mut: &mut Game = &mut *game;
        let mut game = &mut *ref_mut;
        game
    };

    game.next_day();
}
