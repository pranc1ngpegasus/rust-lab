#![allow(warnings, unused)]
use crate::graphql::*;
use rusty_gql::*;
#[derive(GqlInputObject)]
pub struct GetByNameInput {
    pub name: String,
}
