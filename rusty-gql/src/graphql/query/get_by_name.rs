#![allow(warnings, unused)]
use crate::graphql::*;
use rusty_gql::*;
use uuid::Uuid;

pub async fn getByName(ctx: &Context<'_>, input: Option<GetByNameInput>) -> Option<Organization> {
    let all_orgs = vec![Organization {
        id: Uuid::new_v4().to_string(),
        name: "bar".to_string(),
    }];

    match input {
        Some(input) => all_orgs.into_iter().find(|x| x.name == input.name),
        None => None,
    }
}
