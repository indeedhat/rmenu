use std::{cell::RefCell, rc::Rc};

use gtk::{
    gio,
    gio::ListStore,
    prelude::*,
    Application,
    ApplicationWindow,
    SearchBar,
    Align,
    Orientation,
    Box,
    SearchEntry,
    Label,
    ScrolledWindow,
    SignalListItemFactory, 
    ListItem, 
    SingleSelection, 
    ListView, 
    FilterListModel, 
    CustomFilter, 
    FilterChange, 
    Widget, 
    CustomSorter, 
    SortListModel, 
    SorterChange
};

use crate::{fs::Choice, list_entry::TextObject};

/// create a new gtk4 application
pub fn new(entries: Vec<Choice>) -> Application {
    let app = Application::builder()
        .application_id("com.indeedhat.rmenu")
        .build();

    let en = entries.clone();

    app.connect_activate(move |app| build_ui(app, en.to_vec()));

    app
}

/// render the ui
fn build_ui(app: &Application, entries: Vec<Choice>) {
    let query: Rc<RefCell<String>> = Rc::new(RefCell::new("".to_string()));

    let data: Vec<TextObject> = entries.into_iter()
        .map(|entry| TextObject::new(entry.name))
        .collect();

    let data_model = gio::ListStore::new(TextObject::static_type());
    data_model.extend_from_slice(&data);


    let win = ApplicationWindow::builder()
        .application(app)
        .title("rmenu")
        .default_width(800)
        .default_height(600)
        .build();

    let container = Box::new(Orientation::Vertical, 2);
    win.set_child(Some(&container));

    let (search_bar, search_entry) = build_search_bar(&win);
    container.append(&search_bar);

    let (list_scroll_view, filter, sorter) = build_list_view(data_model, &query);
    container.append(&list_scroll_view);

    let search_query = query.clone();
    search_entry.connect_search_changed(move |entry| {
        *search_query.borrow_mut() = entry.text().to_string();
        filter.changed(FilterChange::Different);
        sorter.changed(SorterChange::Different);
    });

    win.present();
}

/// build up the search bar widget
fn build_search_bar(win: &ApplicationWindow) -> (SearchBar, SearchEntry) {
    let search_bar = SearchBar::builder()
        .valign(Align::Start)
        .key_capture_widget(win)
        .build();

    search_bar.set_property("search-mode-enabled", true);

    let entry = SearchEntry::new();
    entry.set_hexpand(true);
    search_bar.set_child(Some(&entry));

    (search_bar, entry)
}

fn build_list_view(
    data_model: ListStore,
    query: &Rc<RefCell<String>>
) -> (
    ScrolledWindow,
    CustomFilter,
    CustomSorter
 ) {
    let list_factory = SignalListItemFactory::new();

    list_factory.connect_setup(|_, item| {
        let label = Label::new(None);

        let item = item.downcast_ref::<ListItem>()
            .expect("Needs to be a list item");

        item.set_child(Some(&label));

        item.property_expression("item")
            .chain_property::<TextObject>("text")
            .bind(&label, "label", Widget::NONE);
    });

    let filter_query = query.clone();
    let filter = CustomFilter::new(move |obj| {
        let string_object = obj.downcast_ref::<TextObject>()
            .expect("The objcet nedds to be a TextObject");

        let text = string_object.property::<String>("text");


        if (*filter_query).borrow().to_string() == "".to_string() {
            return true;
        }

        text.contains(&(*filter_query).borrow().to_string())
    });


    let filter_model = FilterListModel::new(Some(data_model), Some(filter.clone()));

    let sorter_query = query.clone();
    let sorter = CustomSorter::new(move |obj1, obj2| {
        let string_object_1 = obj1.downcast_ref::<TextObject>()
            .expect("the object needs to be a TextObject");

        let string_object_2 = obj2.downcast_ref::<TextObject>()
            .expect("the object needs to be a TextObject");

        let query = &(*sorter_query).borrow().to_string();

        string_object_1.property::<String>("text")
            .to_lowercase()
            .find(query)
            .cmp(&string_object_2.property::<String>("text").to_lowercase().find(query))
            .into()
    });

    let sorter_model = SortListModel::new(Some(filter_model), Some(sorter.clone()));

    let selection_model = SingleSelection::new(Some(sorter_model));
    let list_view = ListView::new(Some(selection_model), Some(list_factory));

    let list_scroll_view = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vexpand(true)
        .min_content_width(360)
        .child(&list_view)
        .build();

    (list_scroll_view, filter, sorter)
}
