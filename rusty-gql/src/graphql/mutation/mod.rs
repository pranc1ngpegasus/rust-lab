#![allow(warnings, unused)]
use crate::graphql::*;
use rusty_gql::*;

#[derive(Clone)]
pub struct Mutation;

#[GqlType]
impl Mutation {}
