use std::cmp::Ordering;
use std::rc::Rc;

use cursive::Cursive;
use cursive::vec::Vec2;
use cursive::view::ScrollBase;

pub type ColumnKey = str;

/// A trait for displaying and sorting items inside a `TableView`.
pub trait SheetViewItem: Clone + Sized {
    /// Method returning a string representation of the item for the
    /// specified column.
    fn to_column(&self, column: &ColumnKey) -> String;

    /// Method comparing two items via their specified column.
    fn cmp(&self, other: &Self, column: &ColumnKey) -> Ordering;
}

/// Callback for when a column is sorted. Takes the column and ordering as input.
type OnSortCallback = Rc<dyn Fn(&mut Cursive, &ColumnKey, Ordering)>;

/// Callback taking as argument the row and the index of an element.
type IndexCallback = Rc<dyn Fn(&mut Cursive, usize, usize)>;

pub struct SpreadsheetView<T: SheetViewItem> {
    enabled: bool,
    scrollbase: ScrollBase,
    last_size: Vec2,

    column_select: bool,
    // columns: Vec<TableColumn<H>>,
    // column_indicies: HashMap<H, usize>,

    focus: usize,
    items: Vec<T>,
    rows_to_items: Vec<usize>,

    on_sort: Option<OnSortCallback>,
    // TODO Pass drawing offsets into the handlers so a popup menu
    // can be created easily?
    on_submit: Option<IndexCallback>,
    on_select: Option<IndexCallback>,
}


#[cfg(test)]
mod tests {}
