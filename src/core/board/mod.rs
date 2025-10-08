use super::{color::Color, piece::Piece};

struct PlayerCastlingRights {
	queenside: bool,
	kingside: bool,
}

pub struct Board {
	state: [Option<Piece>; 64],
	turn: Color,
	castling_rights: [PlayerCastlingRights; 2],
}