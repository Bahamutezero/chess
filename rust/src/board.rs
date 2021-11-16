use crate::piece::Piece;
use crate::piece::Color;
use crate::piece::PieceType;

pub struct Board {
    board: [Option<Piece>; 64],
}

impl Board {
    pub fn new() -> Board {
        let white_rook = Piece{
            color: Color::White,
            piece_type: PieceType::Rook
        };
        Board{
            board:[]
        }
    }
} 

