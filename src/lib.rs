use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

use cursive::Cursive;
use cursive::vec::Vec2;
use cursive::view::ScrollBase;


pub struct ColumnDef {
    pub key: String,
    pub display: String,
}

/// Callback for when a column is sorted. Takes the column and ordering as input.
type OnSortCallback = Rc<dyn Fn(&mut Cursive, &str, Ordering)>;

/// Callback taking as argument the row and the index of an element.
type IndexCallback = Rc<dyn Fn(&mut Cursive, usize, usize)>;

pub struct SpreadsheetView {
    columns: Vec<ColumnDef>,
    records: Vec<HashMap<String, String>>,

    enabled: bool,
    scroll_base: ScrollBase,
    last_size: Vec2,
    read_only: bool,

    selected_cells: HashSet<(usize, usize)>,
    column_select: bool,

    on_sort: Option<OnSortCallback>,
    on_submit: Option<IndexCallback>,
    on_select: Option<IndexCallback>,
}

impl Default for SpreadsheetView {
    /// Creates a new empty `SpreadsheetView` without any columns.
    fn default() -> Self {
        Self::new()
    }
}

impl SpreadsheetView {
    /// Creates a new empty `SpreadsheetView` without any columns.
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            records: Vec::new(),

            enabled: true,
            scroll_base: ScrollBase::new(),
            last_size: Vec2::new(0, 0),
            read_only: true,

            selected_cells: HashSet::new(),
            column_select: false,

            on_sort: None,
            on_submit: None,
            on_select: None,
        }
    }
}

#[cfg(test)]
mod tests {}
