mod piece;
use piece::{Piece::{self, *}, Color::*};

#[derive(Debug)]
pub struct Board {
    // TODO: remove pub
    pub squares: [u8; 64],
    pub white_to_move: bool
}

impl Default for Board {
    fn default() -> Self {
        static start_fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        Self::from_FEN(start_fen).expect("the starting board should always be valid and if it's not, from_FEN() is incorrect")
    }
}

impl Board {
    pub fn new() -> Self { Default::default() }

    pub fn from_FEN(fen: &str) -> Result<Self, &'static str> {
        let mut fen = fen.split_whitespace();

        let fenBoard = fen.next().ok_or("FEN board missing")?;
        if fenBoard.split("/").count() != 8 { return Err("number of ranks should be 8")}

        let mut squares = [0b00111; 64];
        
        for (i, rank) in fenBoard.split("/").enumerate() {
            let mut j: usize = 0;

            for c in rank.chars() {
                if let Some(n) = c.to_digit(10) {
                    j += n as usize;
                    continue;
                }

                squares[8 * i + j] = match c {
                    'K' => Piece::new(White, King),   'Q' => Piece::new(White, Queen),
                    'R' => Piece::new(White, Rook),   'B' => Piece::new(White, Bishop),
                    'N' => Piece::new(White, Knight), 'P' => Piece::new(White, Pawn),

                    'k' => Piece::new(Black, King),   'q' => Piece::new(Black, Queen),
                    'r' => Piece::new(Black, Rook),   'b' => Piece::new(Black, Bishop),
                    'n' => Piece::new(Black, Knight), 'p' => Piece::new(Black, Pawn),

                     _  => return Err("invalid FEN board character")
                };

                j += 1;
            }

            if j != 8 { return Err("rank spacing should be 8")}
        }

        let fenActive = fen.next().ok_or("active color missing")?;
        if fenActive.len() != 1 { return Err("active color should be 1 character long"); }

        let white_to_move = match fenActive.chars().next().expect("We can unwrap as we've made sure fenActive is not empty") {
            'w' => true,
            'b' => false,
             _  => return Err("invalid FEN active color character")
        };

        Ok(Board { squares, white_to_move })
    }
}
