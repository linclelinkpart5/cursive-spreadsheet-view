use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

use indexmap::IndexMap;

use cursive::Cursive;
use cursive::vec::Vec2;
use cursive::view::ScrollBase;


/// Callback for when a column is sorted. Takes the column and ordering as input.
type OnSortCallback = Rc<dyn Fn(&mut Cursive, &str, Ordering)>;

/// Callback taking as argument the row and the index of an element.
type IndexCallback = Rc<dyn Fn(&mut Cursive, usize, usize)>;

pub struct SpreadsheetView {
    // Mapping of column key to display name for column.
    columns: IndexMap<String, String>,
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
            columns: IndexMap::new(),
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

    /// Adds a column to this `SpreadsheetView`. This includes a key value along
    /// with a title for visual display.
    pub fn with_column(&mut self, key: String, title: String) -> &mut Self {
        self.columns.insert(key, title);
        self
    }

    /// Sorts the records in this `SpreadsheetView` by the specified column.
    /// This sort is stable, so multiple calls of this method with different
    /// columns will co-sort as expected.
    pub fn sort_rows(&mut self, key: &str, ascending: bool) {
        // If the key is not in the column list, just no-op.
        if self.columns.contains_key(key) {
            self.records.sort_by(|ra, rb| {
                let o = ra.get(key).cmp(&rb.get(key));
                if ascending { o } else { o.reverse() }
            })
        }
    }
}

#[cfg(test)]
mod tests {}
