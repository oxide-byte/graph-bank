use crate::domain::deposit::{CreateDeposit, Customer, Deposit};
use crate::receiver::deposit_receiver::get_deposit;
use async_graphql::extensions::Tracing;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig, GraphiQLSource};
use async_graphql::{EmptySubscription, Object, ID};
use async_graphql_axum::GraphQL;
use axum::response::IntoResponse;
use axum::routing::{get, post_service};
use axum::{response, Router};

type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;

struct Query;

#[Object]
impl Query {
    async fn deposit(&self, id: ID) -> Option<Deposit> {
        get_deposit(id)
    }

    #[graphql(entity)]
    async fn find_customer_by_id(&self, id: ID) -> Customer {
        Customer { id }
    }
}

struct Mutation;

#[Object]
impl Mutation {
    async fn create_deposit(&self, deposit: CreateDeposit) -> Deposit {
        let CreateDeposit { id, name } = deposit;
        Deposit { id, name }
    }
}

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

async fn playground() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

pub fn graph_routes() -> Router {
    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .enable_federation()
        .extension(Tracing)
        .finish();

    Router::new()
        .route("/", post_service(GraphQL::new(schema)))
        .route("/graphiql", get(graphiql))
        .route("/playground", get(playground))
}