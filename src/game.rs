extern "C" {
    fn alert(s: &str);
    fn fillRect(x: u32, y: u32, w: u32, h: u32, r: u8, g: u8, b: u8, a: u8);
}
const CELL_SIZE: u32 = 30;
const SCREEN_WIDTH: u32 = 1200;
const SCREEN_HEIGHT: u32 = 900;
const BORDER_WIDTH: u32 = 2;
#[derive(Debug)]
pub struct Game {
    name: String,
    state: Vec<Vec<u8>>,
}

#[no_mangle]
pub fn init() -> u32 {
    let mut game = Game {
        name: "New Game".to_string(),
        state: vec![vec![0; (SCREEN_WIDTH / CELL_SIZE) as usize]; (SCREEN_HEIGHT / CELL_SIZE) as usize],
    };
   unsafe {

    fillRect(0, 0, SCREEN_WIDTH , SCREEN_HEIGHT, 23, 23, 23, 200);
   } 
    game.state[0][0] = 100;
    return Box::into_raw(Box::new(game)) as u32;
}

#[no_mangle]
pub fn render(num: *mut Game) {
    let mut game = unsafe {
        let ref_mut: &mut Game = &mut *num;
        let mut game = &mut *ref_mut;
        game
    };

    let mut r = 0;
    for row in game.state.iter() {
        let mut c = 0;
        for col in row.iter().filter(|x| x > &&0) {
            unsafe {
                fillRect(
                    c * CELL_SIZE  + BORDER_WIDTH,
                    r * CELL_SIZE  + BORDER_WIDTH,
                    CELL_SIZE  - BORDER_WIDTH,
                    CELL_SIZE - BORDER_WIDTH,
                    *col ,
                    0,
                    0,
                    255,
                );
            }
            c += 1;
        }
        r += 1;
    }
}
