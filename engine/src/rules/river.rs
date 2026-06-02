use crate::types::{Square, FILES, RANKS, RIVER_RANK};

#[inline(always)]
pub fn river_step(origin: Square, d_rank: i8, d_file: i8) -> Option<Square> {
    let mut r = origin.rank() as isize + d_rank as isize;
    let     f = origin.file() as isize + d_file as isize;

    if f < 0 || f >= FILES as isize { return None; }

    if r == RIVER_RANK as isize {
        r += d_rank.signum() as isize;
    }

    if r < 0 || r >= RANKS as isize { return None; }

    Some(Square::new(r as usize, f as usize))
}