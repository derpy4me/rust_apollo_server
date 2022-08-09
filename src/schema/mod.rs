mod health;

use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema, SchemaBuilder};

#[derive(MergedObject, Default)]
pub struct Query(health::HealthQuery);

/// Build GraphQL Schema
pub fn build_schema() -> SchemaBuilder<Query, EmptyMutation, EmptySubscription> {
    Schema::build(Query::default(), EmptyMutation, EmptySubscription)
}
