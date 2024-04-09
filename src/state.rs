use std::{cell::RefCell, rc::Rc};

use druid::{Data, Lens};
use serde_derive::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub(crate) struct DocumentState {
    pub filepath: Option<String>,
}

#[derive(Clone, Data, Lens)]
pub(crate) struct HelloState {
    pub name: String,
    pub document: Rc<RefCell<DocumentState>>,
}