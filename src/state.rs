use serde::{Deserialize, Serialize};
use yewdux::store::Store;

#[derive(Default, Debug, PartialEq, Deserialize, Serialize, Store)]
#[store(storage = "local")]
pub struct State {
    pub entities: Vec<Entity>,
}

#[derive(Default, Debug, PartialEq, Deserialize, Serialize, Store)]
pub struct Entity {
    pub description: String,
    pub is_completed: bool,
}
