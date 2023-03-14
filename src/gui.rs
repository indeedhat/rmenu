use std::{borrow::Borrow, cell::RefCell, rc::Rc};

use gtk::{
    gio,
    prelude::*,
    Application,
    ApplicationWindow,
    SearchBar,
    Align,
    Orientation,
    Box,
    SearchEntry,
    // ListBox,
    Label,
    ScrolledWindow,
    // FilterListModel,
    // CustomFilter, 
    SignalListItemFactory, 
    ListItem, 
    // glib::List, 
    SingleSelection, 
    ListView, FilterListModel, CustomFilter, FilterChange
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

    let list_factory = SignalListItemFactory::new();

    list_factory.connect_setup(|_, item| {
        let label = Label::new(None);

        item.downcast_ref::<ListItem>()
            .expect("Needs to be a list item")
            .set_child(Some(&label));
    });

    list_factory.connect_bind(|_, item| {
        let text_object = item.downcast_ref::<ListItem>()
            .expect("Needs to be a ListItem")
            .item()
            .and_downcast::<TextObject>()
            .expect("Needs to be a TextObject");

        let text = text_object.property::<String>("text");

        let label = item.downcast_ref::<ListItem>()
            .expect("Needs to be a ListItem")
            .child()
            .and_downcast::<Label>()
            .expect("Needs to be a Label");

        label.set_label(&text.to_string());
    });

    let filter_query = query.clone();
    let filter = CustomFilter::new(move |obj| {
        println!("filter {}", (*filter_query).borrow());
        let string_object = obj.downcast_ref::<TextObject>()
            .expect("The objcet nedds to be a TextObject");

        let text = string_object.property::<String>("text");


        if (*filter_query).borrow().to_string() == "".to_string() {
            return true;
        }

        text.contains(&(*filter_query).borrow().to_string())
    });


    let filter_model = FilterListModel::new(Some(data_model), Some(filter.clone()));

    let selection_model = SingleSelection::new(Some(filter_model));
    let list_view = ListView::new(Some(selection_model), Some(list_factory));

    let list_scroll_view = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vexpand(true)
        .min_content_width(360)
        .child(&list_view)
        .build();

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

    container.append(&list_scroll_view);

    let search_query = query.clone();
    search_entry.connect_search_changed(move |entry| {
        *search_query.borrow_mut() = entry.text().to_string();
        filter.changed(FilterChange::Different);
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
