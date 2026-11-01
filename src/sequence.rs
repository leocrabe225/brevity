use bevy::{prelude::*, state::state::FreelyMutableState};
use std::marker::PhantomData;

pub(super) trait Sequence: FreelyMutableState + Copy {
    fn next(self) -> Option<Self>;
    fn finish(world: &mut World);
}

#[derive(Component)]
pub(super) struct Hold<S: Sequence>(PhantomData<S>);

impl<S: Sequence> Hold<S> {
    fn new() -> Self {
        Self(PhantomData)
    }
}

pub(super) fn advance<S: Sequence>(world: &mut World) {
    let mut holds = world.query_filtered::<(), With<Hold<S>>>();
    while holds.iter(world).next().is_none() {
        let Some(current) = world.get_resource::<State<S>>().map(|s| *s.get()) else {
            break;
        };
        let Some(next) = current.next() else {
            S::finish(world);
            break;
        };
        world.resource_mut::<NextState<S>>().set(next);
        world.run_schedule(StateTransition);
    }
}
