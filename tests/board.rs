use rust_chess::board::Board;

#[test]
fn board_test() {
    let b: Board = Board::new();
    let v = b.get_pseudo_legal_moves();

    for mv in v {
        println!("{:?}", mv);
    }
}
