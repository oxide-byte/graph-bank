use crate::domain::customer::{CreateCustomer, Customer};
use crate::receiver::customer_receiver::get_customer;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig, GraphiQLSource};
use async_graphql::{EmptySubscription, Object, ID};
use async_graphql_axum::{GraphQL, GraphQLRequest, GraphQLResponse};
use axum::response::IntoResponse;
use axum::routing::{get, post_service};
use axum::{response, Extension, Router};

type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;

async fn graphql_handler(schema: Extension<Schema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

struct Query;

#[Object]
impl Query {
    async fn customer(&self, id: ID) -> Option<Customer> {
        get_customer(id)
    }

    #[graphql(entity)]
    async fn customer_entity_by_id(&self, id: ID) -> Option<Customer> {
        get_customer(id)
    }
}

struct Mutation;

#[Object]
impl Mutation {
    async fn create_customer(&self, customer: CreateCustomer) -> Customer {
        let CreateCustomer { id, name } = customer;
        Customer { id, name, deposit: None }
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
        .finish();

    Router::new()
        .route("/", post_service(GraphQL::new(schema)))
        .route("/graphiql", get(graphiql))
        .route("/playground", get(playground))
}