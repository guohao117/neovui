use crate::widget::style::Style;
use std::sync::Arc;

pub type Cell = Option<(String, Option<Arc<Style>>)>;

pub struct Grid {
    pub width: usize,
    pub height: usize,

    content: Vec<Cell>,
}
