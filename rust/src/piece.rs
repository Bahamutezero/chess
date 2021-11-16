#[derive(Copy, Clone)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}


#[derive(Copy, Clone)]
pub struct Piece {
    color: Color,
    piece_type: PieceType,
}

impl Piece {
    pub fn new(rep: String) -> Piece {
        if rep.len() != 2 {
            panic!(rep.len())
        }

    }
}
