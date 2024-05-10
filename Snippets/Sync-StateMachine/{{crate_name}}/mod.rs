//! This is a rudimentary state machine, implemented using static dispatching.
//!
//! # Crate requirements
//! - `tracing`
//! - `derive_more`
//! - `enum_dispatch`
//! - `paste`
//! - `test-log`
#[allow(unused)]
use tracing::*;

use self::states::*;

mod states;

#[derive(Debug, derive_more::Display)]
#[display(fmt = "{{camel_name}}{%raw%} {{ {} }}{%endraw%}", "state")]
pub struct {{camel_name}} {
    state: {{camel_name}}State,
    shared: StateShared,
}

impl {{camel_name}} {
    #[instrument]
    pub fn new() -> Self {
        let mut state = {{camel_name}}State::new();
        let mut shared = StateShared::default();

        let next = state.entry(&mut shared);
        trace!(%state, "Initialised state");
        transition(&mut state, &mut shared, next);

        Self { state, shared }
    }

    #[instrument(skip(self), fields(state = %self.state))]
    pub fn input(&mut self, c: char) {
        let (state, shared) = self.into();
        let next = state.input(shared, c);
        transition(state, shared, next);
    }
}

impl<'a> From<&'a mut {{camel_name}}> for (&'a mut {{camel_name}}State, &'a mut StateShared) {
    fn from(state: &'a mut {{camel_name}}) -> Self {
        (&mut state.state, &mut state.shared)
    }
}

#[cfg(test)]
mod tests {
    use test_log::test;
    #[allow(unused)]
    use tracing::*;

    use super::*;

    #[test]
    fn {{crate_name}}_initialization() {
        let mut sut = {{camel_name}}::new();
        assert!(sut.state.is_before());

        sut.input('a');
        assert!(sut.state.is_inside());

        info!(%sut);
        info!(?sut);
    }
}
