use std::{env::{VarError, self}, fs, rc::Rc, cell::RefCell};
use faccess::PathExt;
use gtk::{
    prelude::*, SignalListItemFactory, Label, ListItem, Widget, CustomFilter, CustomSorter
};

use super::MenuMode;
use crate::{rmenu::{CliArgs, Choice}, list_entry::TextObject};

pub struct Dmenu {
    args: CliArgs
}

impl Dmenu {
    pub fn new(args: CliArgs) -> Self {
        Dmenu{ args }
    }
}

impl MenuMode for Dmenu {
    fn build_choices(&self) -> Result<Vec<Choice>, VarError> {
        let mut bins: Vec<String> = vec![];
        // let mut bins: HashMap<String, String> = HashMap::new();
        let path = env::var("PATH")?;

        for dir in path.split(":") {
            for bin in bins_in_dir(dir.to_string()) {
                if bins.contains(&bin.to_string()) {
                    continue;
                }

                // this seems excessive
                bins.push(bin.clone());
            }
        }

        let mut choices: Vec<Choice> = vec![];
        for bin in bins {
            choices.push(Choice {
                value: bin.to_string(),
                name: bin.to_string(),
                icon: "".to_string()
            })
        }

        Ok(choices)
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

            string_object_1.property::<String>("text")
                .to_lowercase()
                .find(query)
                .cmp(&string_object_2.property::<String>("text").to_lowercase().find(query))
                .into()
        })
    }

    fn connect_active(&self) {
        todo!()
    }
}


/// list binaries in the given directory
fn bins_in_dir(path: String) -> Vec<String> {
    let mut bins: Vec<String> = vec![];
    let paths = match fs::read_dir(path) {
        Ok(paths) => paths,
        Err(_) => return bins
    };

    for path in paths {
        match path {
            Ok(path) => {
                if path.path().executable() {
                    bins.push(path.file_name().to_str().unwrap().to_string());
                }
            },
            _ => continue
        }
    }

    bins
}
