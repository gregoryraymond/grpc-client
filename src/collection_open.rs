use std::{path::PathBuf, thread};

use druid::{widget::{Button, Flex, Label, TextBox}, Env, ExtEventSink, FileInfo, Selector, Target, UnitPoint, Widget};
use druid::WidgetExt;
use crate::state::HelloState;
use crate::grpc_build::grpc_build;

use dyon::{error, run};

use nonepad::widgets::window::{NPWindow, NPWindowState};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;

pub(crate) const FINISH_SLOW_FUNCTION: Selector<String> = Selector::new("finish_slow_function");
pub(crate) const LAUNCH_EDITOR: Selector<Vec<PathBuf>> = Selector::new("launch_editor");

pub(crate) fn build_collection_window(sink: ExtEventSink, files: Vec<FileInfo>) -> impl Widget<HelloState> {
    thread::spawn(move || {
        let _ = sink.submit_command(FINISH_SLOW_FUNCTION, String::from("Compiling..."), Target::Auto);
        match grpc_build(files.iter().map(|x| x.path.clone()).collect()) {
            Ok(x) => {
                let _ = sink.submit_command(LAUNCH_EDITOR, x, Target::Auto);
            },
            Err(e) => {
                let _ = sink.submit_command(FINISH_SLOW_FUNCTION, e.to_string(), Target::Auto);
            }
        }
    });

    // a label that will determine its text based on the current app data.
    let label = Label::new(move |data: &HelloState, _env: &Env| {
        data.name.to_string()
    })
    .with_line_break_mode(druid::widget::LineBreaking::WordWrap)
    .with_text_size(16.0);

    // arrange the two widgets vertically, with some padding
    Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .align_vertical(UnitPoint::CENTER)
}

pub fn build_editor_window(sink: ExtEventSink, libs: &Vec<PathBuf>) -> impl Widget<NPWindowState> {
    let execute = Button::new("Execute").on_click(|ctx, state: &mut NPWindowState, _env| {
        //TODO: Push to a console instead of standard out
        error(run(state.editor.buffer.to_string().as_str()));
    });
    // arrange the two widgets vertically, with some padding
    Flex::column()
        .with_child(NPWindow::build())
        .with_default_spacer()
        .with_child(execute)
        .align_vertical(UnitPoint::CENTER)
}