use std::{fs::File, path::PathBuf, str::FromStr, thread};

use druid::{commands, widget::{Button, Flex, Label, TextBox}, AppDelegate, Command, DelegateCtx, Env, ExtEventSink, FileDialogOptions, FileInfo, FileSpec, Handled, Selector, Target, UnitPoint, Widget};
use druid::WidgetExt;
use std::io::Read;
use crate::state::HelloState;
use crate::grpc_build::grpc_build;

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;

pub(crate) const FINISH_SLOW_FUNCTION: Selector<String> = Selector::new("finish_slow_function");

pub(crate) fn build_collection_window(sink: ExtEventSink, files: Vec<FileInfo>) -> impl Widget<HelloState> {
    
    thread::spawn(move || {
        let first_file_clone = files[0].clone();
        if let Err(e) = grpc_build(files.iter().map(|x| x.path.clone()).collect()) {
            sink.submit_command(FINISH_SLOW_FUNCTION, e.to_string(), Target::Auto)
                .expect("command failed to submit");
        } else {
            sink.submit_command(FINISH_SLOW_FUNCTION, format!("Loaded {:?}", first_file_clone), Target::Auto)
                .expect("command failed to submit");
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