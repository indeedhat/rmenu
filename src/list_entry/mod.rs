mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct TextObject(ObjectSubclass<imp::TextObject>);
}

impl TextObject {
    pub fn new(text: String) -> Self {
        Object::builder().property("text", text).build()
    }
}

