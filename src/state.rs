use std::ops::{Deref, DerefMut};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum StateValue {
    /// Up and running - feed stuff
    Running,
    /// Halted for some reason - can be resumed
    Halted,
}

/// A state with a default `running` state.
/// (as there is no way to default an enum).
///
/// [Deref] and [DerefMut] are implemented for this type, so one
/// can simply use it as a [StateValue].
#[derive(Debug)]
pub struct SimpleState {
    value: StateValue,
}

impl Default for SimpleState {
    fn default() -> Self {
        Self {
            value: StateValue::Running,
        }
    }
}

impl Deref for SimpleState {
    type Target = StateValue;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for SimpleState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl AsRef<StateValue> for SimpleState {
    fn as_ref(&self) -> &StateValue {
        &self.value
    }
}

impl AsMut<StateValue> for SimpleState {
    fn as_mut(&mut self) -> &mut StateValue {
        &mut self.value
    }
}
