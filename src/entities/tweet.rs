use crate::valueobjects::entityid::EntityId;

pub struct Tweet {
    id: EntityId,
}

impl Tweet {
    pub fn new() -> Self {
        Tweet {
            id: EntityId::new(),
        }
    }

    pub fn id(&self) -> String {
        self.id.id()
    }
}

impl Default for Tweet {
    fn default() -> Self {
        Self::new()
    }
}
