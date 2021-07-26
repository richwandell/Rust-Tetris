use std::collections::HashMap;
use crate::utils::context;
use crate::tetris_piece::{TetrisPiece, TetrisPieceType};
use crate::tetris_part::TetrisPart;

const H_CELLS: i64 = 10;
const V_CELLS: i64 = 22;
const H_CELL_SIZE: f64 = 600.0 / (H_CELLS as f64);
const V_CELL_SIZE: f64 = 1000.0 / (V_CELLS as f64);

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

const PINK: &str = "#cd00cd";
const RED: &str = "#ff0000";
const YELLOW: &str = "#ffff0e";
const GREEN: &str = "#00ff00";
const ORANGE: &str = "#ff7800";
const LIGHT_BLUE: &str = "#00ffff";
const DARK_BLUE: &str = "#0000ac";

pub(crate) struct TetrisGame {
    pub(crate) grid: HashMap<String, usize>,
    pub(crate) pieces: Vec<TetrisPiece>,
    pub(crate) piece_bag: Vec<TetrisPieceType>,
    pub(crate) color_bag: Vec<String>,
    pub(crate) active_piece: i64
}

impl TetrisGame {

    pub(crate) fn new() -> TetrisGame {
        TetrisGame {
            grid: Default::default(),
            pieces: Default::default(),
            piece_bag: TetrisGame::new_piece_type_bag(),
            color_bag: TetrisGame::new_color_bag(),
            active_piece: -1
        }
    }

    fn next_piece(&mut self) -> TetrisPieceType {
        let index = (js_sys::Math::random() * (self.piece_bag.len() as f64)).floor() as usize;
        let value = self.piece_bag.remove(index);
        if self.piece_bag.len() == 0 {
            self.piece_bag = TetrisGame::new_piece_type_bag();
        }
        value
    }

    fn next_color(&mut self) -> String {
        let index = (js_sys::Math::random() * (self.color_bag.len() as f64)).floor() as usize;
        let value = self.color_bag.remove(index);
        if self.color_bag.len() == 0 {
            self.color_bag = TetrisGame::new_color_bag();
        }
        value
    }

    fn add_piece(&mut self, piece: TetrisPiece) {
        let piece_num = self.pieces.len();

        for part in &piece.parts {
            let key = format!("{},{}", part.x.to_string(), part.y.to_string());
            self.grid.insert(key, piece_num);
        }

        self.pieces.push(piece);

        self.active_piece = piece_num as i64;
    }

    pub(crate) fn draw(&mut self) {
        TetrisGame::draw_game_board();
        self.draw_pieces();
    }

    pub(crate) fn tick(&mut self) {
        if self.active_piece == -1 {
            let item = self.next_piece();
            let color = self.next_color();
            self.add_piece(TetrisPiece::new(item, 3, color));
        } else {
            self.move_down();
        }

        // self.log_state();
        self.draw();
    }

    pub(crate) fn move_down(&mut self) {
        if self.active_piece == -1 {
            return;
        }
        let mut piece = self.pieces.remove(self.active_piece as usize);
        let mut new_parts = vec![];
        let mut can_move = true;
        for part in &piece.parts {
            let next_key = format!("{},{}", part.x, part.y + 1);
            if self.grid.contains_key(&next_key) {
                if let Some(piece_index) = self.grid.remove(&next_key) {
                    self.grid.insert(next_key, piece_index);
                    if piece_index as i64 != self.active_piece {
                        can_move = false;
                    }
                }
            }
            if (part.y + 1) >= 22 {
                can_move = false;
            }
        }
        if can_move {
            for part in &piece.parts {
                let current_key = format!("{},{}", part.x, part.y);
                self.grid.remove(&current_key);
            }
            for part in piece.parts {
                let next_key = format!("{},{}", part.x, part.y + 1);
                self.grid.insert(next_key, self.pieces.len());
                new_parts.push(TetrisPart {
                    x: part.x,
                    y: part.y + 1
                })
            }
            piece.parts = new_parts;
            self.active_piece = self.pieces.len() as i64;
        } else {
            self.active_piece = -1;
        }
        self.pieces.push(piece);
    }

