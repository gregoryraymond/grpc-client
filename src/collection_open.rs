use std::fs::File;

use druid::{commands, widget::{Button, Flex, Label, TextBox}, AppDelegate, Command, DelegateCtx, Env, FileDialogOptions, FileSpec, Handled, Target, UnitPoint, Widget};
use druid::WidgetExt;
use std::io::Read;
use crate::state::HelloState;

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;

pub(crate) fn build_collection_window() -> impl Widget<HelloState> {
    // a label that will determine its text based on the current app data.
    let label = Label::new(|data: &HelloState, _env: &Env| {
        if data.name.len() > 0 {
            match std::fs::File::open(&data.name) {
                Ok(_) => data.document.borrow_mut().collection_path = Some(data.name.clone()),
                Err(e) => {
                    log::warn!("Could not open file {e}");
                }
            }
        }

        if let Some(x) = &data.document.borrow().collection_path {
            format!("Loaded {}", x.clone())
        } else {
            String::from("No file loaded.")
        }
    })
    .with_text_size(32.0);

    // arrange the two widgets vertically, with some padding
    Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .align_vertical(UnitPoint::CENTER)
}