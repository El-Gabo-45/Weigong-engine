use weigong_engine::position::Position;
use weigong_engine::types::{PieceKind, Side, Square};

#[test]
fn initial_position_white_moves_first() {
    let pos = Position::new();
    assert_eq!(pos.side_to_move, Side::White);
}

#[test]
fn initial_position_fullmove_is_1() {
    let pos = Position::new();
    assert_eq!(pos.fullmove, 1);
}

#[test]
fn initial_position_halfmove_is_0() {
    let pos = Position::new();
    assert_eq!(pos.halfmove, 0);
}

#[test]
fn initial_position_reserve_is_empty() {
    let pos = Position::new();
    for side in 0..2 {
        for kind in 0..PieceKind::COUNT {
            assert_eq!(pos.reserve[side][kind], 0);
        }
    }
}

#[test]
fn initial_position_hash_nonzero() {
    let pos = Position::new();
    assert_ne!(pos.hash, 0);
}

#[test]
fn initial_position_hash_deterministic() {
    let p1 = Position::new();
    let p2 = Position::new();
    assert_eq!(p1.hash, p2.hash);
}

#[test]
fn initial_position_black_king_on_rank0_file6() {
    let pos = Position::new();
    let sq = Square::new(0, 6);
    let piece = pos.board[sq.0 as usize];
    assert!(!piece.is_none());
    assert_eq!(piece.kind(), PieceKind::King);
    assert_eq!(piece.side(), Side::Black);
}

#[test]
fn initial_position_white_king_on_rank12_file6() {
    let pos = Position::new();
    let sq = Square::new(12, 6);
    let piece = pos.board[sq.0 as usize];
    assert!(!piece.is_none());
    assert_eq!(piece.kind(), PieceKind::King);
    assert_eq!(piece.side(), Side::White);
}

#[test]
fn initial_position_black_pawns_on_rank2() {
    let pos = Position::new();
    for file in 0..13 {
        let sq = Square::new(2, file);
        let piece = pos.board[sq.0 as usize];
        assert!(!piece.is_none(), "no pawn at file {}", file);
        assert_eq!(piece.kind(), PieceKind::Pawn);
        assert_eq!(piece.side(), Side::Black);
    }
}

#[test]
fn initial_position_white_pawns_on_rank10() {
    let pos = Position::new();
    for file in 0..13 {
        let sq = Square::new(10, file);
        let piece = pos.board[sq.0 as usize];
        assert!(!piece.is_none(), "no pawn at file {}", file);
        assert_eq!(piece.kind(), PieceKind::Pawn);
        assert_eq!(piece.side(), Side::White);
    }
}

#[test]
fn initial_position_river_is_empty() {
    let pos = Position::new();
    for file in 0..13 {
        let sq = Square::new(6, file);
        assert!(pos.board[sq.0 as usize].is_none(), "river square at file {} is not empty", file);
    }
}

#[test]
fn initial_position_occupancy_matches_board() {
    let pos = Position::new();
    let mut black_count = 0u32;
    let mut white_count = 0u32;
    for sq in 0..169 {
        let piece = pos.board[sq];
        if !piece.is_none() {
            match piece.side() {
                Side::Black => black_count += 1,
                Side::White => white_count += 1,
            }
        }
    }
    assert_eq!(pos.occupancy[Side::Black.index()].popcount(), black_count);
    assert_eq!(pos.occupancy[Side::White.index()].popcount(), white_count);
}

#[test]
fn initial_position_total_pieces_is_58() {
    let pos = Position::new();
    // 29 pieces per side = 58 total
    let total = pos.occupancy[0].popcount() + pos.occupancy[1].popcount();
    assert_eq!(total, 58);
}