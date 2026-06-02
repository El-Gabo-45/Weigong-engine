// Bitboard para Weigong — 13×13 board (169 squares)

// A u128 + u64 = 192 bits, of which we use 169.
// Squares 0-127   → stored in `lo: u128`
// Squares 128-168 → stored in `hi: u64` (bits 0-40)

// This representation allows for efficient bit-wise operations
// and is compatible with SIMD on x86_64 via future intrinsics.

use crate::types::{Square, Side, FILES, RANKS, SQUARES, RIVER_RANK, PALACE_FILE_MIN, PALACE_FILE_MAX, BLACK_PALACE_RANK_MAX, WHITE_PALACE_RANK_MIN};

//  Bitboard

// Set of squares represented as two words: 128 + 64 bits.
// The squares 0-127 go in `lo`, the squares 128-168 go in `hi`.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Bitboard {
    pub lo: u128,
    pub hi: u64,
}

impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard { lo: 0, hi: 0 };

    /// Mask of valid bits in `hi` (squares 128-168 → 41 bits)
    const HI_MASK: u64 = (1u64 << 41) - 1;

    // Construction

    #[inline(always)]
    pub fn from_square(sq: Square) -> Self {
        let idx = sq.0 as usize;
        if idx < 128 {
            Bitboard { lo: 1u128 << idx, hi: 0 }
        } else {
            Bitboard { lo: 0, hi: 1u64 << (idx - 128) }
        }
    }

    #[inline(always)]
    pub fn is_empty(self) -> bool { self.lo == 0 && self.hi == 0 }

    #[inline(always)]
    pub fn is_not_empty(self) -> bool { !self.is_empty() }

    // Verify if a holds the given square
    #[inline(always)]
    pub fn contains(self, sq: Square) -> bool {
        (self & Bitboard::from_square(sq)).is_not_empty()
    }

    // Bits operations

    #[inline(always)]
    pub fn popcount(self) -> u32 {
        self.lo.count_ones() + self.hi.count_ones()
    }

    // Extracts and removes the least significant bit (LSB)
    // Returns None if the bitboard is empty

    #[inline(always)]
    pub fn pop_lsb(&mut self) -> Option<Square> {
        if self.lo != 0 {
            let idx = self.lo.trailing_zeros() as usize;
            self.lo &= self.lo - 1;
            Some(Square(idx as u8))
        } else if self.hi != 0 {
            let idx = self.hi.trailing_zeros() as usize;
            self.hi &= self.hi - 1;
            Some(Square((128 + idx) as u8))
        } else {
            None
        }
    }

    // Square of the least significant bit without removing it

    #[inline(always)]
    pub fn lsb(self) -> Option<Square> {
        if self.lo != 0 {
            Some(Square(self.lo.trailing_zeros() as u8))
        } else if self.hi != 0 {
            Some(Square((128 + self.hi.trailing_zeros()) as u8))
        } else {
            None
        }
    }

    // Shifts

    // The board is 13 columns wide. Shifting N rows = shifting N*13 bits
    // Shifting 1 column = shifting 1 bit, but the edge column must be masked
    // to prevent wrapping around to the other side

    /// Shifts the bitboard 1 row down (rank+1, towards the white side)
    /// Equivalent to shifting 13 bits up

    #[inline(always)]
    pub fn shift_south(self) -> Self {
        // South = higher ranks = higher indices = shift left in bits
        let lo_shifted = self.lo << 13;
        // The bits that overflow from lo (bits 115-127) move to hi
        let carry = (self.lo >> 115) as u64;
        let hi_shifted = ((self.hi << 13) | carry) & Self::HI_MASK;
        Bitboard { lo: lo_shifted, hi: hi_shifted }
    }

    // Shifts the bitboard 1 row up (rank-1, towards the black side)
    
    #[inline(always)]
    pub fn shift_north(self) -> Self {
        let hi_shifted = self.hi >> 13;
        // The low bits of hi (0-12) move to the high bits of lo
        let carry = (self.hi & 0x1FFF) as u128;
        let lo_shifted = (self.lo >> 13) | (carry << 115);
        Bitboard { lo: lo_shifted, hi: hi_shifted }
    }

    // Shifts 1 column to the right (file+1), masking the right edge (file 12)
    
    #[inline(always)]
    pub fn shift_east(self) -> Self {
        (Bitboard { lo: self.lo << 1, hi: ((self.hi << 1) | (self.lo >> 127)) & Self::HI_MASK })
            & NOT_FILE_A // file 0 can't appear to the east
    }

    // Shifts 1 column to the left (file-1), masking the left edge (file 0)

    #[inline(always)]
    pub fn shift_west(self) -> Self {
        (Bitboard { lo: self.lo >> 1, hi: self.hi >> 1 }) & NOT_FILE_M // file 12 can't appear to the west
    }

    // Iterator

    // Iterates over active squares

    #[inline(always)]
    pub fn iter(self) -> BitboardIter { BitboardIter(self) }
}

