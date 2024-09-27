mod widgets;

use std::fs::read_to_string;

use anathema::prelude::*;
use widgets::{
    item::ItemComponent,
    next_item::{NextItemComponent, NextItemComponentState},
};

fn main() {
    let template = read_to_string("src/templates/index.aml").unwrap();

    let doc = Document::new(template);

    let backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_raw_mode()
        .hide_cursor()
        .finish()
        .unwrap();

    let mut runtime = Runtime::builder(doc, backend);

    let _item = runtime
        .register_component("item", "src/templates/item.aml", ItemComponent {}, ())
        .unwrap();

    let _next_item = runtime
        .register_component(
            "nextitem",
            "src/templates/next_item.aml",
            NextItemComponent {},
            NextItemComponentState::new(),
        )
        .unwrap();

    runtime.finish().unwrap().run();
}
