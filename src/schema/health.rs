use async_graphql::Object;

#[derive(Default)]
pub struct HealthQuery;

#[Object]
impl HealthQuery {
    /// Returns `true` to show GraphQL is up
    async fn health(&self) -> bool {
        true
    }
}
