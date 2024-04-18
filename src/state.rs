use std::{cell::RefCell, rc::Rc};

use druid::{Data, Lens, WindowId};
use serde_derive::{Deserialize, Serialize};

#[derive(Default)]
pub (crate) struct Delegate {
    pub windows: Vec<WindowId>,
}

#[derive(Default, Serialize, Deserialize)]
pub(crate) struct DocumentState {
    pub collection_path: Option<String>,
    pub proto_files: Option<Vec<String>>
}

#[derive(Clone, Data, Lens)]
pub(crate) struct HelloState {
    pub name: String,
    pub empty: String,
    pub document: Rc<RefCell<DocumentState>>,
}