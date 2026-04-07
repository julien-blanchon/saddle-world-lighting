use bevy::prelude::*;

pub(super) fn entity_by_name<T: Component>(world: &mut World, target_name: &str) -> Option<Entity> {
    let mut query = world.query_filtered::<(Entity, &Name), With<T>>();
    query
        .iter(world)
        .find_map(|(entity, name)| (name.as_str() == target_name).then_some(entity))
}
