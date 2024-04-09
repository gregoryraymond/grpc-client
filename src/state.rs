use druid::{Data, Lens};


#[derive(Clone, Data, Lens)]
pub(crate) struct HelloState {
    pub name: String,
}