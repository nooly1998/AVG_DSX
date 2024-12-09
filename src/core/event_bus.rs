use bevy::prelude::*;
/// A generic event structure that can hold any type of data.
/// This event can be used within the Bevy framework's event system.
#[derive(Clone, Event)]
pub(crate) struct GenericEvent<T> {
    /// The data associated with the event.
    pub(crate) data: T,
}