//  Operators bit to bit

impl std::ops::BitAnd for Bitboard {
    type Output = Self;
    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self {
        Bitboard { lo: self.lo & rhs.lo, hi: self.hi & rhs.hi }
    }
}

impl std::ops::BitOr for Bitboard {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self {
        Bitboard { lo: self.lo | rhs.lo, hi: self.hi | rhs.hi }
    }
}

impl std::ops::BitXor for Bitboard {
    type Output = Self;
    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self {
        Bitboard { lo: self.lo ^ rhs.lo, hi: self.hi ^ rhs.hi }
    }
}

impl std::ops::Not for Bitboard {
    type Output = Self;
    #[inline(always)]
    fn not(self) -> Self {
        Bitboard { lo: !self.lo, hi: (!self.hi) & Bitboard::HI_MASK }
    }
}

impl std::ops::BitAndAssign for Bitboard {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) { self.lo &= rhs.lo; self.hi &= rhs.hi; }
}

impl std::ops::BitOrAssign for Bitboard {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) { self.lo |= rhs.lo; self.hi |= rhs.hi; }
}

impl std::ops::BitXorAssign for Bitboard {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) { self.lo ^= rhs.lo; self.hi ^= rhs.hi; }
}

impl std::fmt::Debug for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Bitboard {{")?;
        for rank in 0..RANKS {
            write!(f, "  rank {:2} |", rank)?;
            for file in 0..FILES {
                let sq = Square::new(rank, file);
                write!(f, "{}", if self.contains(sq) { "X" } else { "." })?;
            }
            writeln!(f, "|")?;
        }
        write!(f, "}}")
    }
}

//  Iterador

pub struct BitboardIter(Bitboard);

impl Iterator for BitboardIter {
    type Item = Square;
    #[inline(always)]
    fn next(&mut self) -> Option<Square> { self.0.pop_lsb() }
}

//  Static masks precalculated

// Generated at compile time with const fn
// Prevent repetitive calculations at search time

/// Mask of a single square
pub fn square_bb(sq: Square) -> Bitboard { Bitboard::from_square(sq) }

/// Mask of entire row (rank)
pub const fn rank_bb(rank: usize) -> Bitboard {
    // 13 consecutive bits starting at rank*13
    let start = rank * FILES;
    if start < 128 {
        let end = start + FILES; // = start + 13
        if end <= 128 {
            Bitboard { lo: ((1u128 << FILES) - 1) << start, hi: 0 }
        } else {
            // The file crosses the lo/hi boundary
            let lo_bits = !0u128 << start; // bits from start till 127
            let hi_bits = (1u64 << (end - 128)) - 1;
            Bitboard { lo: lo_bits, hi: hi_bits }
        }
    } else {
        Bitboard { lo: 0, hi: (((1u64 << FILES) - 1) << (start - 128)) & Bitboard::HI_MASK }
    }
}

// Mask of entire column (file)
pub const fn file_bb(file: usize) -> Bitboard {
    // Squares of file: file, file+13, file+26, ..., file+156
    // Total: 13 squares
    let mut lo: u128 = 0;
    let mut hi: u64  = 0;
    let mut rank = 0usize;
    while rank < RANKS {
        let idx = rank * FILES + file;
        if idx < 128 {
            lo |= 1u128 << idx;
        } else {
            hi |= 1u64 << (idx - 128);
        }
        rank += 1;
    }
    Bitboard { lo, hi }
}

// Masks of border columns used to avoid wrap in shifts
const FILE_A_MASK: Bitboard = file_bb(0);   // column a (file 0)
const FILE_M_MASK: Bitboard = file_bb(12);  // column m (file 12)

