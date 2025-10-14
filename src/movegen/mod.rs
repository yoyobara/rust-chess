pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

pub use bishop::get_bishop_pseudo_legal_moves;
pub use king::get_king_pseudo_legal_moves;
pub use knight::get_knight_pseudo_legal_moves;
pub use pawn::get_pawn_pseudo_legal_moves;
pub use queen::get_queen_pseudo_legal_moves;
pub use rook::get_rook_pseudo_legal_moves;

// Consolidated move generation module. Each piece-specific generator lives in its own file
// and is re-exported here for a single import path: `crate::movegen::get_*`.
