use std::collections::HashMap;
use crate::utils::{context, next_piece_context, score};
use crate::tetris_piece::{TetrisPiece, TetrisPieceType};
use crate::tetris_part::TetrisPart;



const H_CELLS: i64 = 10;
const V_CELLS: i64 = 22;
const H_CELL_SIZE: f64 = 600.0 / (H_CELLS as f64);
const V_CELL_SIZE: f64 = 1000.0 / (V_CELLS as f64);
pub const NP_HEIGHT: u32 = 400;
pub const NP_WIDTH: u32 = 150;
const NP_SECTION_HEIGHT: f64 = NP_HEIGHT as f64 / 3.0;
const NP_V_CELL_SIZE: f64 = NP_SECTION_HEIGHT as f64 / 4.0;
const NP_H_CELL_SIZE: f64 = NP_WIDTH as f64 / 4.0;

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
    pub(crate) active_piece: i64,
    pub(crate) next_pieces: Vec<TetrisPiece>,
    pub(crate) score: i64,
    pub(crate) clearing: i64,
    cleared_rows: Vec<i64>,
    level: usize
}

impl TetrisGame {

    fn draw_next_pieces(&mut self) {
        let ctx = next_piece_context();
        ctx.set_fill_style(&"#071428".into());
        ctx.fill_rect(0.0, 0.0, NP_WIDTH as f64, NP_HEIGHT as f64);

        let mut i = 0.0;
        for piece in &self.next_pieces {
            let piece_parts = piece.parts.clone();
            let start = i * NP_SECTION_HEIGHT;
            for part in piece_parts {
                let x = part.x - 3;
                let y = part.y;

                ctx.set_fill_style(&piece.color.clone().into());
                let x_start = (x as f64 * NP_H_CELL_SIZE) + 20.0;
                let y_start = (y as f64 * NP_V_CELL_SIZE) + start + 20.0;
                ctx.fill_rect(x_start, y_start, NP_H_CELL_SIZE, NP_V_CELL_SIZE);
            }
            i += 1.0;
        }
    }

