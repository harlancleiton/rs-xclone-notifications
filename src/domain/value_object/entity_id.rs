use uuid::Uuid;

#[derive(Clone, Copy)]
pub struct EntityId {
    id: Uuid,
}

impl EntityId {
    pub fn new() -> Self {
        EntityId {
            id: Uuid::now_v7(),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}

impl Default for EntityId {
    fn default() -> Self {
        EntityId {
            id: Uuid::nil(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let entity_id = EntityId::new();
        assert!(!entity_id.id().is_nil())
    }

    #[test]
    fn test_default() {
        let entity_id = EntityId::default();
        assert!(entity_id.id().is_nil());
    }
}