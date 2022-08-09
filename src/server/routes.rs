use std::convert::Infallible;

use crate::schema;

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Request, Schema,
};
use async_graphql_warp::GraphQLResponse;
use serde_json::json;
use warp::{filters::BoxedFilter, http::Response, reply::json, Filter, Rejection, Reply};

/// Check that server is alive
async fn health() -> Result<impl Reply, Rejection> {
    Ok(json(&json!({"ok": true})))
}

pub(super) fn make_routes() -> BoxedFilter<(impl Reply,)> {
    //Build GraphQL Schema
    let schema = schema::build_schema().finish();

    let health = warp::path::end().and_then(health);

    // GraphQL query and subscription handler
    let graphql_handler = warp::post().and(warp::path("graphql").and(
        async_graphql_warp::graphql(schema).and_then(
            |(schema, request): (Schema<_, _, _>, Request)| async move {
                Ok::<_, Infallible>(GraphQLResponse::from(schema.execute(request).await))
            },
        ),
    ));

    // GraphQL Playground
    let graphql_playground = warp::path("playground").map(|| {
        Response::builder()
            .header("Content-Type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
    });

    health.or(graphql_handler).or(graphql_playground).boxed()
}
