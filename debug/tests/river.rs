use weigong_engine::rules::river::river_step;
use weigong_engine::types::Square;

#[test]
fn pawn_forward_black_crosses_river() {
    let origin = Square::new(5, 6);
    let dest = river_step(origin, 1, 0).unwrap();
    assert_eq!(dest.rank(), 7);
    assert_eq!(dest.file(), 6);
}

#[test]
fn pawn_forward_white_crosses_river() {
    let origin = Square::new(7, 6);
    let dest = river_step(origin, -1, 0).unwrap();
    assert_eq!(dest.rank(), 5);
    assert_eq!(dest.file(), 6);
}

#[test]
fn pawn_capture_diagonal_black_crosses_river() {
    let origin = Square::new(5, 4);
    let dest = river_step(origin, 1, 1).unwrap();
    assert_eq!(dest.rank(), 7);
    assert_eq!(dest.file(), 5);
}

#[test]
fn pawn_capture_diagonal_white_crosses_river() {
    let origin = Square::new(7, 8);
    let dest = river_step(origin, -1, -1).unwrap();
    assert_eq!(dest.rank(), 5);
    assert_eq!(dest.file(), 7);
}

#[test]
fn crossbow_all_8_from_black_bank() {
    let origin = Square::new(5, 6);
    assert_eq!(river_step(origin,  1,  0).unwrap().rank(), 7);
    assert_eq!(river_step(origin, -1,  0).unwrap().rank(), 4);
    assert_eq!(river_step(origin,  0,  1).unwrap().rank(), 5);
    assert_eq!(river_step(origin,  0, -1).unwrap().rank(), 5);
    assert_eq!(river_step(origin,  1,  1).unwrap().rank(), 7);
    assert_eq!(river_step(origin,  1, -1).unwrap().rank(), 7);
    assert_eq!(river_step(origin, -1,  1).unwrap().rank(), 4);
    assert_eq!(river_step(origin, -1, -1).unwrap().rank(), 4);
}

#[test]
fn crossbow_all_8_from_white_bank() {
    let origin = Square::new(7, 6);
    assert_eq!(river_step(origin, -1,  0).unwrap().rank(), 5);
    assert_eq!(river_step(origin,  1,  0).unwrap().rank(), 8);
    assert_eq!(river_step(origin,  0,  1).unwrap().rank(), 7);
    assert_eq!(river_step(origin,  0, -1).unwrap().rank(), 7);
    assert_eq!(river_step(origin, -1,  1).unwrap().rank(), 5);
    assert_eq!(river_step(origin, -1, -1).unwrap().rank(), 5);
    assert_eq!(river_step(origin,  1,  1).unwrap().rank(), 8);
    assert_eq!(river_step(origin,  1, -1).unwrap().rank(), 8);
}

#[test]
fn fortress_forward_crosses_river() {
    let origin = Square::new(5, 3);
    let dest = river_step(origin, 1, 0).unwrap();
    assert_eq!(dest.rank(), 7);
}

#[test]
fn fortress_backward_2_skips_river() {
    let origin = Square::new(8, 3);
    let dest = river_step(origin, -2, 0).unwrap();
    assert_eq!(dest.rank(), 5);
}

#[test]
fn step_off_board_returns_none() {
    assert!(river_step(Square::new(0,  6), -1,  0).is_none()); // top
    assert!(river_step(Square::new(12, 6),  1,  0).is_none()); // bottom
    assert!(river_step(Square::new(5,  0),  0, -1).is_none()); // left
    assert!(river_step(Square::new(5, 12),  0,  1).is_none()); // right
}

#[test]
fn normal_step_unaffected_by_river() {
    let origin = Square::new(3, 6);
    let dest = river_step(origin, 1, 0).unwrap();
    assert_eq!(dest.rank(), 4);
}