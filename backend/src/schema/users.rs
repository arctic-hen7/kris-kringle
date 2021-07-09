use diana::async_graphql::{
    Object as GQLObject,
};

#[derive(Default, Clone)]
pub struct UsersQuery {}
#[GQLObject]
impl UsersQuery {
    async fn test(&self) -> &str {
        "test"
    }
}
#[derive(Default, Clone)]
pub struct UsersMutation {}
#[GQLObject]
impl UsersMutation {
    async fn test(&self) -> &str {
        "test"
    }
}
