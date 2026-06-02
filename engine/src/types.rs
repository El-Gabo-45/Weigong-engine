pub const FILES: usize = 13;
pub const RANKS: usize = 13;
pub const SQUARES: usize = FILES * RANKS; // 169 squares

// No piece can occupy the river rank
pub const RIVER_RANK: usize = 6;

// Columns of the palace (0-indexed: columns 5, 6, 7)
pub const PALACE_FILE_MIN: usize = 5;
pub const PALACE_FILE_MAX: usize = 7;

// Rows of the black palace (top): ranks 0-2
pub const BLACK_PALACE_RANK_MIN: usize = 0;
pub const BLACK_PALACE_RANK_MAX: usize = 2;

// Rows of the white palace (bottom): ranks 10-12
pub const WHITE_PALACE_RANK_MIN: usize = 10;
pub const WHITE_PALACE_RANK_MAX: usize = 12;

// A square on the board, represented as an index 0..168.
// sq = rank * 13 + file
// rank 0 = top row (black side), rank 12 = bottom row (white side)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Square(pub u8);

impl Square {
    pub const NONE: Square = Square(u8::MAX);

    #[inline(always)]
    pub fn new(rank: usize, file: usize) -> Self {
        debug_assert!(rank < RANKS && file < FILES);
        Square((rank * FILES + file) as u8)
    }

    #[inline(always)]
    pub fn rank(self) -> usize { (self.0 as usize) / FILES }

    #[inline(always)]
    pub fn file(self) -> usize { (self.0 as usize) % FILES }

    #[inline(always)]
    pub fn is_valid(self) -> bool { (self.0 as usize) < SQUARES }

    // The square is on the own side of the river for the given side
    #[inline(always)]
    pub fn is_own_side(self, side: Side) -> bool {
        match side {
            Side::Black => self.rank() < RIVER_RANK,
            Side::White => self.rank() > RIVER_RANK,
        }
    }

    // The square is on the bank of the river for the given side
    // The bank is the row immediately before the river.
    #[inline(always)]
    pub fn is_bank(self, side: Side) -> bool {
        match side {
            Side::Black => self.rank() == RIVER_RANK - 1, // rank 5
            Side::White => self.rank() == RIVER_RANK + 1, // rank 7
        }
    }

    // Can ignore the river
    #[inline(always)]
    pub fn river_step(origin: Square, d_rank: i8, d_file: i8) -> Option<Square> {
        let mut r = origin.rank() as isize + d_rank as isize;
        let     f = origin.file() as isize + d_file as isize;

        // Borde lateral
        if f < 0 || f >= FILES as isize { return None; }

        if r == RIVER_RANK as isize {
            r += d_rank.signum() as isize;
        }

        if r < 0 || r >= RANKS as isize { return None; }

        Some(Square::new(r as usize, f as usize))
    }

    // The square is in the palace of the given side
    #[inline(always)]
    pub fn is_palace(self, side: Side) -> bool {
        let f = self.file();
        let r = self.rank();
        if f < PALACE_FILE_MIN || f > PALACE_FILE_MAX { return false; }
        match side {
            Side::Black => r <= BLACK_PALACE_RANK_MAX,
            Side::White => r >= WHITE_PALACE_RANK_MIN,
        }
    }

    // Promotion zone: the last 3 ranks on the opponent's side of the river
    #[inline(always)]
    pub fn is_promotion_zone(self, side: Side) -> bool {
        match side {
            Side::Black => self.rank() >= WHITE_PALACE_RANK_MIN, // ranks 10-12
            Side::White => self.rank() <= BLACK_PALACE_RANK_MAX, // ranks 0-2
        }
    }
}

impl std::fmt::Display for Square {
    // WGN notation: column as letter (a-m), row as number (1-13)
    // file 0 = 'a', rank 0 = '13' (like in chess, from the black side)
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file_char = (b'a' + self.file() as u8) as char;
        let rank_num  = RANKS - self.rank(); // rank 0 → "13", rank 12 → "1"
        write!(f, "{}{}", file_char, rank_num)
    }
}

// Side

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Side {
    Black = 0, // Move first, occupies ranks 0-5
    White = 1, // Move second, occupies ranks 7-12
}

