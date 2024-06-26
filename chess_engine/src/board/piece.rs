#[repr(u8)]
pub(super) enum Piece {
    King   = 0b00000,
    Queen  = 0b00001,
    Rook   = 0b00010,
    Bishop = 0b00011,
    Knight = 0b00100,
    Pawn   = 0b00101,

    None   = 0b00111
}

#[repr(u8)]
pub(super) enum Color {
    White = 0b01000,
    Black = 0b10000
}

impl Piece {
    pub fn new(c: Color, p: Piece) -> u8 {
        p as u8 | c as u8
    }
}

