use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;
use std::fmt::Display;

use indexmap::IndexMap;

use cursive::Cursive;
use cursive::vec::Vec2;
use cursive::view::ScrollBase;


pub type Record<D> = HashMap<String, D>;

/// Callback for when a column is sorted. Takes the column and ordering as input.
type OnSortCallback = Rc<dyn Fn(&mut Cursive, &str, Ordering)>;

/// Callback taking as argument the row and the index of an element.
type IndexCallback = Rc<dyn Fn(&mut Cursive, usize, usize)>;

pub struct SpreadsheetView<D: Display + Ord> {
    // Mapping of column key to display name for column.
    columns: IndexMap<String, String>,
    records: Vec<Record<D>>,

    enabled: bool,
    scroll_base: ScrollBase,
    last_size: Vec2,
    read_only: bool,

    cursor_pos: (usize, usize),
    selected_cells: HashSet<(usize, usize)>,
    column_select: bool,

    on_sort: Option<OnSortCallback>,
    on_submit: Option<IndexCallback>,
    on_select: Option<IndexCallback>,
}

impl<D: Display + Ord> Default for SpreadsheetView<D> {
    /// Creates a new empty view without any columns.
    fn default() -> Self {
        Self::new()
    }
}

impl<D: Display + Ord> SpreadsheetView<D> {
    /// Creates a new empty view without any columns.
    pub fn new() -> Self {
        Self {
            columns: IndexMap::new(),
            records: Vec::new(),

            enabled: true,
            scroll_base: ScrollBase::new(),
            last_size: Vec2::new(0, 0),
            read_only: true,

            cursor_pos: (0, 0),
            selected_cells: HashSet::new(),
            column_select: false,

            on_sort: None,
            on_submit: None,
            on_select: None,
        }
    }

    /// Adds a column to this view. This includes a key value along with a title
    /// for visual display.
    pub fn add_column(&mut self, key: String, title: String) {
        self.columns.insert(key, title);
    }

    /// Chainable version of `add_column`.
    pub fn with_column(&mut self, key: String, title: String) -> &mut Self {
        self.add_column(key, title);
        self
    }

    /// Removes a column from this view.
    pub fn remove_column(&mut self, key: &str) {
        self.columns.remove(key);
    }

    /// Appends a record to the end of this view.
    pub fn push_record(&mut self, record: Record<D>) {
        self.records.push(record)
    }

    /// Chainable version of `push_record`.
    pub fn with_record(&mut self, record: Record<D>) -> &mut Self {
        self.push_record(record);
        self
    }

    /// Extends this view with the contents of an iterator containing records.
    pub fn extend_records<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Record<D>>,
    {
        self.records.extend(iter);
    }

    /// Chainable version of `extend_records`.
    pub fn with_records<I>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = Record<D>>,
    {
        self.extend_records(iter);
        self
    }

    /// Removes and returns the last record from this view, or `None` if there
    /// are no records.
    pub fn pop_record(&mut self) -> Option<Record<D>> {
        self.records.pop()
    }

    /// Removes and returns the record at a specified index from this view, or
    /// `None` if the index is out of bounds.
    pub fn remove_record(&mut self, index: usize) -> Option<Record<D>> {
        if index < self.records.len() { Some(self.records.remove(index)) }
        else { None }
    }

    /// Clears all records from this view.
    pub fn clear_records(&mut self) {
        self.records.clear();
    }

    /// Returns the number of records in this view.
    pub fn len_records(&self) -> usize {
        self.records.len()
    }

    /// Sorts the records in this view by the specified column.
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
