// This unifies all the component schemas into one

mod users;

use diana::async_graphql::{MergedObject, Object as GQLObject};
use crate::schema::users::{UsersQuery, UsersMutation};

// This is a basic query to determine the version of the API endpoint (good practice)
#[derive(Default, Clone)]
pub struct BaseQuery {}
#[GQLObject]
impl BaseQuery {
    async fn api_version(&self) -> &str {
        "0.1.0"
    }
}

// Here, we combine the queries and mutations all into query and mutation roots
#[derive(Default, MergedObject, Clone)]
pub struct Query(BaseQuery, UsersQuery);
#[derive(MergedObject, Default, Clone)]
pub struct Mutation(UsersMutation);
