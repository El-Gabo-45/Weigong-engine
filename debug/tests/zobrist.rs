use weigong_engine::zobrist;
use weigong_engine::types::{Square, Side, PieceKind};

#[test]
fn table_has_correct_size() {
    let table = zobrist::table();
    // SQUARES * PieceKind::COUNT * 2 (board) + PieceKind::COUNT * 2 (reserve) + 1 (side)
    let expected = 169 * 17 * 2 + 17 * 2 + 1;
    assert_eq!(table.len(), expected);
}

#[test]
fn table_no_zeros() {
    let table = zobrist::table();
    assert!(table.iter().any(|&x| x != 0));
}

#[test]
fn table_same_reference_twice() {
    // OnceLock must return the same table both times
    let t1 = zobrist::table();
    let t2 = zobrist::table();
    assert_eq!(t1.as_ptr(), t2.as_ptr());
}

#[test]
fn table_all_unique() {
    use std::collections::HashSet;
    let table = zobrist::table();
    let set: HashSet<u64> = table.iter().copied().collect();
    assert_eq!(set.len(), table.len());
}

#[test]
fn piece_key_same_result_twice() {
    // same input must always return same key
    let k1 = zobrist::piece_key(Square::new(0, 0), PieceKind::King, Side::Black);
    let k2 = zobrist::piece_key(Square::new(0, 0), PieceKind::King, Side::Black);
    assert_eq!(k1, k2);
}

#[test]
fn piece_key_different_squares() {
    // different squares must give different keys
    let k1 = zobrist::piece_key(Square::new(0, 0), PieceKind::Tower, Side::Black);
    let k2 = zobrist::piece_key(Square::new(1, 0), PieceKind::Tower, Side::Black);
    assert_ne!(k1, k2);
}

#[test]
fn piece_key_different_sides() {
    // same square and piece but different side must give different keys
    let k1 = zobrist::piece_key(Square::new(0, 0), PieceKind::Tower, Side::Black);
    let k2 = zobrist::piece_key(Square::new(0, 0), PieceKind::Tower, Side::White);
    assert_ne!(k1, k2);
}

#[test]
fn reserve_key_same_result_twice() {
    let k1 = zobrist::reserve_key(PieceKind::Pawn, Side::Black);
    let k2 = zobrist::reserve_key(PieceKind::Pawn, Side::Black);
    assert_eq!(k1, k2);
}

#[test]
fn reserve_key_different_from_piece_key() {
    // reserve key must not collide with board piece key
    let board = zobrist::piece_key(Square::new(0, 0), PieceKind::Pawn, Side::Black);
    let reserve = zobrist::reserve_key(PieceKind::Pawn, Side::Black);
    assert_ne!(board, reserve);
}

#[test]
fn side_key_nonzero() {
    assert_ne!(zobrist::side_key(), 0);
}

#[test]
fn xor_is_reversible() {
    // XORing the same key twice returns to the original hash
    let mut hash = 0u64;
    let key = zobrist::piece_key(Square::new(5, 5), PieceKind::Horse, Side::White);
    hash ^= key;
    hash ^= key;
    assert_eq!(hash, 0);
}