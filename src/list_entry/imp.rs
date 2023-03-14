use std::cell::RefCell;

use glib::{ParamSpec,  Value, ParamSpecString};
use gtk::glib;
use gtk::subclass::prelude::*;
use once_cell::sync::Lazy;

#[derive(Default)]
pub struct TextObject {
    text: RefCell<String>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for TextObject {
    const NAME: &'static str = "MyTextObject";
    type Type = super::TextObject;
}

// Trait shared by all GObjects
impl ObjectImpl for TextObject {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> =
            Lazy::new(|| vec![ParamSpecString::builder("text").build()]);
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
        match pspec.name() {
            "text" => {
                let text_value = value.get().expect("vaule needs to be a string");
                self.text.replace(text_value);
            }
            _ => unimplemented!(),
        }
    }

fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "text" => Value::from(self.text.borrow().to_string()),
            _ => unimplemented!(),
        }
    }
}
