use crate::domain::customer::{CreateCustomer, Customer};
use crate::receiver::customer_receiver::get_customer;
use async_graphql::http::GraphiQLSource;
use async_graphql::{EmptySubscription, Object, ID};
use async_graphql_axum::{GraphQL, GraphQLRequest, GraphQLResponse};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{response, Extension, Router};

type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;

async fn graphql_handler(schema: Extension<Schema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

struct Query;

#[Object]
impl Query {
    // TODO: Fill in query AND entity resolvers
    /// This will show up in the supergraph schema as part of Query.
    async fn customer(&self, id: ID) -> Option<Customer> {
        get_customer(id)
    }

    /// This will be available to other subgraphs as an entity.
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
        Customer { id, name }
    }
}

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

pub fn graph_routes() -> Router {
    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .finish();

    Router::new()
        .route("/", get(graphiql).post_service(GraphQL::new(schema)))
}