impl Side {
    #[inline(always)]
    pub fn opponent(self) -> Self {
        match self {
            Side::Black => Side::White,
            Side::White => Side::Black,
        }
    }

    // Advance direction: black moves to high ranks (+1), white moves to low ranks (-1)

    #[inline(always)]
    pub fn forward(self) -> i8 {
        match self {
            Side::Black => 1,
            Side::White => -1,
        }
    }

    #[inline(always)]
    pub fn index(self) -> usize { self as usize }
}

impl std::fmt::Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self { Side::Black => write!(f, "black"), Side::White => write!(f, "white") }
    }
}

//  PieceKind

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PieceKind {
    // Base pieces
    King      = 0,  // 王
    Queen     = 1,  // 后
    General   = 2,  // 師
    Elephant  = 3,  // 象
    Priest    = 4,  // 仙
    Horse     = 5,  // 馬
    Cannon    = 6,  // 炮
    Tower     = 7,  // 塔
    Carriage  = 8,  // 輦
    Archer    = 9,  // 矢
    Pawn      = 10, // 兵

    // Promoted pieces
    Fortress    = 11, // 毅 (象 promoted)
    Wisdom      = 12, // 叡 (仙 promoted)
    Steed       = 13, // 駿 (馬 promoted)
    Artillery   = 14, // 熕 (炮 promoted)
    Barricade   = 15, // 𨐌 (塔 promoted
    Crossbow    = 16, // 弩 (兵 promoted)
}

impl PieceKind {
    pub const COUNT: usize = 17;
    pub const BASE_COUNT: usize = 11; // Pieces without promoted forms

    pub fn kanji(self) -> &'static str {
        match self {
            PieceKind::King      => "王",
            PieceKind::Queen     => "后",
            PieceKind::General   => "師",
            PieceKind::Elephant  => "象",
            PieceKind::Priest    => "仙",
            PieceKind::Horse     => "馬",
            PieceKind::Cannon    => "炮",
            PieceKind::Tower     => "塔",
            PieceKind::Carriage  => "輦",
            PieceKind::Archer    => "矢",
            PieceKind::Pawn      => "兵",
            PieceKind::Fortress  => "毅",
            PieceKind::Wisdom    => "叡",
            PieceKind::Steed     => "駿",
            PieceKind::Artillery => "熕",
            PieceKind::Barricade => "𨐌",
            PieceKind::Crossbow  => "弩",
        }
    }

    // Piece can be dropped from reserve after being captured
    pub fn is_reusable(self) -> bool {
        matches!(self,
            PieceKind::Tower    |
            PieceKind::General  |
            PieceKind::Pawn     |
            PieceKind::Crossbow
        )
    }

    // Pieces that can be promoted when moving into the promotion zone
    pub fn can_promote(self) -> bool {
        matches!(self,
            PieceKind::Elephant |
            PieceKind::Priest   |
            PieceKind::Horse    |
            PieceKind::Cannon   |
            PieceKind::Tower    |
            PieceKind::Pawn
        )
    }

    // Promoted form of a piece (if it can promote)
    pub fn promoted_form(self) -> Option<PieceKind> {
        match self {
            PieceKind::Elephant => Some(PieceKind::Fortress),
            PieceKind::Priest   => Some(PieceKind::Wisdom),
            PieceKind::Horse    => Some(PieceKind::Steed),
            PieceKind::Cannon   => Some(PieceKind::Artillery),
            PieceKind::Tower    => Some(PieceKind::Barricade),
            PieceKind::Pawn     => Some(PieceKind::Crossbow),
            _ => None,
        }
    }

    // Base form of a promoted piece (for drops from reserve)
    pub fn base_form(self) -> PieceKind {
        match self {
            PieceKind::Fortress  => PieceKind::Elephant,
            PieceKind::Wisdom    => PieceKind::Priest,
            PieceKind::Steed     => PieceKind::Horse,
            PieceKind::Artillery => PieceKind::Cannon,
            PieceKind::Barricade => PieceKind::Tower,
            PieceKind::Crossbow  => PieceKind::Pawn,
            other => other,
        }
    }

    // Analyze if the piece can cross the river (for movement rules)
    pub fn can_cross_river(self) -> bool {
        !matches!(self, PieceKind::Archer | PieceKind::Carriage)
    }

    #[inline(always)]
    pub fn index(self) -> usize { self as usize }
}

