#![allow(warnings, unused)]
use crate::graphql::*;
use rusty_gql::*;
mod get_by_name;

#[derive(Clone)]
pub struct Query;

#[GqlType]
impl Query {
    pub async fn getByName(
        &self,
        ctx: &Context<'_>,
        input: Option<GetByNameInput>,
    ) -> Option<Organization> {
        get_by_name::getByName(&ctx, input).await
    }
}
