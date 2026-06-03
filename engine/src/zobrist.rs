use crate::types::{Square, Side, PieceKind, SQUARES};
use std::sync::OnceLock;
use rand::{SeedableRng, rngs::StdRng, Rng};

const TABLE_SIZE: usize = SQUARES * PieceKind::COUNT * 2
                        + PieceKind::COUNT * 2
                        + 1;

static ZOBRIST_TABLE: OnceLock<[u64; TABLE_SIZE]> = OnceLock::new();

pub fn table() -> &'static [u64; TABLE_SIZE] {
    ZOBRIST_TABLE.get_or_init(|| {
        let mut rng = StdRng::seed_from_u64(0x5765_6967_6F6E_6700); // "Weigong"
        let mut table = [0u64; TABLE_SIZE];
        for i in 0..TABLE_SIZE {
            table[i] = rng.gen::<u64>();
        }
        table
    })
}

pub fn piece_key(sq: Square, piece: PieceKind, side: Side) -> u64 {
    let idx = sq.0 as usize * PieceKind::COUNT * 2
            + piece as usize * 2
            + side as usize;
    table()[idx]
}

pub fn reserve_key(piece: PieceKind, side: Side) -> u64 {
    let idx = SQUARES * PieceKind::COUNT * 2
            + piece as usize * 2
            + side as usize;
    table()[idx]
}

pub fn side_key() -> u64 { table()[TABLE_SIZE - 1] }