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
    pub async fn new() -> Self {
        let mut state = {{camel_name}}State::new();
        let mut shared = StateShared::default();

        let next = state.entry(&mut shared).await;
        trace!(%state, "Initialised state");
        transition(&mut state, &mut shared, next).await;

        Self { state, shared }
    }

    #[instrument(skip(self), fields(state = %self.state))]
    pub async fn input(&mut self, c: char) {
        let (state, shared) = self.into();
        let next = state.input(shared, c).await;
        transition(state, shared, next).await;
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

    #[test(tokio::test)]
    async fn {{crate_name}}_initialization() {
        let mut sut = {{camel_name}}::new().await;
        assert!(sut.state.is_before());

        sut.input('a').await;
        assert!(sut.state.is_inside());

        info!(%sut);
        info!(?sut);
    }
}
