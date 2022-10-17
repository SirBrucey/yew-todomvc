use serde::{Deserialize, Serialize};
use yewdux::store::Store;

#[derive(Clone, Default, Debug, PartialEq, Deserialize, Serialize, Store)]
#[store(storage = "local", storage_tab_sync)]
pub struct State {
    pub entities: Vec<Entity>,
}

#[derive(Clone, Default, Debug, PartialEq, Deserialize, Serialize, Store)]
pub struct Entity {
    pub description: String,
    pub is_completed: bool,
}
