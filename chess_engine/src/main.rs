mod board;

use board::Board;

fn main() {
    let b = Board::new();
    draw_board(b);
}

fn draw_board(b: Board) {
    print!("   ");
    for c in 'a'..='h' { print!("{c} "); }
    println!("\n");
    for rank in 0..8 {
        print!("{}  ", (rank as i32 - 8).abs());

        for file in 0..8 {
            let square: usize = 8usize * rank + file;
            print!("{} ", u2piece(b.squares[square]));
        }

        print!(" {}", rank+1);

        println!();
    }
    print!("\n   ");
    for c in 'a'..='h' { print!("{c} "); }
    println!("\n    {} to move", if b.white_to_move { "white" } else { "black" });
}

fn u2piece(n: u8) -> char {
    if n == 0b00111 { return 'x'; }
    
    static WHITES: [char; 6] = ['\u{2654}', '\u{2655}', '\u{2656}', '\u{2657}', '\u{2658}', '\u{2659}'];
    static BLACKS: [char; 6] = ['\u{265A}', '\u{265B}', '\u{265C}', '\u{265D}', '\u{265E}', '\u{265F}'];

    if n & 0b10000 > 0 { WHITES[n as usize & 0b00111] }
    else { BLACKS[n as usize & 0b00111] }
}
