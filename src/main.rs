mod handlers;
mod root_widget;
mod state;

use std::{cell::RefCell, default, rc::Rc};

use druid::{AppLauncher, WindowDesc};
use env_logger::Builder;
use log::LevelFilter;
use state::{DocumentState, HelloState};
use root_widget::build_root_widget;

extern crate serde_derive;

fn setup_logging() {
    use std::io::Write;
    use chrono::Local;

    Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Debug)
        .init();
    }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logging();

    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
    .title("gRPC Client")
    .window_size((800.0, 600.0));

    // create the initial app state
    let initial_state: HelloState = HelloState {
        name: "".into(),
        document: Rc::new(RefCell::new(DocumentState {
            filepath: Option::None,
        })),
    };

    // start the application. Here we pass in the application state.
    AppLauncher::with_window(main_window)
        .log_to_console()
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