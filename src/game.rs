
extern "C" {
    fn fillRect(x: u32, y: u32, w: u32, h: u32, r: u8, g: u8, b: u8, a: u8);
    fn print(text: &str);
}

#[derive(Debug, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct Cell {
    is_alive: bool,
    alive_neighbours : u8,
    coord: Coord,
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
    fn is_alive(&self, row: i32, col: i32) -> bool {
        if row < 0 || col < 0 || col >= self.cols as i32 || row >= self.rows as i32 {
            return false;
        }

        if self.state[row as usize][col as usize].is_alive {
            return true;
        }
        return false;
    }

    fn mark_neighbours(&mut self , coord: Coord) {

        for row_offset in -1..2 as i32 {
            for col_offset in -1..2 as i32 {
                if  !(row_offset ==0 && col_offset ==0 ) && 
                    coord.x as i32 + col_offset  >= 0 &&
                    coord.x as i32 + col_offset < self.cols as  i32 &&

                    coord.y  as i32 + row_offset >= 0 &&
                    coord.y as i32 + row_offset < self.rows  as i32
                {
                    self.state[(coord.y as i32 + row_offset) as usize][(coord.x as i32 + col_offset) as usize].alive_neighbours += 1;
                }
            }
        }

    }

    fn next_generation(&mut self) {
        let mut next_state = self.state.clone();
        let mut new_alive_cells = self.live_cells.clone();


        

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

                //Die of loneliness
                if alive_neighbours < 2 {
                    next_state[row as usize][col as usize].is_alive = false;
                } 
                //Die of overpopulation 
                else if alive_neighbours > 3 {
                    next_state[row as usize][col as usize].is_alive =false
                } 
                
                else if !self.state[row as usize][col as usize].is_alive {
                    //new life
                    if alive_neighbours == 3 {
                        next_state[row as usize][col as usize].is_alive = true;
                    }
                }
                else {

                        next_state[row as usize][col as usize].is_alive = true;
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
            log_console(format!("Got initial state :{}, rows: {}, cols: {}", initial_state.len(), self.rows, self.cols));

            log_console(format!("cells size rows: {}, cols: {}", self.state.len(), self.state[0].len()));
        let mut r = 0;
        for row in initial_state.iter() {
            let mut c = 0;
            for col in row.iter() {
                
                if r + row_offset >= self.rows || c+col_offset >= self.cols {
                    c += 1;
                    continue;
                }
                log_console(format!("updating row: {}, col: {}", r+ row_offset, c + col_offset));
                self.state[r + row_offset][c + col_offset].is_alive = *col > 0 ;
                //if *col > 0 {self.live_cells.push(Coord{ x: (c + col_offset), y: (r + row_offset)})}
                c += 1;
            }
            r += 1;
        }
    }

    fn new_state ( rows: usize, cols: usize) -> Vec<Vec<Cell>> {
        let mut state = vec![vec![]];
        for row in 0..rows {
            state.push(vec![]);
            for col in 0.. cols {
                state[row].push(Cell { coord: Coord {x: col, y: row}, is_alive: false, alive_neighbours: 0 });
            }
        }
            log_console(format!("state rows: {} cols: {}", state.len(), state[0].len()));
        return state;
    }
    fn reset(&mut self){
        self.state  = Game::new_state(self.rows,self.cols);
    }

    fn change_cell_size(&mut self, cell_size: u32){
        self.cell_size = cell_size;
        self.rows = (self.screen_height / cell_size) as usize;
        self.cols = (self.screen_width / cell_size) as usize;
        self.state  = Game::new_state(self.rows, self.cols);
    }

    fn new(width: u32, height: u32, cell_size: u32, border_width: u32) -> Game {

        let color = 230;
        let screen_height : u32= height;
        let screen_width : u32= width;
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

    fn render(&self){

   
    for row in 0..self.rows as u32 {
        for col in 0..self.cols as u32 {
            unsafe {
                fillRect(
                    (col * self.cell_size) + self.border_width,
                    (row * self.cell_size) + self.border_width,
                    self.cell_size - self.border_width,
                    self.cell_size - self.border_width,
                    if self.state[row as usize][col as usize].is_alive { self.color } else {0},
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
pub fn init(width: u32, height: u32, cell_size: u32, border_width: u32) -> u32 {
    let game = Game::new(width,height,cell_size, border_width);
    draw_background(&game);
    return Box::into_raw(Box::new(game)) as u32;
}

fn draw_background(game: &Game){
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

fn log_console(message: String){

    return ;
    unsafe {
    print(&message.as_str());
    }
}

#[no_mangle]
pub fn add_pattern(game: *mut Game, pattern: &str, row_offset: usize, col_offset: usize){
    log_console(format!("game: {:?}", game));
    let game = get_game(game);

    log_console(format!("got game: {:?}", game.cols));
    game.update(pattern, row_offset, col_offset);
}

#[no_mangle]
pub fn reset(game: *mut Game){
    let game = get_game(game);
    game.reset();
    draw_background(&game);
}

#[no_mangle]
pub fn change_cell_size(game: *mut Game, cell_size: u32){
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