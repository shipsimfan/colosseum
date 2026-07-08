use crate::{
    logging::Logger,
    update::{ECS, ecs::ArchetypeSet},
};
use alexandria::SlotMap;

impl ECS {
    /// Create a new [`ECS`] system
    pub(in crate::update) fn new(logger: &Logger) -> ECS {
        let logger = logger.logger("ecs");
        ECS {
            entities: SlotMap::new(),
            archetypes: ArchetypeSet::new(logger.clone()),
            logger,
        }
    }
}
