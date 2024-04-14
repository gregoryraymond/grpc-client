use std::fs::File;

use druid::{commands, widget::{Button, Flex, Label, TextBox}, AppDelegate, Command, DelegateCtx, Env, FileDialogOptions, FileInfo, FileSpec, Handled, Target, UnitPoint, Widget, WindowDesc, WindowHandle, WindowId};
use druid::WidgetExt;
use std::io::Read;
use crate::{collection_open::build_collection_window, state::{Delegate, HelloState}};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;

pub(crate) fn build_root_widget() -> impl Widget<HelloState> {
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
            String::from("")
        }
    })
    .with_text_size(32.0);

    let proto = FileSpec::new("Protobuf file(s)", &["proto"]);
    let grpccoll = FileSpec::new("gRPC collection file", &["grpccoll"]);

    let open_dialog_options = FileDialogOptions::new()
        .allowed_types(vec![proto, grpccoll])
        .default_type(proto)
        .multi_selection()
        .name_label("Source")
        .title("Choose the protobuf file(s) to import")
        .button_text("Import");

    // a textbox that modifies `name`.
    let textbox = TextBox::new()
        .with_placeholder("gRPC Proto File Path")
        .with_text_size(18.0)
        .disabled_if(|_, _| { true })
        .fix_width(TEXT_BOX_WIDTH)
        .lens(HelloState::name);

    let open = Button::new("Open").on_click(move |ctx, _, _| {
        ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(open_dialog_options.clone()))
    });

    // arrange the two widgets vertically, with some padding
    Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(textbox)
        .with_child(open)
        .align_vertical(UnitPoint::CENTER)
}

impl Delegate {
    pub fn open_file(&mut self, data: &mut HelloState, info: &FileInfo) -> Option<()> {
        let file = File::open(info.path());
        match file {
            Ok(mut x) => {
                let mut buf = Vec::with_capacity(1);
                if let Err(e) = x.read_exact(buf.as_mut_slice()) {
                    println!("Error opening file: {e}");
                    return None;
                }
                if let Some(s) = data.document.borrow_mut().proto_files.as_mut() {
                    s.push(info.path().to_string_lossy().to_string());
                }
            },
            Err(e) => {
                println!("Error opening file: {e}");
                return None;
            }
        }
        return Some(())
    }
}

impl AppDelegate<HelloState> for Delegate {
    fn command(
        &mut self,
        ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut HelloState,
        _env: &Env,
    ) -> Handled {
        if let Some(file_info) = cmd.get(commands::SAVE_FILE_AS) {
            data.document.borrow_mut().collection_path = Some(file_info.path().to_string_lossy().to_string());
            return Handled::Yes;
        }
        if let Some(info) = cmd.get(commands::OPEN_FILE) {
            self.open_file(data, &info);
            ctx.new_window(WindowDesc::new(build_collection_window()).title("gRPC Collection").window_size((800.0, 600.0)));
            return Handled::Yes;
        }
        if let Some(file_info) = cmd.get(commands::OPEN_FILES) {
            for info in file_info.clone() {
                self.open_file(data, &info);
            }
            ctx.new_window(WindowDesc::new(build_collection_window()).title("gRPC Collection").window_size((800.0, 600.0)));
            return Handled::Yes;
        }
        Handled::No
    }
    fn window_added(
        &mut self,
        id: WindowId,
        _handle: WindowHandle,
        _data: &mut HelloState,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
        log::info!("Window added, id: {:?}", id);
        self.windows.push(id);
    }

    fn window_removed(
        &mut self,
        id: WindowId,
        _data: &mut HelloState,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
        log::info!("Window removed, id: {:?}", id);
        if let Some(pos) = self.windows.iter().position(|x| *x == id) {
            self.windows.remove(pos);
        }
    }
}