//  Specific piece on the board, including its side (black/white)

// A piece on the board: type + side
// Compress in 1 byte: bits [4:0] = PieceKind, bit [5] = Side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Piece(pub u8);

impl Piece {
    pub const NONE: Piece = Piece(u8::MAX);

    #[inline(always)]
    pub fn new(kind: PieceKind, side: Side) -> Self {
        Piece((kind as u8) | ((side as u8) << 5))
    }

    #[inline(always)]
    pub fn kind(self) -> PieceKind {
        // SAFETY: PieceKind has 17 variants (0-16), and the 5 low bits are always valid
        unsafe { std::mem::transmute(self.0 & 0x1F) }
    }

    #[inline(always)]
    pub fn side(self) -> Side {
        if (self.0 >> 5) & 1 == 0 { Side::Black } else { Side::White }
    }

    #[inline(always)]
    pub fn is_none(self) -> bool { self == Piece::NONE }

    #[inline(always)]
    pub fn index(self) -> usize { self.0 as usize }
}

//  Move

// A move in Weigong, encoded in 32 bits (4 bytes) for compactness and fast copying.
// Layout:
// bits  0- 7 : source square (0-168, 255 = drop from reserve)
// bits  8-15 : destination square (0-168)
// bits 16-20 : PieceKind moving
// bits 21-25 : captured PieceKind (NONE = 31 if no capture)
// bit  26    : promotion in this move
// bit  27    : is drop from reserve
// bits 28-31 : reserved

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move(pub u32);

impl Move {
    pub const NULL: Move = Move(0);

    #[inline(always)]
    pub fn new_normal(from: Square, to: Square, piece: PieceKind, captured: Option<PieceKind>, promote: bool) -> Self {
        let cap = captured.map(|k| k as u32).unwrap_or(31);
        Move(
            (from.0 as u32)         |
            ((to.0 as u32) << 8)    |
            ((piece as u32) << 16)  |
            (cap << 21)             |
            ((promote as u32) << 26)
        )
    }

    #[inline(always)]
    pub fn new_drop(to: Square, piece: PieceKind) -> Self {
        Move(
            (255u32)                 |  // from = 255 indicates drops
            ((to.0 as u32) << 8)     |
            ((piece as u32) << 16)   |
            (31 << 21)               |  // without capture is drop
            (1 << 27)
        )
    }

    #[inline(always)]
    pub fn from_sq(self) -> Square   { Square((self.0 & 0xFF) as u8) }

    #[inline(always)]
    pub fn to_sq(self) -> Square     { Square(((self.0 >> 8) & 0xFF) as u8) }

    #[inline(always)]
    pub fn piece(self) -> PieceKind  { unsafe { std::mem::transmute(((self.0 >> 16) & 0x1F) as u8) } }

    #[inline(always)]
    pub fn captured(self) -> Option<PieceKind> {
        let k = ((self.0 >> 21) & 0x1F) as u8;
        if k == 31 { None } else { unsafe { Some(std::mem::transmute(k)) } }
    }

    #[inline(always)]
    pub fn is_promotion(self) -> bool { (self.0 >> 26) & 1 == 1 }

    #[inline(always)]
    pub fn is_drop(self) -> bool      { (self.0 >> 27) & 1 == 1 }

    #[inline(always)]
    pub fn is_capture(self) -> bool   { self.captured().is_some() }

    #[inline(always)]
    pub fn is_null(self) -> bool      { self.0 == 0 }
}

// Notation terminal

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotationTerminal {
    None,
    Check,
    Checkmate,
    PalaceMate,
    Stalemate,
    Draw,
    DrawAgreement,
    MoveLimitDraw,
}

impl Default for NotationTerminal {
    fn default() -> Self {
        Self::None
    }
}

// Suffix for move notation indicating check, checkmate, stalemate, draw, etc.

