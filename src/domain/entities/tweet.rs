use crate::domain::value_object::entity_id::EntityId;

pub struct Tweet {
    id: EntityId,
}

impl Tweet {
    pub fn new() -> Self {
        Tweet {
            id: EntityId::new(),
        }
    }

    pub fn id(&self) -> EntityId {
        self.id
    }
}

impl Default for Tweet {
    fn default() -> Self {
        Self::new()
    }
}
