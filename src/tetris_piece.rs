use crate::tetris_part::TetrisPart;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum TetrisPieceType {
    Q,
    Z,
    S,
    T,
    I,
    L,
    J
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct TetrisPiece {
    pub(crate) parts: Vec<TetrisPart>,
    pub(crate) color: String,
    pub(crate) piece_type: TetrisPieceType,
    pub(crate) rotation: i64
}

impl TetrisPiece {

    pub(crate) fn new(piece_type: TetrisPieceType, start_x: i64, color: String) -> TetrisPiece {
        match piece_type {
            TetrisPieceType::Q => {
                TetrisPiece {
                    rotation: 1,
                    piece_type,
                    color,
                    parts: vec![
                        TetrisPart::new(start_x, 0),
                        TetrisPart::new(start_x+1, 0),
                        TetrisPart::new(start_x, 1),
                        TetrisPart::new(start_x+1, 1)
                    ]
                }
            }
            TetrisPieceType::Z => {
                TetrisPiece {
                    rotation: 1,
                    piece_type,
                    color,
                    parts: vec![
                        TetrisPart::new(start_x, 0),
                        TetrisPart::new(start_x+1, 0),
                        TetrisPart::new(start_x+1, 1),
                        TetrisPart::new(start_x+2, 1)
                    ]
                }
            }
            TetrisPieceType::S => {
                TetrisPiece {
                    rotation: 1,
                    piece_type,
                    color,
                    parts: vec![
                        TetrisPart::new(start_x+1, 0),
                        TetrisPart::new(start_x+2, 0),
                        TetrisPart::new(start_x, 1),
                        TetrisPart::new(start_x+1, 1)
                    ]
                }
            }
            TetrisPieceType::T => {
                TetrisPiece {
                    rotation: 1,
                    piece_type,
                    color,
                    parts: vec![
                        TetrisPart::new(start_x, 1),
                        TetrisPart::new(start_x+1, 1),
                        TetrisPart::new(start_x+2, 1),
                        TetrisPart::new(start_x+1, 0)
                    ]
                }
            }
            TetrisPieceType::I => {
                TetrisPiece {
                    rotation: 1,
                    piece_type,
                    color,
                    parts: vec![
                        TetrisPart::new(start_x, 0),
                        TetrisPart::new(start_x+1, 0),
                        TetrisPart::new(start_x+2, 0),
                        TetrisPart::new(start_x+3, 0)
                    ]
                }
            }
            TetrisPieceType::L => {
                TetrisPiece {
                    rotation: 1,
                    piece_type,
                    color,
                    parts: vec![
                        TetrisPart::new(start_x, 1),
                        TetrisPart::new(start_x+1, 1),
                        TetrisPart::new(start_x+2, 1),
                        TetrisPart::new(start_x+2, 0)
                    ]
                }
            }
            TetrisPieceType::J => {
                TetrisPiece {
                    rotation: 1,
                    piece_type,
                    color,
                    parts: vec![
                        TetrisPart::new(start_x, 0),
                        TetrisPart::new(start_x, 1),
                        TetrisPart::new(start_x+1, 1),
                        TetrisPart::new(start_x+2, 1)
                    ]
                }
            }
        }
    }
}