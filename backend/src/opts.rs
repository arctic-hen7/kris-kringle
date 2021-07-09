use crate::{Query, Mutation, ctx::Context};
use diana::async_graphql::EmptySubscription;
use diana::{Options, OptionsBuilder, AuthBlockLevel};
use std::env;

pub fn get_opts() -> Options<Context, Query, Mutation, EmptySubscription> {
    dotenv::from_filename("./backend/.env.local").expect("Couldn't load environment variables. Have you run `bonnie setup`?");
    OptionsBuilder::new()
        .ctx(Context {
            pool: "test".to_string()
        })
        .auth_block_state(AuthBlockLevel::AllowMissing) // TODO fix in prod
        .jwt_secret(
            &env::var("JWT_SECRET").expect("Couldn't load `JWT_SECRET` environment variable. Have you run `bonnie setup`?")
        )
        // We use `::default()` because these are merged roots
        .schema(Query::default(), Mutation::default(), EmptySubscription {})
        .finish()
        .expect("Couldn't build options.")
}
