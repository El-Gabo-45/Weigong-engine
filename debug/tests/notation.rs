#[test]
fn notation_normal_move() {
    let mv = Move::new_normal(
        Square::new(12, 6),
        Square::new(11, 6),
        PieceKind::King,
        None,
        false,
    );

    assert_eq!(
        mv.notation_with(
            PieceKind::King,
            None,
            None,
            0,
            None,
            NotationTerminal::None,
            false,
        ),
        "Kg2"
    );
}

#[test]
fn notation_drop() {
    let mv = Move::new_drop(
        Square::new(6, 5),
        PieceKind::Pawn,
    );

    assert_eq!(
        mv.notation_with(
            PieceKind::Pawn,
            None,
            None,
            0,
            None,
            NotationTerminal::None,
            false,
        ),
        "p*f7"
    );
}