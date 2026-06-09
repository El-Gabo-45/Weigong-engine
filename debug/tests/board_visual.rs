use weigong_engine::bitboard::{BLACK_SIDE, WHITE_SIDE, RIVER, BLACK_PALACE, WHITE_PALACE};
use weigong_engine::types::{FILES, RANKS, Square};

#[test]
fn print_board_sides() {
    println!("\n=== BOARD LAYOUT ===");
    println!("     a b c d e f g h i j k l m");
    for rank in 0..RANKS {
        let rank_num = RANKS - rank;
        print!("  {:2} ", rank_num);
        for file in 0..FILES {
            let sq = Square::new(rank, file);
            let ch = if BLACK_PALACE.contains(sq)  { 'B' }
                else if WHITE_PALACE.contains(sq)  { 'W' }
                else if RIVER.contains(sq)         { '~' }
                else if BLACK_SIDE.contains(sq)    { 'b' }
                else if WHITE_SIDE.contains(sq)    { 'w' }
                else                               { '?' };
            print!("{} ", ch);
        }
        println!();
    }
    println!();
    println!("  B = black palace  W = white palace");
    println!("  b = black side    w = white side");
    println!("  ~ = river");
}