use bevy::prelude::*;

pub struct ChessPlugin;

#[derive(Debug, Event)]
pub struct UpdateChessboard {
    fen: String,
}

impl Plugin for ChessPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChessGame::new())
            .add_event::<UpdateChessboard>();
    }
}

const K: usize = 0x01;
const Q: usize = 0x02;
const R: usize = 0x03;
const B: usize = 0x04;
const N: usize = 0x05;
const P: usize = 0x06;

const WH: usize = 0x00;
const BL: usize = 0x10;

const PIECE_MASK: usize = 0x0F;
const COLOR_MASK: usize = 0x10;

type Piece = usize;

#[derive(Resource)]
pub struct ChessGame {
    pieces: [Piece; 64],
}

impl ChessGame {
    fn new() -> Self {
        Self {
            pieces: [
                BL | R,
                BL | N,
                BL | B,
                BL | Q,
                BL | K,
                BL | B,
                BL | N,
                BL | R,
                BL | P,
                BL | P,
                BL | P,
                BL | P,
                BL | P,
                BL | P,
                BL | P,
                BL | P,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                WH | P,
                WH | P,
                WH | P,
                WH | P,
                WH | P,
                WH | P,
                WH | P,
                WH | P,
                WH | R,
                WH | N,
                WH | B,
                WH | Q,
                WH | K,
                WH | B,
                WH | N,
                WH | R,
            ],
        }
    }

    pub fn handle_move(from: (usize, usize), to: (usize, usize)) {
        todo!("Handle move from {:?} to {:?}", from, to)
    }

    pub fn fen(&self) -> String {
        let mut fen_str = String::new();

        // Board state
        for rank in (0..8).rev() {
            let mut empty_count = 0;

            for file in 0..8 {
                let index = rank * 8 + file;
                let piece = self.pieces[index];

                if piece == 0 {
                    empty_count += 1;
                } else {
                    if empty_count > 0 {
                        fen_str.push_str(&empty_count.to_string());
                        empty_count = 0;
                    }

                    let piece_char = match (piece & PIECE_MASK, piece & COLOR_MASK) {
                        (K, BL) => 'k',
                        (Q, BL) => 'q',
                        (R, BL) => 'r',
                        (B, BL) => 'b',
                        (N, BL) => 'n',
                        (P, BL) => 'p',
                        (K, WH) => 'K',
                        (Q, WH) => 'Q',
                        (R, WH) => 'R',
                        (B, WH) => 'B',
                        (N, WH) => 'N',
                        (P, WH) => 'P',
                        _ => panic!("Invalid piece"),
                    };

                    fen_str.push(piece_char);
                }
            }

            if empty_count > 0 {
                fen_str.push_str(&empty_count.to_string());
            }

            if rank > 0 {
                fen_str.push('/');
            }
        }

        fen_str
    }
}
