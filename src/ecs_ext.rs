use bevy::prelude::*;

pub(crate) trait InsertNewExt {
    fn insert_new<T: Component>(&mut self, component: T) -> &mut Self;
}

impl InsertNewExt for EntityCommands<'_> {
    fn insert_new<T: Component>(&mut self, component: T) -> &mut Self {
        self.queue(move |mut entity: EntityWorldMut| -> Result<()> {
            if entity.contains::<T>() {
                return Err(BevyError::panic(format!(
                    "{} already exists on {}",
                    ShortName::of::<T>(),
                    entity.id(),
                )));
            }
            entity.insert(component);
            Ok(())
        });
        self
    }
}
