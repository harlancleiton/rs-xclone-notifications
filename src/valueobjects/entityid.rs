pub struct EntityId {
    id: uuid::Uuid,
}

impl EntityId {
    pub fn new() -> Self {
        EntityId {
            id: uuid::Uuid::now_v7(),
        }
    }

    pub fn id(&self) -> String {
        self.id.to_string()
    }
}

impl Default for EntityId {
    fn default() -> Self {
        Self::new()
    }
}
