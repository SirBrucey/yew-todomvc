use serde::{Deserialize, Serialize};
use yewdux::store::Store;

#[derive(Clone, Default, Debug, PartialEq, Deserialize, Serialize, Store)]
#[store(storage = "local", storage_tab_sync)]
pub struct State {
    pub entities: Vec<Entity>,
    pub edited_value: String,
}

#[derive(Clone, Default, Debug, PartialEq, Deserialize, Serialize, Store)]
pub struct Entity {
    pub description: String,
    pub is_completed: bool,
    pub editing: bool,
}

impl State {
    pub fn add(&mut self, value: String) {
        if !value.is_empty() {
            if !self
                .entities
                .iter()
                .any(|entity| entity.description == value)
            {
                self.entities.push(Entity {
                    description: value,
                    ..Default::default()
                })
            }
        }
    }

    pub fn toggle_edit(&mut self, index: usize) {
        let entity = self.get_mut_entity(index);
        entity.editing = !entity.editing;
    }

    pub fn update_description(&mut self, index: usize, value: String) {
        if value.is_empty() {
            self.entities.remove(index);
        } else {
            let entity = self.get_mut_entity(index);
            entity.description = value;
            entity.editing = !entity.editing;
        }
    }

    fn get_mut_entity(&mut self, index: usize) -> &mut Entity {
        self.entities.iter_mut().nth(index).unwrap()
    }
}