    pub(crate) fn new() -> TetrisGame {
        let mut t = TetrisGame {
            grid: Default::default(),
            pieces: Default::default(),
            piece_bag: TetrisGame::new_piece_type_bag(),
            color_bag: TetrisGame::new_color_bag(),
            active_piece: -1,
            next_pieces: vec![],
            level: 1,
            score: 0,
            clearing: 0,
            cleared_rows: vec![]
        };

        for _i in 0..3 {
            let item = t.next_piece();
            let color = t.next_color();
            let next_piece = TetrisPiece::new(item, 3, color);
            t.next_pieces.push(next_piece);
        }
        t
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

    pub(crate) fn draw_clearing_rows(&mut self) {
        let context = context();
        let r = 255 - self.clearing;
        let g = 213 - self.clearing;
        let b = 0;
        let color = format!("rgb({}, {}, {})", r, g, b);
        log!("drawing: {}", color);
        context.set_fill_style(&color.into());
        for line in &self.cleared_rows {
            log!("{}", line);
            let x_start = 0.0;
            let y_start =  *line as f64 * V_CELL_SIZE;
            context.fill_rect(x_start, y_start, H_CELLS as f64 * H_CELL_SIZE, V_CELL_SIZE);
        }
        self.clearing -= 3;
    }

    pub(crate) fn tick(&mut self) {
        if self.active_piece == -1 {
            let mut lines_cleared = vec![];
            loop {
                let cleared_line = self.check_lines();
                if cleared_line == -1 {
                    break;
                }
                lines_cleared.push(cleared_line);
            }
            if lines_cleared.len() == 4 {
                // tetris
                self.score += 800 * self.level as i64;
            } else if lines_cleared.len() == 3 {
                self.score += 500 * self.level as i64;
            } else if lines_cleared.len() == 2 {
                self.score += 300 * self.level as i64;
            } else if lines_cleared.len() == 1 {
                self.score += 100 * self.level as i64;
            }
            if lines_cleared.len() > 0 {
                self.cleared_rows = lines_cleared;
                self.clearing = 200;
            }
            let next = self.next_pieces.remove(0);
            let item = self.next_piece();
            let color = self.next_color();
            self.next_pieces.push(TetrisPiece::new(item, 3, color));
            self.add_piece(next);
            self.draw_next_pieces();
            self.draw_score();
            self.draw_clearing_rows();
        } else {
            self.move_down();
        }
        // self.log_state();
        self.draw();
    }

    fn draw_score(&mut self) {
        score().set_inner_text(&format!("{}", self.score));
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
            if (part.y + 1) >= V_CELLS {
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
                    y: part.y + 1,
                    visible: part.visible
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
                if !part.visible {
                    continue
                }
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
        for n in 0..V_CELLS+1 {
            context.begin_path();
            context.move_to(0.0, V_CELL_SIZE * (n as f64));
            context.line_to(600.0, V_CELL_SIZE * (n as f64));
            context.stroke();
        }

        for n in 0..H_CELLS+1 {
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
                    y: part.y,
                    visible: part.visible
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
            if (part.x + 1) >= H_CELLS {
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
                    y: part.y,
                    visible: part.visible
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
        macro_rules! tetris_part {
            ($part:expr, $x: expr, $y: expr) => {
                TetrisPart {
                    x: piece.parts[$part].x + $x,
                    y: piece.parts[$part].y + $y,
                    visible: piece.parts[$part].visible
                }
            }
        }
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
                        tetris_part! (0, 2, 0),
                        tetris_part! (1, 1, 1),
                        tetris_part! (2, 0, 0),
                        tetris_part! (3, -1, 1)
                    ]
                } else if piece.rotation == 2 {
                    piece.rotation = 3;
                    vec![
                        tetris_part! (0, 0, 2),
                        tetris_part! (1, -1, 1),
                        tetris_part! (2, 0, 0),
                        tetris_part! (3, -1, -1)
                    ]
                } else if piece.rotation == 3 {
                    piece.rotation = 4;
                    vec![
                        tetris_part! (0, -2, 0),
                        tetris_part! (1, -1, -1),
                        tetris_part! (2, 0, 0),
                        tetris_part! (3, 1, -1)
                    ]
                } else {
                    // rotation == 4
                    piece.rotation = 1;
                    vec![
                        tetris_part! (0, 0, -2),
                        tetris_part! (1, 1, -1),
                        tetris_part! (2, 0, 0),
                        tetris_part! (3, 1, 1)
                    ]
                }
            }
            TetrisPieceType::S => {
                if piece.rotation == 1 {
                    piece.rotation = 2;
                    vec![
                        tetris_part! (0, 1, 1),
                        tetris_part! (1, 0, 2),
                        tetris_part! (2, 1, -1),
                        tetris_part! (3, 0, 0)
                    ]
                } else if piece.rotation == 2 {
                    piece.rotation = 3;
                    vec![
                        tetris_part! (0, -1, 1),
                        tetris_part! (1, -2, 0),
                        tetris_part! (2, 1, 1),
                        tetris_part! (3, 0, 0)
                    ]
                } else if piece.rotation == 3 {
                    piece.rotation = 4;
                    vec![
                        tetris_part! (0, -1, -1),
                        tetris_part! (1, 0, -2),
                        tetris_part! (2, -1, 1),
                        tetris_part! (3, 0, 0)
                    ]
                } else {
                    piece.rotation = 1;
                    vec![
                        tetris_part! (0, 1, -1),
                        tetris_part! (1, 2, 0),
                        tetris_part! (2, -1, -1),
                        tetris_part! (3, 0, 0)
                    ]
                }
            }
            TetrisPieceType::T => {
                if piece.rotation == 1 {
                    piece.rotation = 2;
                    vec![
                        tetris_part! (0, 1, -1),
                        tetris_part! (1, 0, 0),
                        tetris_part! (2, -1, 1),
                        tetris_part! (3, 1, 1)
                    ]
                } else if piece.rotation == 2 {
                    piece.rotation = 3;
                    vec![
                        tetris_part! (0, 1, 1),
                        tetris_part! (1, 0, 0),
                        tetris_part! (2, -1, -1),
                        tetris_part! (3, -1, 1)
                    ]
                } else if piece.rotation == 3 {
                    piece.rotation = 4;
                    vec![
                        tetris_part! (0, -1, 1),
                        tetris_part! (1, 0, 0),
                        tetris_part! (2, 1, -1),
                        tetris_part! (3, -1, -1)
                    ]
                } else {
                    piece.rotation = 1;
                    vec![
                        tetris_part! (0, -1, -1),
                        tetris_part! (1, 0, 0),
                        tetris_part! (2, 1, 1),
                        tetris_part! (3, 1, -1)
                    ]
                }
            }
            TetrisPieceType::I => {
                if piece.rotation == 1 {
                    piece.rotation = 2;
                    vec![
                        tetris_part! (0, 2, -1),
                        tetris_part! (1, 1, 0),
                        tetris_part! (2, 0, 1),
                        tetris_part! (3, -1, 2)
                    ]
                } else if piece.rotation == 2 {
                    piece.rotation = 3;
                    vec![
                        tetris_part! (0, 1, 2),
                        tetris_part! (1, 0, 1),
                        tetris_part! (2, -1, 0),
                        tetris_part! (3, -2, -1)
                    ]
                } else if piece.rotation == 3 {
                    piece.rotation = 4;
                    vec![
                        tetris_part! (0, -2, 1),
                        tetris_part! (1, -1, 0),
                        tetris_part! (2, 0, -1),
                        tetris_part! (3, 1, -2)
                    ]
                } else {
                    piece.rotation = 1;
                    vec![
                        tetris_part! (0, -1, -2),
                        tetris_part! (1, 0, -1),
                        tetris_part! (2, 1, 0),
                        tetris_part! (3, 2, 1)
                    ]
                }
            }
            TetrisPieceType::L => {
                if piece.rotation == 1 {
                    piece.rotation = 2;
                    vec![
                        tetris_part! (0, 1, -1),
                        tetris_part! (1, 0, 0),
                        tetris_part! (2, -1, 1),
                        tetris_part! (3, 0, 2)
                    ]
                } else if piece.rotation == 2 {
                    piece.rotation = 3;
                    vec![
                        tetris_part! (0, 1, 1),
                        tetris_part! (1, 0, 0),
                        tetris_part! (2, -1, -1),
                        tetris_part! (3, -2, 0)
                    ]
                } else if piece.rotation == 3 {
                    piece.rotation = 4;
                    vec![
                        tetris_part! (0, -1, 1),
                        tetris_part! (1, 0, 0),
                        tetris_part! (2, 1, -1),
                        tetris_part! (3, 0, -2)
                    ]
                } else {
                    piece.rotation = 1;
                    vec![
                        tetris_part! (0, -1, -1),
                        tetris_part! (1, 0, 0),
                        tetris_part! (2, 1, 1),
                        tetris_part! (3, 2, 0)
                    ]
                }
            }
            TetrisPieceType::J => {
                if piece.rotation == 1 {
                    piece.rotation = 2;
                    vec![
                        tetris_part! (0, 2, 0),
                        tetris_part! (1, 1, -1),
                        tetris_part! (2, 0, 0),
                        tetris_part! (3, -1, 1)
                    ]
                } else if piece.rotation == 2 {
                    piece.rotation = 3;
                    vec![
                        tetris_part! (0, 0, 2),
                        tetris_part! (1, 1, 1),
                        tetris_part! (2, 0, 0),
                        tetris_part! (3, -1, -1)
                    ]
                } else if piece.rotation == 3 {
                    piece.rotation = 4;
                    vec![
                        tetris_part! (0, -2, 0),
                        tetris_part! (1, -1, 1),
                        tetris_part! (2, 0, 0),
                        tetris_part! (3, 1, -1)
                    ]
                } else {
                    piece.rotation = 1;
                    vec![
                        tetris_part! (0, 0, -2),
                        tetris_part! (1, -1, -1),
                        tetris_part! (2, 0, 0),
                        tetris_part! (3, 1, 1)
                    ]
                }
            }
        };

        let mut can_rotate = true;
        for part in &new_parts {
            if part.x >= H_CELLS || part.x < 0 || part.y >= V_CELLS {
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

    fn remove_line(&mut self, line_no: i64) {
        for x in 0..H_CELLS {
            let key = format!("{},{}", x, line_no);
            if self.grid.contains_key(&key) {
                if let Some(index) = self.grid.remove(&key) {
                    self.pieces[index].hide_row(line_no);
                }
            }
        }
        for y in (0..=line_no).rev() {
            for x in 0..H_CELLS {
                let key = format!("{},{}", x, y);
                if self.grid.contains_key(&key) {
                    if let Some(item) = self.grid.remove(&key) {
                        let key = format!("{},{}", x, y+1);
                        self.grid.insert(key, item);
                        self.pieces[item].lower_row(y, 1);
                    }
                }
            }
        }
    }

    pub(crate) fn check_lines(&mut self) -> i64 {
        for y in (0..V_CELLS).rev() {
            let mut all_filled = true;
            for x in 0..H_CELLS {
                let key = format!("{},{}", x, y);
                all_filled = all_filled && self.grid.contains_key(&key);
            }
            if all_filled {
                self.remove_line(y);
                return y;
            }
        }
        return -1;
    }
}