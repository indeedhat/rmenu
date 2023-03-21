use std::{env::VarError, rc::Rc, cell::RefCell};

use gtk::{SignalListItemFactory, CustomFilter, CustomSorter};

use crate::rmenu::{Choice, CliArgs};

mod app_info;
pub mod dmenu;
mod text_select;


pub trait MenuMode {
    /// build the list of choices to be displayed in the list
    fn build_choices(&self) -> Result<Vec<Choice>, VarError>;

    /// callback to be passed to SignalListItemFactory.connect_setup
    fn connect_setup(&self, list_factory: &SignalListItemFactory);

    /// callback to be passed to CustomFilter::new
    fn custom_filter(&self, query: Rc<RefCell<String>>) -> CustomFilter;
    
    /// callback to be passed to CustomSorter::new
    fn custom_sorter(&self, query: Rc<RefCell<String>>) -> CustomSorter;

    /// callback to be called when a list item is clicked (still dont know what the method for that
    /// is)
    fn connect_active(&self);
}
