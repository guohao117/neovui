use std::sync::Arc;
use crate::widget::style::Style;

pub type Cell = Option<(String, Option<Arc<Style>>)>;

pub struct Grid {
    pub width: usize,
    pub height: usize,
    
    content: Vec<Cell>,
}

