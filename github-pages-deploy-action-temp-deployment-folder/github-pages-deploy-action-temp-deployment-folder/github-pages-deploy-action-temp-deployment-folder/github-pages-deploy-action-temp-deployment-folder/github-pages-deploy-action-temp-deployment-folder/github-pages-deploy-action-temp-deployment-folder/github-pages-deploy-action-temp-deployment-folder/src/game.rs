
extern "C" {
    fn alert(s: &str);
    fn fillRect(x: u32, y: u32, w: u32, h: u32, r: u8, g: u8, b: u8, a: u8);
}
#[derive(Debug)]
pub struct Game {
    state: Vec<Vec<u8>>,
    cell_size: u32,
    screen_width: u32,
    screen_height: u32,
    border_width: u32,
    rows: usize,
    cols: usize,
    color: u8,
}

impl Game {
    fn is_alive(&self, row: i32, col: i32) -> bool {
        if row < 0 || col < 0 || col >= self.cols as i32 || row >= self.rows as i32 {
            return false;
        }

        if self.state[row as usize][col as usize] > 0 {
            return true;
        }
        return false;
    }

    fn next_day(&mut self) {
        let mut next_state = self.state.clone();

        for row in 0..self.rows as i32 {
            for col in 0..self.cols as i32 {
                let mut alive_neighbours = 0;

                for r in -1..2 {
                    for c in -1..2 {
                        if !(r== 0 && c ==0) && self.is_alive(row + r, col + c) {
                            alive_neighbours +=1;
                        }
                    }
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

                //Die of loneliness
                if alive_neighbours < 2 {
                    next_state[row as usize][col as usize] = 0;
                } 
                //Die of overpopulation 
                else if alive_neighbours > 3 {
                    next_state[row as usize][col as usize] = 0;
                } 
                
                else if self.state[row as usize][col as usize] == 0 {
                    //new life
                    if alive_neighbours == 3 {
                        next_state[row as usize][col as usize] = self.color;
                    }
                }
                else {

                        next_state[row as usize][col as usize] = (alive_neighbours - 3 ) * 50 + self.color;
                }
            }
        }
        self.state = next_state;
    }

    fn update(&mut self, pattern: &str, row_offset: usize, col_offset: usize) {
        let initial_state: Vec<Vec<u8>> = pattern
            .lines()
            .filter(|l| !(l.is_empty() || l.starts_with("!")))
            .map(|c| {
                c.chars()
                    .map(|c| if c == 'O' { self.color } else { 0 })
                    .collect()
            })
            .collect();

        let mut r = 0;
        for row in initial_state.iter() {
            let mut c = 0;
            for col in row.iter() {
                
                if r + row_offset >= self.rows || c+col_offset >= self.cols {
                    c += 1;
                    continue;
                }

                self.state[r + row_offset][c + col_offset] = *col;
                c += 1;
            }
            r += 1;
        }
    }

    fn reset(&mut self){
        self.state  = vec![vec![0; self.cols]; self.rows];
    }
    fn new(initial: &str, row_offset: usize, col_offset: usize) -> Game {
        unsafe {
            //alert(format!("initial : {:?}", initial).as_str());
        }

        let color = 200;
        let cell_size: u32 = 8;
        let border_width = 1;
        let screen_height : u32= 600;
        let screen_width : u32= 1300;
        let rows = (screen_height / cell_size) as usize;
        let cols = (screen_width / cell_size) as usize;

        let mut game = Game {
            state: vec![vec![0; cols]; rows],
            color: color,
            cell_size: cell_size,
            border_width: border_width,
            screen_height: screen_height,
            screen_width: screen_width,
            rows: rows,
            cols: cols,
        };
        //for row in 0..initial_state.len() {
        //for col in 0..initial_state[0].len() {
        //game.state[row + row_offset][col + col_offset] = initial_state[row][col];
        //}
        //}
        game.update(initial, row_offset, col_offset);
        return game;
    }

    fn render(&self){

   
    for row in 0..self.rows as u32 {
        for col in 0..self.cols as u32 {
            unsafe {
                fillRect(
                    (col * self.cell_size) + self.border_width,
                    (row * self.cell_size) + self.border_width,
                    self.cell_size - self.border_width,
                    self.cell_size - self.border_width,
                    self.state[row as usize][col as usize],
                    0,
                    0,
                    255,
                );
            }
        }
    }
    }
}

#[no_mangle]
pub fn init(initial_state: &str, row_offset: usize, col_offset: usize) -> u32 {
    let mut game = Game::new(initial_state, row_offset, col_offset);
    unsafe {
        fillRect(0, 0, game.screen_width, game.screen_height, 56, 56, 56, 255);
    }
    unsafe {
        //alert(format!("{:?}", game).as_str());
    }

    return Box::into_raw(Box::new(game)) as u32;
}

#[no_mangle]
pub fn render(game: *mut Game) {
    let mut game = get_game(game);
    unsafe {
        fillRect(0, 0, game.screen_width, game.screen_height, 56, 56, 23, 10);
    }
    game.render();
}

#[no_mangle]
pub fn add_pattern(game: *mut Game, pattern: &str, row_offset: usize, col_offset: usize){
    let game = get_game(game);

    game.update(pattern, row_offset, col_offset);
}


#[no_mangle]
pub fn reset(game: *mut Game){
    let game = get_game(game);
    game.reset();
}

#[no_mangle]
pub fn update(game: *mut Game) {
    let mut game = get_game(game);
    game.next_day();
}

pub fn get_game(game: *mut Game) -> &'static mut Game {
    let mut game = unsafe {
        let ref_mut: &mut Game = &mut *game;
        let mut game = &mut *ref_mut;
        game
    };
    return game;
}