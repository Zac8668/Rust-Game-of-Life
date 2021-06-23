use ::rand::random;
use macroquad::prelude::*;

const X: usize = 50;
const Y: usize = 50;
const SCALE: f32 = 12.0;
type Grid = [[bool; X]; Y];
const RANDOMIZE: bool = false;

fn window_conf() -> Conf {
    Conf {
        window_title: "ConwayGameOfLife".to_string(),
        window_resizable: false,
        window_width: X as i32 * SCALE as i32,
        window_height: Y as i32 * SCALE as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut grid: [[bool; X as usize]; Y as usize] = [[false; X as usize]; Y as usize];
    let mut running: bool = true;

    if RANDOMIZE {
        for x in 0..X {
            for y in 0..Y {
                grid[y][x] = random();
            }
        }
    }

    loop {
        if is_key_pressed(KeyCode::P) {
            running = !running;
        }

        if is_mouse_button_down(MouseButton::Left) {
            let pos = mouse_position();
            let pos = ((pos.0 / SCALE) as usize, (pos.1 / SCALE) as usize);

            grid[pos.1][pos.0] = true;
        }

        clear_background(LIGHTGRAY);
        grid = update_grid(grid, &running);
        draw_text(&running.to_string(), 10.0, 50.0, 60.0, DARKGRAY);
        draw_text(&get_fps().to_string(), 10.0, 100.0, 60.0, DARKGRAY);

        next_frame().await
    }
}

fn update_grid(mut grid: Grid, running: &bool) -> Grid {
    let mut changed_cells = Vec::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let after_cell = [grid[y as usize][x as usize]];

            if *running {
                let after_cell = update_cell(&[x as i16, y as i16], &grid, &cell);

                if after_cell[1] {
                    changed_cells.push(([x, y], after_cell[0]));
                }
            }

            if after_cell[0] {
                draw_rectangle(
                    (x * SCALE as usize) as f32,
                    (y * SCALE as usize) as f32,
                    SCALE,
                    SCALE,
                    BLACK,
                );
            }
        }
    }

    for cell in changed_cells {
        grid[cell.0[1]][cell.0[0]] = cell.1;
    }

    grid
}

fn update_cell(cell: &[i16; 2], grid: &Grid, state: &bool) -> [bool; 2] {
    let n = live_neigh(&cell, &grid);
    let new_state;

    if *state {
        //alive
        new_state = n == 2 || n == 3;
    } else {
        //dead
        new_state = n == 3;
    }

    //current state | if the cell changed
    [new_state, new_state != *state]
}

//checks if the position exists
fn valid_pos(pos: &[i16; 2]) -> bool {
    if pos[0] < 0 || pos[0] > X as i16 - 1 {
        return false;
    }

    if pos[1] < 0 || pos[1] > Y as i16 - 1 {
        return false;
    }

    true
}

//checks how much neighbours a cell has
fn live_neigh(cell: &[i16; 2], grid: &Grid) -> i8 {
    let mut n: i8 = 0;

    for x in -1..2 {
        for y in -1..2 {
            if valid_pos(&[cell[0] + x, cell[1] + y])
                && !(x == 0 && y == 0)
                && grid[(cell[1] + y) as usize][(cell[0] + x) as usize]
            {
                n += 1;
            }
        }
    }
    n
}
