mod make_move;
mod undo;
mod drops;
mod history;
mod io;

use crate::types::{Piece, PieceKind, Side, Square, PalaceState, SQUARES, FILES};
use crate::bitboard::Bitboard;

pub struct Position {
    pub board: [Piece; SQUARES],
    pub pieces: [[Bitboard; PieceKind::COUNT]; 2],
    pub occupancy: [Bitboard; 2],
    pub side_to_move: Side,
    pub hash: u64,
    pub reserve: [[u8; PieceKind::COUNT]; 2],
    pub palace: [PalaceState; 2],
    pub fullmove: u32,
    pub halfmove: u32,
}

impl Position {
    pub fn new() -> Self {
        let mut board     = [Piece::NONE; SQUARES];
        let mut pieces    = [[Bitboard::EMPTY; PieceKind::COUNT]; 2];
        let mut occupancy = [Bitboard::EMPTY; 2];
        let mut hash      = 0u64;

        let mut place = |sq: Square, kind: PieceKind, side: Side| {
            let piece = Piece::new(kind, side);
            board[sq.0 as usize] = piece;
            pieces[side.index()][kind.index()] |= Bitboard::from_square(sq);
            occupancy[side.index()]            |= Bitboard::from_square(sq);
            hash ^= crate::zobrist::piece_key(sq, kind, side);
        };

        // rank 0 — black back rank
        place(Square::new(0, 0),  PieceKind::Tower,    Side::Black);
        place(Square::new(0, 1),  PieceKind::Cannon,   Side::Black);
        place(Square::new(0, 2),  PieceKind::Horse,    Side::Black);
        place(Square::new(0, 3),  PieceKind::Priest,   Side::Black);
        place(Square::new(0, 4),  PieceKind::Elephant, Side::Black);
        place(Square::new(0, 5),  PieceKind::General,  Side::Black);
        place(Square::new(0, 6),  PieceKind::King,     Side::Black);
        place(Square::new(0, 7),  PieceKind::Queen,    Side::Black);
        place(Square::new(0, 8),  PieceKind::Elephant, Side::Black);
        place(Square::new(0, 9),  PieceKind::Priest,   Side::Black);
        place(Square::new(0, 10), PieceKind::Horse,    Side::Black);
        place(Square::new(0, 11), PieceKind::Cannon,   Side::Black);
        place(Square::new(0, 12), PieceKind::Tower,    Side::Black);

        // rank 1 — black carriages and archer
        place(Square::new(1, 1),  PieceKind::Carriage, Side::Black);
        place(Square::new(1, 6),  PieceKind::Archer,   Side::Black);
        place(Square::new(1, 11), PieceKind::Carriage, Side::Black);

        // rank 2 — black pawns
        for file in 0..FILES {
            place(Square::new(2, file), PieceKind::Pawn, Side::Black);
        }

        // rank 10 — white pawns
        for file in 0..FILES {
            place(Square::new(10, file), PieceKind::Pawn, Side::White);
        }

        // rank 11 — white carriages and archer
        place(Square::new(11, 1),  PieceKind::Carriage, Side::White);
        place(Square::new(11, 6),  PieceKind::Archer,   Side::White);
        place(Square::new(11, 11), PieceKind::Carriage, Side::White);

        // rank 12 — white back rank
        place(Square::new(12, 0),  PieceKind::Tower,    Side::White);
        place(Square::new(12, 1),  PieceKind::Cannon,   Side::White);
        place(Square::new(12, 2),  PieceKind::Horse,    Side::White);
        place(Square::new(12, 3),  PieceKind::Priest,   Side::White);
        place(Square::new(12, 4),  PieceKind::Elephant, Side::White);
        place(Square::new(12, 5),  PieceKind::General,  Side::White);
        place(Square::new(12, 6),  PieceKind::King,     Side::White);
        place(Square::new(12, 7),  PieceKind::Queen,    Side::White);
        place(Square::new(12, 8),  PieceKind::Elephant, Side::White);
        place(Square::new(12, 9),  PieceKind::Priest,   Side::White);
        place(Square::new(12, 10), PieceKind::Horse,    Side::White);
        place(Square::new(12, 11), PieceKind::Cannon,   Side::White);
        place(Square::new(12, 12), PieceKind::Tower,    Side::White);

        Position {
            board,
            pieces,
            occupancy,
            side_to_move: Side::White,
            hash,
            reserve:  [[0; PieceKind::COUNT]; 2],
            palace:   [PalaceState::default(); 2],
            fullmove: 1,
            halfmove: 0,
        }
    }
}