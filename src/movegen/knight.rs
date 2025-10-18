use crate::core::{board_view::BoardView, chess_move::Move, piece::Piece, square::Square};

pub fn get_knight_pseudo_legal_moves(
    board: &impl BoardView,
    src_square: Square,
    piece: Piece,
) -> Vec<Move> {
    const KNIGHT_MOVES: [(i8, i8); 8] = [
        (-1, 2),
        (-1, -2),
        (1, 2),
        (1, -2),
        (-2, 1),
        (-2, -1),
        (2, 1),
        (2, -1),
    ];

    let mut moves = Vec::new();

    for (i, j) in KNIGHT_MOVES {
        let Some(sq) = src_square.get_relative_square(i, j) else {
            continue;
        };

        if let Some(target_piece) = board.get(sq) {
            if target_piece.piece_color != piece.piece_color {
                moves.push(Move::new(
                    src_square,
                    sq,
                    Some(target_piece.piece_type),
                    None,
                ));
            }
        } else {
            moves.push(Move::new(src_square, sq, None, None));
        }
    }

    moves
}