    fn draw_pieces(&mut self) {
        let context = context();
        for piece in &self.pieces {
            context.set_fill_style(&piece.color.clone().into());
            for part in &piece.parts {
                let x_start = (part.x as f64) * H_CELL_SIZE;
                let y_start = (part.y as f64) * V_CELL_SIZE;
                context.fill_rect(x_start, y_start, H_CELL_SIZE, V_CELL_SIZE);
            }
        }
    }

    fn new_piece_type_bag() -> Vec<TetrisPieceType> {
        vec![
            TetrisPieceType::Q,
            TetrisPieceType::Z,
            TetrisPieceType::S,
            TetrisPieceType::T,
            TetrisPieceType::I,
            TetrisPieceType::L,
            TetrisPieceType::J
        ]
    }

    fn new_color_bag() -> Vec<String> {
        vec![
            PINK.to_string(), RED.to_string(), YELLOW.to_string(), GREEN.to_string(), ORANGE.to_string(), LIGHT_BLUE.to_string(), DARK_BLUE.to_string()
        ]
    }

    fn draw_game_board() {
        let context = context();
        context.set_fill_style(&"#000712".into());
        context.fill_rect(0.0, 0.0, 600.0, 1000.0);

        context.set_stroke_style(&"#0654df".into());
        for n in 0..23 {
            context.begin_path();
            context.move_to(0.0, V_CELL_SIZE * (n as f64));
            context.line_to(600.0, V_CELL_SIZE * (n as f64));
            context.stroke();
        }

        for n in 0..11 {
            context.begin_path();
            context.move_to(H_CELL_SIZE * (n as f64), 0.0);
            context.line_to(H_CELL_SIZE * (n as f64), 1000.0);
            context.stroke();
        }
    }

    #[allow(dead_code)]
    fn log_state(&mut self) {
        log!("{}, num pieces: {}", "logging state", self.pieces.len());
        log!("{:?}", self.grid);
    }

    pub(crate) fn move_left(&mut self) {
        if self.active_piece == -1 {
            return;
        }

        let active_piece = self.active_piece.clone();
        let mut piece = self.pieces.remove(active_piece as usize);
        let mut new_parts = vec![];
        let mut can_move_left = true;
        for part in &piece.parts {
            let next_key = format!("{},{}", part.x - 1, part.y);
            if self.grid.contains_key(&next_key) {
                if let Some(piece_index) = self.grid.remove(&next_key) {
                    self.grid.insert(next_key, piece_index);
                    if piece_index as i64 != active_piece {
                        can_move_left = false;
                    }
                }
            }
            if (part.x - 1) < 0 {
                can_move_left = false;
            }
        }
        if can_move_left {
            for part in &piece.parts {
                let current_key = format!("{},{}", part.x, part.y);
                self.grid.remove(&current_key);
            }
            for part in piece.parts {
                let next_key = format!("{},{}", part.x - 1, part.y);
                self.grid.insert(next_key, self.pieces.len());
                new_parts.push(TetrisPart {
                    x: part.x - 1,
                    y: part.y
                })
            }
            piece.parts = new_parts;
        }
        self.active_piece = self.pieces.len() as i64;
        self.pieces.push(piece);
    }

    pub(crate) fn move_right(&mut self) {
        if self.active_piece == -1 {
            return;
        }

        let active_piece = self.active_piece.clone();
        let mut piece = self.pieces.remove(active_piece as usize);
        let mut new_parts = vec![];
        let mut can_move = true;
        for part in &piece.parts {
            let next_key = format!("{},{}", part.x + 1, part.y);
            if self.grid.contains_key(&next_key) {
                if let Some(piece_index) = self.grid.remove(&next_key) {
                    self.grid.insert(next_key, piece_index);
                    if piece_index as i64 != active_piece {
                        can_move = false;
                    }
                }
            }
            if (part.x + 1) >= 10 {
                can_move = false;
            }
        }
        if can_move {
            for part in &piece.parts {
                let current_key = format!("{},{}", part.x, part.y);
                self.grid.remove(&current_key);
            }
            for part in piece.parts {
                let next_key = format!("{},{}", part.x + 1, part.y);
                self.grid.insert(next_key, self.pieces.len());
                new_parts.push(TetrisPart {
                    x: part.x + 1,
                    y: part.y
                })
            }
            piece.parts = new_parts;
        }
        self.active_piece = self.pieces.len() as i64;
        self.pieces.push(piece);
    }

