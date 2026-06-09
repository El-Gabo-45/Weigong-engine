use crate::types::*;

// Notation terminal

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NotationTerminal {
    #[default]
    None,
    Checkmate,
    PalaceMate,
    Stalemate,
    Draw,
    DrawAgreement,
    MoveLimitDraw,
}

impl NotationTerminal {
    #[inline(always)]
    pub fn as_str(self) -> &'static str {
        match self {
            NotationTerminal::None          => "",
            NotationTerminal::Checkmate     => "#",
            NotationTerminal::PalaceMate    => "##",
            NotationTerminal::Stalemate     => "^",
            NotationTerminal::Draw          => "=",
            NotationTerminal::DrawAgreement => "==",
            NotationTerminal::MoveLimitDraw => "/",
        }
    }
}

// Archer move notation

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArcherOption {
    pub piece: PieceKind,
    pub square: Square,
    pub retreat_to: Option<Square>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArcherNotation {
    SingleCapture {
        victim: PieceKind,
        victim_sq: Square,
    },
    AutoCaptureAll {
        victims: Vec<(PieceKind, Square)>,
    },
    ChooseCapture {
        chosen_index: usize,
        options: Vec<ArcherOption>,
    },
}

// Palace curse notation

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PalaceCurseNotation {
    pub groups: Vec<Vec<(PieceKind, Square)>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MoveNotationContext {
    pub archer:   Option<ArcherNotation>,
    pub curse:    Option<PalaceCurseNotation>,
    pub terminal: NotationTerminal,
    pub check:    bool,
}

// Piece symbols

#[inline(always)]
fn piece_symbol(kind: PieceKind) -> &'static str {
    match kind {
        PieceKind::King      => "K",
        PieceKind::Queen     => "Q",
        PieceKind::General   => "G",
        PieceKind::Elephant  => "E",
        PieceKind::Priest    => "P",
        PieceKind::Horse     => "H",
        PieceKind::Cannon    => "C",
        PieceKind::Tower     => "T",
        PieceKind::Carriage  => "Ca",
        PieceKind::Archer    => "A",
        PieceKind::Pawn      => "p",
        PieceKind::Fortress  => "F",
        PieceKind::Wisdom    => "W",
        PieceKind::Steed     => "S",
        PieceKind::Artillery => "R",
        PieceKind::Barricade => "U",
        PieceKind::Crossbow  => "B",
    }
}

#[inline(always)]
fn piece_symbol_lower(kind: PieceKind) -> String {
    piece_symbol(kind).to_ascii_lowercase()
}

// Palace curse suffix

pub fn append_curse_notation(curse: &PalaceCurseNotation) -> String {
    let mut suffix = String::new();
    for group in &curse.groups {
        if group.is_empty() { continue; }
        suffix.push('&');
        for (i, (piece, sq)) in group.iter().enumerate() {
            if i > 0 { suffix.push('&'); }
            suffix.push_str(piece_symbol(*piece));
            suffix.push_str(&sq.to_string());
        }
        suffix.push('-');
    }
    suffix
}

// Core notation builder

pub fn generate_move_notation(
    mv: Move,
    moved_piece_after: PieceKind,
    captured_piece: Option<PieceKind>,
    ambush: Option<&ArcherNotation>,
    _chosen_ambush_index: usize,
    curse: Option<&PalaceCurseNotation>,
    terminal: NotationTerminal,
    check: bool,
) -> String {
    let mut s = String::new();
    let to_str = mv.to_sq().to_string();

    if mv.is_drop() {
        s.push_str(piece_symbol(moved_piece_after.base_form()));
        s.push('*');
        s.push_str(&to_str);
    } else if let Some(ambush_info) = ambush {
        s.push('A');
        s.push_str(&to_str);

        match ambush_info {
            ArcherNotation::SingleCapture { victim, victim_sq } => {
                s.push('>');
                s.push_str(piece_symbol(*victim));
                s.push('x');
                s.push_str(&victim_sq.to_string());
            }

            ArcherNotation::AutoCaptureAll { victims } => {
                if !victims.is_empty() {
                    s.push('>');
                    for (i, (victim, sq)) in victims.iter().enumerate() {
                        if i > 0 { s.push(','); }
                        s.push_str(piece_symbol(*victim));
                        s.push('x');
                        s.push_str(&sq.to_string());
                    }
                }
            }

            ArcherNotation::ChooseCapture { chosen_index, options } => {
                if !options.is_empty() && *chosen_index < options.len() {
                    let chosen = options[*chosen_index];
                    s.push('>');
                    s.push_str(piece_symbol(chosen.piece));
                    s.push('x');
                    s.push_str(&chosen.square.to_string());

                    for (i, opt) in options.iter().enumerate() {
                        if i == *chosen_index { continue; }
                        s.push(',');
                        s.push_str(piece_symbol(opt.piece));
                        if let Some(retreat_to) = opt.retreat_to {
                            s.push_str(&opt.square.to_string());
                            s.push('→');
                            s.push_str(&retreat_to.to_string());
                        } else {
                            s.push('x');
                            s.push_str(&opt.square.to_string());
                        }
                    }
                }
            }
        }
    } else {
        s.push_str(piece_symbol(moved_piece_after));
        if let Some(target) = captured_piece {
            s.push('x');
            s.push_str(&piece_symbol_lower(target));
        }
        s.push_str(&to_str);
        if mv.is_promotion() { s.push('+'); }
    }

    // Terminal takes priority over plain check
    if terminal != NotationTerminal::None {
        s.push_str(terminal.as_str());
    } else if check {
        s.push('%');
    }

    if let Some(curse) = curse {
        s.push_str(&append_curse_notation(curse));
    }

    s
}

// Convenience method on Move

impl Move {
    pub fn notation_with(
        self,
        moved_piece_after: PieceKind,
        captured_piece: Option<PieceKind>,
        ambush: Option<&ArcherNotation>,
        chosen_ambush_index: usize,
        curse: Option<&PalaceCurseNotation>,
        terminal: NotationTerminal,
        check: bool,
    ) -> String {
        generate_move_notation(
            self,
            moved_piece_after,
            captured_piece,
            ambush,
            chosen_ambush_index,
            curse,
            terminal,
            check,
        )
    }
}