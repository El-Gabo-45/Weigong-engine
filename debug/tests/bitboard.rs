#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_is_empty() {
        assert!(Bitboard::EMPTY.is_empty());
        assert_eq!(Bitboard::EMPTY.popcount(), 0);
    }

    #[test]
    fn square_roundtrip() {
        for i in 0..SQUARES {
            let sq = Square(i as u8);
            let bb = Bitboard::from_square(sq);
            assert_eq!(bb.popcount(), 1);
            assert!(bb.contains(sq));
            assert!(!bb.contains(Square(((i + 1) % SQUARES) as u8)));
        }
    }

    #[test]
    fn rank_mask_has_13_bits() {
        for rank in 0..RANKS {
            assert_eq!(rank_bb(rank).popcount(), FILES as u32,
                "rank {} should have 13 squares", rank);
        }
    }

    #[test]
    fn file_mask_has_13_bits() {
        for file in 0..FILES {
            assert_eq!(file_bb(file).popcount(), RANKS as u32,
                "file {} should have 13 squares", file);
        }
    }

    #[test]
    fn all_squares_covered() {
        let mut all = Bitboard::EMPTY;
        for rank in 0..RANKS {
            all |= rank_bb(rank);
        }
        assert_eq!(all.popcount(), SQUARES as u32);
    }

    #[test]
    fn palace_has_9_squares() {
        assert_eq!(BLACK_PALACE.popcount(), 9);
        assert_eq!(WHITE_PALACE.popcount(), 9);
    }

    #[test]
    fn palaces_do_not_overlap() {
        assert!((BLACK_PALACE & WHITE_PALACE).is_empty());
    }

    #[test]
    fn shift_south_moves_rank() {
        let rank0 = rank_bb(0);
        let shifted = rank0.shift_south();
        assert_eq!(shifted, rank_bb(1));
    }

    #[test]
    fn shift_north_moves_rank() {
        let rank5 = rank_bb(5);
        let shifted = rank5.shift_north();
        assert_eq!(shifted, rank_bb(4));
    }

    #[test]
    fn shift_east_no_wrap() {
        // Square in file 12 (white border) shouldn't appear in file 0 when shifting east
        let bb = Bitboard::from_square(Square::new(0, 12));
        let shifted = bb.shift_east();
        assert!(shifted.is_empty(), "shift_east en file 12 debe dar vacío");
    }

    #[test]
    fn shift_west_no_wrap() {
        let bb = Bitboard::from_square(Square::new(0, 0));
        let shifted = bb.shift_west();
        assert!(shifted.is_empty(), "shift_west en file 0 debe dar vacío");
    }

    #[test]
    fn pop_lsb_iterates_all() {
        let mut bb = BLACK_PALACE;
        let mut count = 0;
        while let Some(_sq) = bb.pop_lsb() { count += 1; }
        assert_eq!(count, 9);
        assert!(bb.is_empty());
    }

    #[test]
    fn sides_do_not_overlap_with_river() {
        // BLACK_SIDE, RIVER, WHITE_SIDE are disjoint and cover all squares
        let total = (BLACK_SIDE | RIVER | WHITE_SIDE).popcount();
        assert_eq!(total, SQUARES as u32);
        assert!((BLACK_SIDE & WHITE_SIDE).is_empty());
        assert!((BLACK_SIDE & RIVER).is_empty());
        assert!((WHITE_SIDE & RIVER).is_empty());
    }
}