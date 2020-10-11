mod util;

use std::io::stdin;
use std::io::prelude::*;

fn main() {
    play()
}

const CELL_IS_TURNED: u8 = 0b001;
const CELL_IS_BOMB: u8 = 0b010;
const CELL_IS_MARKED: u8 = 0b100;

const FIELD_WIDTH: u8 = 15;
const FIELD_HEIGHT: u8 = 10;
const BOMB_COUNT: u8 = 10;
type Field = [[u8; FIELD_WIDTH as usize]; FIELD_HEIGHT as usize];

fn play() {
    let mut field = generate_field(BOMB_COUNT);
    let mut hit_bomb = false;
    while !hit_bomb {
        draw(&field);
        hit_bomb = action(&mut field);
    }
}

fn action(field: &mut Field) -> bool {
    let mut action = [0; 3];
    stdin().read_exact(&mut action).unwrap();
    stdin().read_line(&mut String::new()).unwrap();

    let a = action[0];
    let mut x = action[1] as usize - 'A' as usize;

    if x >= 32 {
        x -= 32;
    }

    let y = action[2] as usize - '0' as usize;

    match a as char {
        'p' | 'P' => {
            if field[y][x] & CELL_IS_TURNED == 0 {
                field[y][x] ^= CELL_IS_MARKED;
            }
        },
        't' | 'T' => {
            if field[y][x] & (CELL_IS_MARKED | CELL_IS_TURNED) != 0 {
                return false;
            }
            if field[y][x] & CELL_IS_BOMB == CELL_IS_BOMB {
                field[y][x] |= CELL_IS_TURNED;
                println!("You lost");
                return true;
            }
            turn_field(x, y, field);
        },
        _ => println!("Unknown input")
    }
    false
}

fn turn_field(x: usize, y: usize, field: &mut Field) {
    
    if (field[y][x] & CELL_IS_TURNED) == CELL_IS_TURNED {
        return;
    }

    if (field[y][x] >> 4) != 0 {
        field[y][x] |= CELL_IS_TURNED;
        return;
    }

    field[y][x] |= CELL_IS_TURNED;

    if x < FIELD_WIDTH as usize - 1 {
        turn_field(x + 1, y, field);
    }
    if x > 0 {
        turn_field(x - 1, y, field);
    }
    if y < FIELD_HEIGHT as usize - 1 {
        turn_field(x, y + 1, field);
    }
    if y > 0 {
        turn_field(x, y - 1, field);
    }
}

fn draw(field: &Field) {
    util::clear();
    print!(" |");
    ('A'..(FIELD_WIDTH + 'A' as u8) as char).for_each(|c| print!("{}|", c));
    println!();
    println!("{}", "-+".repeat(FIELD_WIDTH as usize + 1));
    for y in 0..FIELD_HEIGHT as usize {
        let row = field[y];
        print!("{}|", y);
        for x in 0..FIELD_WIDTH as usize {
            const CELL_IS_BOMB_AND_TURNED: u8 = CELL_IS_BOMB | CELL_IS_TURNED;
            const CELL_IS_BOMB_AND_MARKED: u8 = CELL_IS_BOMB | CELL_IS_MARKED;
            let cell = row[x];
            match cell & 0b0000_1111 {
                0 | CELL_IS_BOMB => print!("#|"),
                CELL_IS_MARKED | CELL_IS_BOMB_AND_MARKED => print!("P|"),
                CELL_IS_BOMB_AND_TURNED => print!("*|"),
                CELL_IS_TURNED => {
                    let bomb_count = cell >> 4;
                    if bomb_count > 0 {
                        print!("{}|", bomb_count);
                    } else {
                        print!(" |");
                    }
                },
                _ => print!("?|")
            }
        }
        println!("\n{}", "-+".repeat(FIELD_WIDTH as usize + 1));
    }
}

fn count_bombs(x: usize, y: usize, field: &Field) -> u8 {
    const SEARCH_AREA_X: core::ops::Range<i8> = -1..2;
    const SEARCH_AREA_Y: core::ops::Range<i8> = -1..2;
    
    let mut bomb_count = 0;
    for y_area in SEARCH_AREA_Y {
        for x_area in SEARCH_AREA_X {
            let check_x = x as i8 + x_area;
            let check_y = y as i8 + y_area;
            if y_area == 0 && x_area == 0 || check_x < 0 || check_x >= FIELD_WIDTH as i8 || check_y < 0 || check_y >= FIELD_HEIGHT as i8 {
                continue;
            }
            if field[check_y as usize][check_x as usize] & CELL_IS_BOMB == CELL_IS_BOMB {
                bomb_count += 1;
            }
        }
    }
    bomb_count
}

fn generate_field(mines: u8) -> Field{
    let mut field = [[0; FIELD_WIDTH as usize]; FIELD_HEIGHT as usize];
    
    for _ in 0..mines {
        loop {
            let x = (rand::random::<f64>() * FIELD_WIDTH as f64).floor() as usize;
            let y = (rand::random::<f64>() * FIELD_HEIGHT as f64).floor() as usize;

            if field[y][x] != CELL_IS_BOMB {
                field[y][x] = CELL_IS_BOMB;
                break;
            }
        }
    }

    for y in 0..FIELD_HEIGHT as usize {
        for x in 0..FIELD_WIDTH as usize {
            let mut bomb_count = count_bombs(x, y, &field);
            bomb_count <<= 4;
            field[y][x] += bomb_count;
        }
    }

    field
}