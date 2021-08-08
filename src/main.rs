use std::env;
use std::cmp::max;
use std::cmp::min;

type Position = (usize, usize);

fn value_to_tuple(value: u64) -> (i64, i64) {
    (((value & 0b01u64) * 2) as i64 - 1, ((value & 0b10u64) as i64) - 1)
}

fn move_1d(pos: usize, delta: i64, bound: usize) -> usize {
    let attempt_pos: i64 = pos as i64 + delta;
    min(max(attempt_pos, 0) as usize, bound - 1)
}

fn apply_move((pos_x, pos_y): Position,
              (delta_x, delta_y): (i64, i64),
              (width, height): Position
             ) -> Position {
    (
        move_1d(pos_x, delta_x, width),
        move_1d(pos_y, delta_y, height)
    )
}

fn populate_table(mut value: u64, width: usize, height: usize) -> Vec<Vec<u64>> {
    let mut table = vec![vec![0u64; width]; height];
    let mut bishop = (width / 2, height / 2);

    while value != 0 {
        let delta = value_to_tuple(value);
        value >>= 2;

        bishop = apply_move(bishop, delta, (width, height));
        table[bishop.1][bishop.0] += 1;
    }

    table
}

fn pretty_print(table: Vec<Vec<u64>>) -> String {
    let chars = [' ', '.', 'o', '+', '=', '*', 'B', '0', 'X', '@', '%', '&', '#', '/', '^'];
    let mut header = String::new();
    header.push('+');
    header.push_str("-".repeat(table[0].len()).as_str());
    header.push('+');
    header.push('\n');

    let mut res = String::new();
    res.push_str(header.as_str());
    for row in table {
        res.push('|');
        for i in row {
            res.push(chars[min(i as usize, chars.len() - 1)]);
        }
        res.push_str("|\n");
    }
    res.push_str(header.as_str());

    res
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        panic!("Expected an argument");
    }

    match args[1].parse::<u64>() {
        Err(error) => panic!("Could not parse argument {}: {:?}", args[1], error),
        Ok(n) => println!("{}", pretty_print(populate_table(n, 9, 7))),
    };
}
