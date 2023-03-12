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
    ListBox,
    Label,
    ScrolledWindow,
    FilterListModel,
    CustomFilter
};

use crate::{fs::Choice, list_entry::TextObject};

/// create a new gtk4 application
pub fn new(entries: Vec<Choice>) -> Application {
    let app = Application::builder()
        .application_id("com.indeedhat.rmenu")
        .build();

    app.connect_activate(move |app| build_ui(app, &entries));

    app
}

/// render the ui
fn build_ui(app: &Application, entries: Vec<Choice>) {
    let win = ApplicationWindow::builder()
        .application(app)
        .title("rmenu")
        .default_width(800)
        .default_height(600)
        .build();

    let data: Vec<TextObject> = entries.into_iter()
        .map(|entry| TextObject::new(entry.name))
        .collect();

    let data_model = gio::ListStore::new(TextObject::static_type());
    data_model.extend_from_slice(&data);
    // TODO: got to here (https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/list_widgets/2/main.rs)
    let container = Box::new(Orientation::Vertical, 2);
    win.set_child(Some(&container));

    let (search_bar, search_entry) = build_search_bar(&win);
    container.append(&search_bar);

    let list = build_list_view(entries);
    container.append(&list);

    search_entry.connect_search_changed(|entry| {
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

/// build up the list view widget
fn build_list_view(entries: &Vec<Choice>, input: &SearchEntry) -> FilterListModel {
    let list_box = ListBox::new();
    list_box.set_vexpand(true);

    for entry in entries {
        let label = Label::new(Some(&entry.name.to_string()));
        label.set_halign(Align::Start);

        list_box.append(&label);
    }

     let scroll_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .child(&list_box)
        .build();

    let filter = CustomFilter::new(|| {
        
    });
}
