use weigong_engine::zobrist;

#[test]
fn table_has_correct_size() {
    let table = zobrist::table();
    assert_eq!(table.len(), 169 * 17 * 2);
}

#[test]
fn table_no_zeros() {
    // extremely unlikely all entries are zero if RNG is working
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
    // with 5746 random u64s, collisions are astronomically unlikely
    assert_eq!(set.len(), table.len());
}