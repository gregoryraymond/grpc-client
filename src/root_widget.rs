use druid::{widget::{Flex, Label, TextBox}, Env, UnitPoint, Widget};
use druid::WidgetExt;
use crate::state::HelloState;

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;

pub(crate) fn build_root_widget() -> impl Widget<HelloState> {
    // a label that will determine its text based on the current app data.
    let label = Label::new(|data: &HelloState, _env: &Env| {
        if data.name.len() > 0 {
            match std::fs::File::open(&data.name) {
                Ok(_) => data.document.borrow_mut().filepath = Some(data.name.clone()),
                Err(e) => {
                    log::warn!("Could not open file {e}");
                }
            }
        }

        if let Some(x) = &data.document.borrow().filepath {
            format!("Loaded {}", x.clone())
        } else {
            String::from("No file loaded.")
        }
    })
    .with_text_size(32.0);

    // a textbox that modifies `name`.
    let textbox = TextBox::new()
        .with_placeholder("gRPC Proto File Path")
        .with_text_size(18.0)
        .fix_width(TEXT_BOX_WIDTH)
        .lens(HelloState::name);

    // arrange the two widgets vertically, with some padding
    Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(textbox)
        .align_vertical(UnitPoint::CENTER)
}