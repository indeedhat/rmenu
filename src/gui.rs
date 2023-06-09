use std::{cell::RefCell, rc::Rc, path::Path};

use gtk::{
    gio,
    gio::{ListStore, SimpleAction},
    prelude::*,
    Application,
    ApplicationWindow,
    SearchBar,
    Align,
    Orientation,
    Box as GtkBox,
    SearchEntry,
    ScrolledWindow,
    SignalListItemFactory, 
    SingleSelection, 
    ListView, 
    FilterListModel, 
    CustomFilter, 
    FilterChange, 
    CustomSorter, 
    SortListModel, 
    SorterChange, 
    STYLE_PROVIDER_PRIORITY_APPLICATION, 
    gdk::Display, EventControllerKey, Inhibit, glib
};

use crate::{list_entry::TextObject, modes::MenuMode};

/// create a new gtk4 application
pub fn new<T: MenuMode + 'static + ?Sized>(mode: Box<T>) -> Application {
    let app = Application::builder()
        .application_id("com.indeedhat.rmenu")
        .build();

    app.connect_startup(|_app| {
        let provider = gtk::CssProvider::new();
        provider.load_from_path(Path::new("./config/style.css"));

        gtk::StyleContext::add_provider_for_display(
            &Display::default().expect("failed to connect to display"),
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION
        );

    });

    app.connect_activate(move |app| {
        build_ui(app, &mode);

        let close_action = SimpleAction::new("quit", None);
        close_action.connect_activate(glib::clone!(@weak app => move |_, _| {
            app.quit();
        }));

        app.add_action(&close_action);
    });

    // close app on escape key
    app.set_accels_for_action("app.quit", &["Escape"]);

    app
}

/// render the ui
fn build_ui<T: MenuMode + ?Sized>(app: &Application, mode: &Box<T>) {
    let entries = mode.build_choices().unwrap();

    let query: Rc<RefCell<String>> = Rc::new(RefCell::new("".to_string()));

    let data: Vec<TextObject> = entries.into_iter()
        .map(|entry| TextObject::new(entry.name))
        .collect();

    let data_model = gio::ListStore::new(TextObject::static_type());
    data_model.extend_from_slice(&data);


    let win = ApplicationWindow::builder()
        .application(app)
        .title("r-menu")
        .default_width(800)
        .default_height(600)
        .build();

    let selected_action = SimpleAction::new("selected", Some(&String::static_variant_type()));
    selected_action.connect_activate(move |_action, param| {
        print!("value clicked: {}", param.unwrap());
    });
    win.add_action(&selected_action);

    let container = GtkBox::new(Orientation::Vertical, 2);
    win.set_child(Some(&container));

    let (search_bar, search_entry) = build_search_bar(&win);
    container.append(&search_bar);

    let (list_scroll_view, filter, sorter) = build_list_view(mode, data_model, &query);
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

fn build_list_view<T: MenuMode + ?Sized>(
    mode: &Box<T>,
    data_model: ListStore,
    query: &Rc<RefCell<String>>
) -> (
    ScrolledWindow,
    CustomFilter,
    CustomSorter
 ) {
    let list_factory = SignalListItemFactory::new();

    mode.connect_setup(&list_factory);

    let filter = mode.custom_filter(query.clone());
    let filter_model = FilterListModel::new(Some(data_model), Some(filter.clone()));

    let sorter = mode.custom_sorter(query.clone());
    let sorter_model = SortListModel::new(Some(filter_model), Some(sorter.clone()));

    let selection_model = SingleSelection::new(Some(sorter_model));
    let list_view = ListView::new(Some(selection_model), Some(list_factory));
    list_view.add_css_class("list");
    list_view.connect_activate(|list_view, position| {
        let model = list_view.model().expect("failed to get list model");
        let text = model.item(position)
            .and_downcast::<TextObject>()
            .expect("fail to get text object")
            .property::<String>("text");

        list_view.activate_action("win.selected", Some(&text.to_variant()))
            .expect("failed to trigger window.selected action");
        list_view.activate_action("app.quit", None)
            .expect("failed to trigger app.quit action");
    });

    let list_scroll_view = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vexpand(true)
        .min_content_width(360)
        .child(&list_view)
        .build();


    (list_scroll_view, filter, sorter)
}
