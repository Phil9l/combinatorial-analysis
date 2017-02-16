use std::fs::File;
use std::io::Read;

const FIELD_SIZE : usize = 12;
static DELTAS: [(i32, i32); 8] = [(-2, 1), (-2, -1), (-1, -2), (1, -2), (2, -1), (2, 1), (1, 2), (-1, 2)];

fn get_chess_coords(coords: (usize, usize)) -> (char, char) {
	return (('a' as u8 + coords.0 as u8 - 2) as char, ('1' as u8 + coords.1 as u8 - 2) as char);
}

fn dfs(field: &mut [[bool; FIELD_SIZE]; FIELD_SIZE], start: (usize, usize), finish: (usize, usize)) -> bool {
	if field[start.0][start.1] {
		return false;
	}
	
    field[start.0][start.1] = true;

	if start == finish {
		let out_pos = get_chess_coords(start);
		println!("{}{}", out_pos.0, out_pos.1);
		return true;
	}
    
    for delta in DELTAS.iter() {
		let new_pos = ((start.0 as i32 + delta.0) as usize, (start.1 as i32 + delta.1) as usize);
		// println!("{} {} -> {} {}", start.0, start.1, new_pos.0, new_pos.1);
		// return true;
		if dfs(field, new_pos, finish) {
			let out_pos = get_chess_coords(start);
			println!("{}{}", out_pos.0, out_pos.1);
			return true;
		}
    }

    return false;
}

fn main() {
    let mut file = File::open("input.txt").expect("opening file");
    let mut input_text = String::new();
    file.read_to_string(&mut input_text).expect("reading file");
   
    let mut lines = input_text.lines();

    let mut line = lines.next().unwrap().chars();
    let knight = (line.next().unwrap() as usize - 'a' as usize + 2, line.next().unwrap() as usize - '1' as usize + 2);

    let mut line = lines.next().unwrap().chars();
    let pawn = (line.next().unwrap() as usize - 'a' as usize + 2, line.next().unwrap() as usize - '1' as usize + 2);
    
    let mut field = [[false; FIELD_SIZE]; FIELD_SIZE];
    
    for i in 0..FIELD_SIZE {
		for j in 0..2 {
			field[i][j] = true;
			field[i][FIELD_SIZE - 1 - j] = true;
			field[j][i] = true;
			field[FIELD_SIZE - 1 - j][i] = true;
		}
    }
    field[pawn.0 - 1][pawn.1 - 1] = true;
    field[pawn.0 + 1][pawn.1 - 1] = true;
	
	dfs(&mut field, pawn, knight);
}
