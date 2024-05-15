use std::thread;

use druid::{widget::{Button, Flex, Label, TextBox}, Env, ExtEventSink, FileInfo, Selector, Target, UnitPoint, Widget};
use druid::WidgetExt;
use crate::state::HelloState;
use crate::grpc_build::grpc_build;

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;

pub(crate) const FINISH_SLOW_FUNCTION: Selector<String> = Selector::new("finish_slow_function");

pub(crate) fn build_collection_window(sink: ExtEventSink, files: Vec<FileInfo>) -> impl Widget<HelloState> {
    
    thread::spawn(move || {
        let _ = sink.submit_command(FINISH_SLOW_FUNCTION, String::from("Compiling..."), Target::Auto);
        let first_file_clone = files[0].clone();
        if let Err(e) = grpc_build(files.iter().map(|x| x.path.clone()).collect()) {
            let _ = sink.submit_command(FINISH_SLOW_FUNCTION, e.to_string(), Target::Auto);
        } else {
            let _ = sink.submit_command(FINISH_SLOW_FUNCTION, String::from(""), Target::Auto);
        }
    });

    // a label that will determine its text based on the current app data.
    let label = Label::new(move |data: &HelloState, _env: &Env| {
        data.name.to_string()
    })
    .with_line_break_mode(druid::widget::LineBreaking::WordWrap)
    .with_text_size(16.0);

    let input = TextBox::multiline()
        .with_placeholder("Your dyon script goes here.")
        .lens(HelloState::document_contents);

    let button = Button::new("Execute").on_click(move |data: &HelloState, _, _| {
        // ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(data.document_contents))
    });

    // arrange the two widgets vertically, with some padding
    Flex::column()
        .with_child(label)
        .with_default_spacer()
        .with_child(input)
        .with_default_spacer()
        .with_child(button)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .align_vertical(UnitPoint::CENTER)
}