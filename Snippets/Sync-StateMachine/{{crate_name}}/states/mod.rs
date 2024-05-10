#[allow(unused)]
use tracing::*;

macro_rules! define_state_enum {
    ($state:ident{$($sub_state:ident),+}
    implements $state_functions:ident) => {
        paste::paste! {
            $(mod [<$sub_state:lower>];)+
            mod prelude {
                $(pub(crate) use super::[<$sub_state:lower>]::*;)+
                pub(crate) use super::$state;
                pub(crate) use super::$state_functions;
                pub(crate) use super::StateShared;
            }
            use self::prelude::*;

            #[enum_dispatch::enum_dispatch($state_functions)]
            #[derive(Debug, derive_more::IsVariant)]
            pub(crate) enum $state {
                $($sub_state,)+
            }

            impl core::fmt::Display for $state {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {$(
                        Self::$sub_state(_) => write!(f, "{}", stringify!($sub_state)),
                    )+}
                }
            }
         }
    };
}

define_state_enum!({{camel_name}}State {
    Before,
    Inside,
    After
} implements {{camel_name}}StateFunctions);

impl {{camel_name}}State {
    pub(crate) fn new() -> Self {
        Before::default().into()
    }
}

#[enum_dispatch::enum_dispatch]
pub(crate) trait {{camel_name}}StateFunctions {
    fn entry(&mut self, _shared: &mut StateShared) -> Option<{{camel_name}}State> {
        None
    }
    fn exit(&mut self, _shared: &mut StateShared) {}

    #[allow(unused)]
    fn input(&mut self, _shared: &mut StateShared, _c: char) -> Option<{{camel_name}}State> {
        None
    }
}

#[derive(Debug, Default)]
pub(crate) struct StateShared {
    // Any shared state between all states go here
}

pub(crate) fn transition(
    state: &mut {{camel_name}}State,
    shared: &mut StateShared,
    mut next_state: Option<{{camel_name}}State>,
) {
    while let Some(new_state) = next_state {
        if core::mem::discriminant(state) == core::mem::discriminant(&new_state) {
            break;
        }
        trace!(%state, %new_state, "Transitioning state");
        state.exit(shared);
        *state = new_state;
        next_state = state.entry(shared);
    }
}
