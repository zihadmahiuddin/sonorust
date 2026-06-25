use std::collections::HashMap;

use sonorust_ir::IRValue;
use tracing::warn;

use crate::{
    blocks::ReadableBlock,
    ids::{ArchetypeId, EntityId},
};

#[derive(Debug)]
pub struct PlayEntityInfoArray {
    pub items: HashMap<EntityId, PlayEntityInfo>,
}

impl PlayEntityInfoArray {
    pub const BLOCK_ID: u64 = 4103;

    pub fn new<'a>(entities: impl Iterator<Item = (&'a EntityId, ArchetypeId)>) -> Self {
        Self {
            items: entities
                .map(|(entity_id, archetype_id)| {
                    (
                        *entity_id,
                        PlayEntityInfo {
                            index: **entity_id,
                            archetype_id,
                            state: EntityState::Waiting,
                        },
                    )
                })
                .collect(),
        }
    }

    pub fn entry(&self, entity_id: &EntityId) -> Option<&PlayEntityInfo> {
        self.items.get(entity_id)
    }

    pub fn entry_mut(&mut self, entity_id: &EntityId) -> Option<&mut PlayEntityInfo> {
        self.items.get_mut(entity_id)
    }
}

impl ReadableBlock for PlayEntityInfoArray {
    fn read(&self, index: usize) -> Option<IRValue> {
        let entity_index = index / PlayEntityInfo::SIZE;
        let index_in_entity_info = index % PlayEntityInfo::SIZE;
        let entity_id = EntityId(entity_index);
        match self.entry(&entity_id) {
            Some(entity) => entity.read(index_in_entity_info),
            None => {
                warn!("Attempted to read PlayEntityInfo of non-existent entity {entity_index}");
                None
            }
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum EntityState {
    #[default]
    Waiting,
    Active,
    Despawned,
}

impl TryFrom<IRValue> for EntityState {
    type Error = ();

    fn try_from(value: IRValue) -> Result<Self, Self::Error> {
        Ok(match value {
            0.0 => EntityState::Waiting,
            1.0 => EntityState::Active,
            2.0 => EntityState::Despawned,
            _ => return Err(()),
        })
    }
}

impl From<EntityState> for IRValue {
    fn from(value: EntityState) -> Self {
        match value {
            EntityState::Waiting => 0.0,
            EntityState::Active => 1.0,
            EntityState::Despawned => 2.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayEntityInfo {
    pub index: usize,
    pub archetype_id: ArchetypeId,
    pub state: EntityState,
}

impl PlayEntityInfo {
    pub const BLOCK_ID: u64 = 4003;

    pub const SIZE: usize = 3;
}

impl ReadableBlock for PlayEntityInfo {
    fn read(&self, index: usize) -> Option<IRValue> {
        match index {
            0 => Some(self.index as IRValue),
            1 => Some(self.archetype_id.0 as IRValue),
            2 => Some(self.state.into()),
            other => {
                warn!("Attempted to read from out of bounds index {other} on PlayEntityInfo");
                None
            }
        }
    }
}
