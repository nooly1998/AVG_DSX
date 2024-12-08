use bevy::prelude::*;
#[derive(Clone,Event)]
pub(crate) struct GenericEvent<T> {
    pub(crate) data: T,
}