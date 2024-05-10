#[allow(unused)]
use tracing::*;

use super::prelude::*;

#[derive(Debug, Default)]
pub struct Before {}

impl {{camel_name}}StateFunctions for Before {
    fn input(&mut self, _shared: &mut StateShared, _c: char) -> Option<{{camel_name}}State> {
        Some(Inside::default().into())
    }
}
