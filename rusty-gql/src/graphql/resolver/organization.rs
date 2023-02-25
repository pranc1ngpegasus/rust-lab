#![allow(warnings, unused)]
use crate::graphql::*;
use rusty_gql::*;
#[derive(Clone)]
pub struct Organization {
    pub id: String,
    pub name: String,
}
#[GqlType]
impl Organization {
    pub async fn id(&self, ctx: &Context<'_>) -> String {
        self.id.clone()
    }
    pub async fn name(&self, ctx: &Context<'_>) -> String {
        self.name.clone()
    }
}
