#[derive(Debug, PartialEq, Clone)]
pub(crate) struct TetrisPart {
    pub(crate) x: i64,
    pub(crate) y: i64
}

impl TetrisPart {
    pub(crate) fn new(x: i64, y: i64) -> TetrisPart {
        TetrisPart {
            x,
            y
        }
    }
}