    pub(crate) fn rotate(&mut self) {
        if self.active_piece == -1 {
            return;
        }

        let mut piece = self.pieces.remove(self.active_piece as usize);
        let original_rotation = piece.rotation.clone();
        for part in &piece.parts {
            let current_key = format!("{},{}", part.x, part.y);
            self.grid.remove(&current_key);
        }
        let new_parts = match piece.piece_type {
            TetrisPieceType::Q => {
                piece.parts.clone()
            }
            TetrisPieceType::Z => {
                if piece.rotation == 1 {
                    piece.rotation = 2;
                    vec![
                        TetrisPart { x: piece.parts[0].x + 2, y: piece.parts[0].y },
                        TetrisPart {x: piece.parts[1].x + 1, y: piece.parts[1].y + 1},
                        TetrisPart {x: piece.parts[2].x, y: piece.parts[2].y},
                        TetrisPart {x: piece.parts[3].x - 1, y: piece.parts[3].y + 1}
                    ]
                } else if piece.rotation == 2 {
                    piece.rotation = 3;
                    vec![
                        TetrisPart { x: piece.parts[0].x, y: piece.parts[0].y + 2 },
                        TetrisPart { x: piece.parts[1].x - 1, y: piece.parts[1].y + 1 },
                        TetrisPart { x: piece.parts[2].x, y: piece.parts[2].y },
                        TetrisPart { x: piece.parts[3].x - 1, y: piece.parts[3].y - 1 }
                    ]
                } else if piece.rotation == 3 {
                    piece.rotation = 4;
                    vec![
                        TetrisPart { x: piece.parts[0].x - 2, y: piece.parts[0].y },
                        TetrisPart { x: piece.parts[1].x - 1, y: piece.parts[1].y - 1 },
                        TetrisPart { x: piece.parts[2].x, y: piece.parts[2].y },
                        TetrisPart { x: piece.parts[3].x + 1, y: piece.parts[3].y - 1 }
                    ]
                } else {
                    // rotation == 4
                    piece.rotation = 1;
                    vec![
                        TetrisPart { x: piece.parts[0].x, y: piece.parts[0].y - 2},
                        TetrisPart { x: piece.parts[1].x + 1, y: piece.parts[1].y - 1 },
                        TetrisPart { x: piece.parts[2].x, y: piece.parts[2].y },
                        TetrisPart { x: piece.parts[3].x + 1, y: piece.parts[3].y + 1 }
                    ]
                }
            }
            TetrisPieceType::S => {
                if piece.rotation == 1 {
                    piece.rotation = 2;
                    vec![
                        TetrisPart { x: piece.parts[0].x + 1, y: piece.parts[0].y + 1},
                        TetrisPart {x: piece.parts[1].x, y: piece.parts[1].y + 2},
                        TetrisPart {x: piece.parts[2].x + 1, y: piece.parts[2].y - 1},
                        TetrisPart {x: piece.parts[3].x, y: piece.parts[3].y}
                    ]
                } else if piece.rotation == 2 {
                    piece.rotation = 3;
                    vec![
                        TetrisPart { x: piece.parts[0].x - 1, y: piece.parts[0].y + 1},
                        TetrisPart {x: piece.parts[1].x - 2, y: piece.parts[1].y},
                        TetrisPart {x: piece.parts[2].x + 1, y: piece.parts[2].y + 1},
                        TetrisPart {x: piece.parts[3].x, y: piece.parts[3].y}
                    ]
                } else if piece.rotation == 3 {
                    piece.rotation = 4;
                    vec![
                        TetrisPart { x: piece.parts[0].x - 1, y: piece.parts[0].y - 1},
                        TetrisPart {x: piece.parts[1].x, y: piece.parts[1].y - 2},
                        TetrisPart {x: piece.parts[2].x - 1, y: piece.parts[2].y + 1},
                        TetrisPart {x: piece.parts[3].x, y: piece.parts[3].y}
                    ]
                } else {
                    piece.rotation = 1;
                    vec![
                        TetrisPart { x: piece.parts[0].x + 1, y: piece.parts[0].y - 1},
                        TetrisPart {x: piece.parts[1].x + 2, y: piece.parts[1].y},
                        TetrisPart {x: piece.parts[2].x - 1, y: piece.parts[2].y - 1},
                        TetrisPart {x: piece.parts[3].x, y: piece.parts[3].y}
                    ]
                }
            }
            TetrisPieceType::T => {
                if piece.rotation == 1 {
                    piece.rotation = 2;
                    vec![
                        TetrisPart { x: piece.parts[0].x + 1, y: piece.parts[0].y - 1},
                        TetrisPart {x: piece.parts[1].x, y: piece.parts[1].y},
                        TetrisPart {x: piece.parts[2].x - 1, y: piece.parts[2].y + 1},
                        TetrisPart {x: piece.parts[3].x + 1, y: piece.parts[3].y + 1}
                    ]
                } else if piece.rotation == 2 {
                    piece.rotation = 3;
                    vec![
                        TetrisPart { x: piece.parts[0].x + 1, y: piece.parts[0].y + 1},
                        TetrisPart {x: piece.parts[1].x, y: piece.parts[1].y},
                        TetrisPart {x: piece.parts[2].x - 1, y: piece.parts[2].y - 1},
                        TetrisPart {x: piece.parts[3].x - 1, y: piece.parts[3].y + 1}
                    ]
                } else if piece.rotation == 3 {
                    piece.rotation = 4;
                    vec![
                        TetrisPart { x: piece.parts[0].x - 1, y: piece.parts[0].y + 1},
                        TetrisPart {x: piece.parts[1].x, y: piece.parts[1].y},
                        TetrisPart {x: piece.parts[2].x + 1, y: piece.parts[2].y - 1},
                        TetrisPart {x: piece.parts[3].x - 1, y: piece.parts[3].y - 1}
                    ]
                } else {
                    piece.rotation = 1;
                    vec![
                        TetrisPart { x: piece.parts[0].x - 1, y: piece.parts[0].y - 1},
                        TetrisPart {x: piece.parts[1].x, y: piece.parts[1].y},
                        TetrisPart {x: piece.parts[2].x + 1, y: piece.parts[2].y + 1},
                        TetrisPart {x: piece.parts[3].x + 1, y: piece.parts[3].y - 1}
                    ]
                }
            }
            TetrisPieceType::I => {
                if piece.rotation == 1 {
                    piece.rotation = 2;
                    vec![
                        TetrisPart { x: piece.parts[0].x + 2, y: piece.parts[0].y - 1},
                        TetrisPart {x: piece.parts[1].x + 1, y: piece.parts[1].y},
                        TetrisPart {x: piece.parts[2].x, y: piece.parts[2].y + 1},
                        TetrisPart {x: piece.parts[3].x - 1, y: piece.parts[3].y + 2}
                    ]
                } else if piece.rotation == 2 {
                    piece.rotation = 3;
                    vec![
                        TetrisPart { x: piece.parts[0].x + 1, y: piece.parts[0].y + 2},
                        TetrisPart {x: piece.parts[1].x, y: piece.parts[1].y + 1},
                        TetrisPart {x: piece.parts[2].x - 1, y: piece.parts[2].y},
                        TetrisPart {x: piece.parts[3].x - 2, y: piece.parts[3].y - 1}
                    ]
                } else if piece.rotation == 3 {
                    piece.rotation = 4;
                    vec![
                        TetrisPart { x: piece.parts[0].x - 2, y: piece.parts[0].y + 1},
                        TetrisPart {x: piece.parts[1].x - 1, y: piece.parts[1].y},
                        TetrisPart {x: piece.parts[2].x, y: piece.parts[2].y - 1},
                        TetrisPart {x: piece.parts[3].x + 1, y: piece.parts[3].y - 2}
                    ]
                } else {
                    piece.rotation = 1;
                    vec![
                        TetrisPart { x: piece.parts[0].x - 1, y: piece.parts[0].y - 2},
                        TetrisPart {x: piece.parts[1].x, y: piece.parts[1].y - 1},
                        TetrisPart {x: piece.parts[2].x + 1, y: piece.parts[2].y},
                        TetrisPart {x: piece.parts[3].x + 2, y: piece.parts[3].y + 1}
                    ]
                }
            }
            TetrisPieceType::L => {
                if piece.rotation == 1 {
                    piece.rotation = 2;
                    vec![
                        TetrisPart { x: piece.parts[0].x + 1, y: piece.parts[0].y - 1},
                        TetrisPart {x: piece.parts[1].x, y: piece.parts[1].y},
                        TetrisPart {x: piece.parts[2].x - 1, y: piece.parts[2].y + 1},
                        TetrisPart {x: piece.parts[3].x, y: piece.parts[3].y + 2}
                    ]
                } else if piece.rotation == 2 {
                    piece.rotation = 3;
                    vec![
                        TetrisPart { x: piece.parts[0].x + 1, y: piece.parts[0].y + 1},
                        TetrisPart {x: piece.parts[1].x, y: piece.parts[1].y},
                        TetrisPart {x: piece.parts[2].x - 1, y: piece.parts[2].y - 1},
                        TetrisPart {x: piece.parts[3].x - 2, y: piece.parts[3].y}
                    ]
                } else if piece.rotation == 3 {
                    piece.rotation = 4;
                    vec![
                        TetrisPart { x: piece.parts[0].x - 1, y: piece.parts[0].y + 1},
                        TetrisPart {x: piece.parts[1].x, y: piece.parts[1].y},
                        TetrisPart {x: piece.parts[2].x + 1, y: piece.parts[2].y - 1},
                        TetrisPart {x: piece.parts[3].x, y: piece.parts[3].y - 2}
                    ]
                } else {
                    piece.rotation = 1;
                    vec![
                        TetrisPart { x: piece.parts[0].x - 1, y: piece.parts[0].y - 1},
                        TetrisPart {x: piece.parts[1].x, y: piece.parts[1].y},
                        TetrisPart {x: piece.parts[2].x + 1, y: piece.parts[2].y + 1},
                        TetrisPart {x: piece.parts[3].x + 2, y: piece.parts[3].y}
                    ]
                }
            }
            TetrisPieceType::J => {
                if piece.rotation == 1 {
                    piece.rotation = 2;
                    vec![
                        TetrisPart { x: piece.parts[0].x + 2, y: piece.parts[0].y},
                        TetrisPart {x: piece.parts[1].x + 1, y: piece.parts[1].y - 1},
                        TetrisPart {x: piece.parts[2].x, y: piece.parts[2].y},
                        TetrisPart {x: piece.parts[3].x - 1, y: piece.parts[3].y + 1}
                    ]
                } else if piece.rotation == 2 {
                    piece.rotation = 3;
                    vec![
                        TetrisPart { x: piece.parts[0].x, y: piece.parts[0].y + 2},
                        TetrisPart {x: piece.parts[1].x + 1, y: piece.parts[1].y + 1},
                        TetrisPart {x: piece.parts[2].x, y: piece.parts[2].y},
                        TetrisPart {x: piece.parts[3].x - 1, y: piece.parts[3].y - 1}
                    ]
                } else if piece.rotation == 3 {
                    piece.rotation = 4;
                    vec![
                        TetrisPart { x: piece.parts[0].x - 2, y: piece.parts[0].y},
                        TetrisPart {x: piece.parts[1].x - 1, y: piece.parts[1].y + 1},
                        TetrisPart {x: piece.parts[2].x, y: piece.parts[2].y},
                        TetrisPart {x: piece.parts[3].x + 1, y: piece.parts[3].y - 1}
                    ]
                } else {
                    piece.rotation = 1;
                    vec![
                        TetrisPart { x: piece.parts[0].x, y: piece.parts[0].y - 2},
                        TetrisPart {x: piece.parts[1].x - 1, y: piece.parts[1].y - 1},
                        TetrisPart {x: piece.parts[2].x, y: piece.parts[2].y},
                        TetrisPart {x: piece.parts[3].x + 1, y: piece.parts[3].y + 1}
                    ]
                }
            }
        };

        let mut can_rotate = true;
        for part in &new_parts {
            if part.x >= 10 || part.x < 0 || part.y >= 22 {
                can_rotate = false;
                break;
            }
        }
        if can_rotate {
            piece = TetrisPiece {
                parts: new_parts,
                color: piece.color,
                piece_type: piece.piece_type,
                rotation: piece.rotation
            };
        } else {
            piece = TetrisPiece {
                parts: piece.parts.clone(),
                color: piece.color,
                piece_type: piece.piece_type,
                rotation: original_rotation
            };
        }
        self.active_piece = self.pieces.len() as i64;

        for part in &piece.parts {
            let new_key = format!("{},{}", part.x, part.y);
            self.grid.insert(new_key, self.active_piece.clone() as usize);
        }
        self.pieces.push(piece);
    }
}