// Everything except column a — for shift_east
pub const NOT_FILE_A: Bitboard = Bitboard {
    lo: !FILE_A_MASK.lo,
    hi: (!FILE_A_MASK.hi) & Bitboard::HI_MASK,
};

// Everything except column m for shift_west
pub const NOT_FILE_M: Bitboard = Bitboard {
    lo: !FILE_M_MASK.lo,
    hi: (!FILE_M_MASK.hi) & Bitboard::HI_MASK,
};

// River (rank 6)
pub const RIVER: Bitboard = rank_bb(RIVER_RANK);

// Black side (ranks 0-5)
pub const BLACK_SIDE: Bitboard = {
    let mut bb = Bitboard::EMPTY;
    let mut r = 0;
    while r < RIVER_RANK {
        let rank = rank_bb(r);
        bb.lo |= rank.lo;
        bb.hi |= rank.hi;
        r += 1;
    }
    bb
};

// White side (ranks 7-12)
pub const WHITE_SIDE: Bitboard = {
    let mut bb = Bitboard::EMPTY;
    let mut r = RIVER_RANK + 1;
    while r < RANKS {
        let rank = rank_bb(r);
        bb.lo |= rank.lo;
        bb.hi |= rank.hi;
        r += 1;
    }
    bb
};

// Black palace (ranks 0-2, files 4-6)
pub const BLACK_PALACE: Bitboard = {
    let mut bb = Bitboard::EMPTY;
    let mut r = 0;
    while r <= BLACK_PALACE_RANK_MAX {
        let mut f = PALACE_FILE_MIN;
        while f <= PALACE_FILE_MAX {
            let idx = r * FILES + f;
            if idx < 128 { bb.lo |= 1u128 << idx; }
            else          { bb.hi |= 1u64  << (idx - 128); }
            f += 1;
        }
        r += 1;
    }
    bb
};

// White palace (ranks 10-12, files 4-6)
pub const WHITE_PALACE: Bitboard = {
    let mut bb = Bitboard::EMPTY;
    let mut r = WHITE_PALACE_RANK_MIN;
    while r < RANKS {
        let mut f = PALACE_FILE_MIN;
        while f <= PALACE_FILE_MAX {
            let idx = r * FILES + f;
            if idx < 128 { bb.lo |= 1u128 << idx; }
            else          { bb.hi |= 1u64  << (idx - 128); }
            f += 1;
        }
        r += 1;
    }
    bb
};

// Promotion zone for each side (the last 3 rows of the enemy territory)
pub const BLACK_PROMOTION_ZONE: Bitboard = {
    let mut bb = Bitboard::EMPTY;
    let mut r = WHITE_PALACE_RANK_MIN; // ranks 10-12
    while r < RANKS {
        let rank = rank_bb(r);
        bb.lo |= rank.lo;
        bb.hi |= rank.hi;
        r += 1;
    }
    bb
};

pub const WHITE_PROMOTION_ZONE: Bitboard = {
    let mut bb = Bitboard::EMPTY;
    let mut r = 0;
    while r <= BLACK_PALACE_RANK_MAX { // ranks 0-2
        let rank = rank_bb(r);
        bb.lo |= rank.lo;
        bb.hi |= rank.hi;
        r += 1;
    }
    bb
};

// River bank for each side (rank 5 for black, rank 7 for white)
pub const BLACK_BANK: Bitboard = rank_bb(RIVER_RANK - 1); // rank 5
pub const WHITE_BANK: Bitboard = rank_bb(RIVER_RANK + 1); // rank 7

// Conveniencia: banco según el bando
pub fn bank_bb(side: Side) -> Bitboard {
    match side { Side::Black => BLACK_BANK, Side::White => WHITE_BANK }
}

pub fn palace_bb(side: Side) -> Bitboard {
    match side { Side::Black => BLACK_PALACE, Side::White => WHITE_PALACE }
}

pub fn promotion_zone_bb(side: Side) -> Bitboard {
    match side { Side::Black => BLACK_PROMOTION_ZONE, Side::White => WHITE_PROMOTION_ZONE }
}

pub fn own_side_bb(side: Side) -> Bitboard {
    match side { Side::Black => BLACK_SIDE, Side::White => WHITE_SIDE }
}