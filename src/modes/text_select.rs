use std::{env::{VarError, self}, fs, rc::Rc, cell::RefCell, sync::Arc};
use faccess::PathExt;
use gtk::{
    prelude::*, SignalListItemFactory, Label, ListItem, Widget, CustomFilter, CustomSorter, Ordering
};

use super::MenuMode;
use crate::{rmenu::{CliArgs, Choice}, list_entry::TextObject};

pub struct TextSelect {
    args: CliArgs
}

impl TextSelect {
    pub fn new(args: CliArgs) -> Self {
        TextSelect{ args }
    }
}

impl MenuMode for TextSelect {
    fn build_choices(&self) -> Result<Vec<Choice>, VarError> {
        Ok(self.args.user_options())
    }

    fn connect_setup(&self, list_factory: &SignalListItemFactory) {
        list_factory.connect_setup(|_, item| {
            let label = Label::new(None);
            label.add_css_class("list-item");
            label.set_xalign(0.0);

            let item = item.downcast_ref::<ListItem>()
                .expect("Needs to be a list item");

            item.set_child(Some(&label));

            item.property_expression("item")
                .chain_property::<TextObject>("text")
                .bind(&label, "label", Widget::NONE);
        });
    }

    fn custom_filter(&self, query: Rc<RefCell<String>>) -> CustomFilter{
        CustomFilter::new(move |obj| {
            let string_object = obj.downcast_ref::<TextObject>()
                .expect("The objcet nedds to be a TextObject");

            let text = string_object.property::<String>("text");


            if (*query).borrow().to_string() == "".to_string() {
                return true;
            }

            text.contains(&(*query).borrow().to_string())
        })
    }

    fn custom_sorter(&self, query: Rc<RefCell<String>>) -> CustomSorter {
        CustomSorter::new(move |obj1, obj2| {
            let string_object_1 = obj1.downcast_ref::<TextObject>()
                .expect("the object needs to be a TextObject");

            let string_object_2 = obj2.downcast_ref::<TextObject>()
                .expect("the object needs to be a TextObject");

            let query = &(*query).borrow().to_string();

            let pos_cmp = string_object_1.property::<String>("text")
                .to_lowercase()
                .find(query)
                .cmp(&string_object_2.property::<String>("text").to_lowercase().find(query))
                .into();

            if pos_cmp != Ordering::Equal {
                return pos_cmp;
            }

            string_object_1.property::<String>("text")
                .len()
                .cmp(&string_object_2.property::<String>("text").len())
                .into()
        })
    }

    fn connect_active(&self) {
        todo!()
    }
}
