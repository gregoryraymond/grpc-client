mod compile;
mod handlers;
mod root_widget;
mod state;
mod collection_open;
mod grpc_build;

use std::{cell::RefCell, rc::Rc};

use druid::{AppLauncher, WindowDesc};
use state::{Delegate, DocumentState, HelloState};
use root_widget::build_root_widget;

extern crate serde_derive;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
    .title("gRPC Client")
    .window_size((800.0, 600.0));

    // create the initial app state
    let initial_state: HelloState = HelloState {
        name: "".into(),
        document: Rc::new(RefCell::new(DocumentState {
            collection_path: Option::None,
            proto_files: Some(vec![]),
        })),
        empty: "".into(),
        document_contents: String::new(),
    };

    // start the application. Here we pass in the application state.
    AppLauncher::with_window(main_window)
        .log_to_console()
        .delegate(Delegate::default())
        .launch(initial_state)?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use std::thread::Thread;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_example() -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}