impl NotationTerminal {
    #[inline(always)]
    pub fn as_str(self) -> &'static str {
        match self {
            NotationTerminal::None => "",
            NotationTerminal::Check => "%",
            NotationTerminal::Checkmate => "#",
            NotationTerminal::PalaceMate => "##",
            NotationTerminal::Stalemate => "^",
            NotationTerminal::Draw => "=",
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
    pub archer: Option<ArcherNotation>,
    pub curse: Option<PalaceCurseNotation>,
    pub terminal: NotationTerminal,
    pub check: bool,
}

// Pieces symbols for notation

#[inline(always)]
fn piece_symbol(kind: PieceKind) -> &'static str {
    match kind {
        PieceKind::King => "K",
        PieceKind::Queen => "Q",
        PieceKind::General => "G",
        PieceKind::Elephant => "E",
        PieceKind::Priest => "P",
        PieceKind::Horse => "H",
        PieceKind::Cannon => "C",
        PieceKind::Tower => "T",
        PieceKind::Carriage => "Ca",
        PieceKind::Archer => "A",
        PieceKind::Pawn => "p",
        PieceKind::Fortress => "F",
        PieceKind::Wisdom => "W",
        PieceKind::Steed => "S",
        PieceKind::Artillery => "R",
        PieceKind::Barricade => "U",
        PieceKind::Crossbow => "B",
    }
}

// Palace curse notation

#[inline(always)]
fn piece_symbol_lower(kind: PieceKind) -> String {
    piece_symbol(kind).to_ascii_lowercase()
}

pub fn append_curse_notation(curse: &PalaceCurseNotation) -> String {
    let mut suffix = String::new();

    for group in &curse.groups {
        if group.is_empty() {
            continue;
        }

        suffix.push('&');
        for (i, (piece, sq)) in group.iter().enumerate() {
            if i > 0 {
                suffix.push('&');
            }
            suffix.push_str(piece_symbol(*piece));
            suffix.push_str(&sq.to_string());
        }
        suffix.push('-');
    }

    suffix
}

// Drops notation

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
    } else {
        let sym = piece_symbol(moved_piece_after); // Notation for archer ambush

        if let Some(ambush_info) = ambush {
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
                            if i > 0 {
                                s.push(',');
                            }
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
                            if i == *chosen_index {
                                continue;
                            }

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
            s.push_str(sym);
            if let Some(target) = captured_piece {
                s.push('x');
                s.push_str(&piece_symbol_lower(target));
                s.push_str(&to_str);
            } else {
                s.push_str(&to_str);
            }

            if mv.is_promotion() {
                s.push('+');
            }
        }
    }

    // Append check/checkmate/stalemate/draw notation

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

// Main method to get move notation, combining all the context (captures, promotions, archer ambushes, palace curses, check/checkmate/stalemate/draw)

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

//  MoveList in the stack (without heap allocation).
// 256 is enough for Weigong

pub struct MoveList {
    moves: [Move; 256],
    len: usize,
}

impl MoveList {
    #[inline(always)]
    pub fn new() -> Self {
        MoveList { moves: [Move::NULL; 256], len: 0 }
    }

    #[inline(always)]
    pub fn push(&mut self, mv: Move) {
        debug_assert!(self.len < 256, "MoveList overflow");
        self.moves[self.len] = mv;
        self.len += 1;
    }

    #[inline(always)]
    pub fn len(&self) -> usize { self.len }

    #[inline(always)]
    pub fn is_empty(&self) -> bool { self.len == 0 }

    #[inline(always)]
    pub fn as_slice(&self) -> &[Move] { &self.moves[..self.len] }

    #[inline(always)]
    pub fn as_mut_slice(&mut self) -> &mut [Move] { &mut self.moves[..self.len] }
}

impl Default for MoveList {
    fn default() -> Self { Self::new() }
}

//  GameResult

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameResult {
    Win(Side),
    Draw,
    Ongoing,
}

impl GameResult {
    // For training data: 1.0 = black wins, 0.5 = stalemate, 0.0 = white wins
    pub fn wdl_from_black(self) -> f32 {
        match self {
            GameResult::Win(Side::Black) => 1.0,
            GameResult::Win(Side::White) => 0.0,
            GameResult::Draw             => 0.5,
            GameResult::Ongoing          => 0.5, // It shouldn't be used
        }
    }
}

//  PalaceState

// Tracks if an enemy piece is inside a palace and how many turns it has been there (for the 3-turn rule).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PalaceState {
    // Verify if there's a foreign piece in the palace
    pub invaded: bool,
    // Remaining turns the defender has to expel it (0-3)
    pub turns_remaining: u8,
    // Verify if the palace is currently under curse (after 3 turns of invasion)
    pub cursed: bool,
}