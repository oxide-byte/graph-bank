use async_graphql::{InputObject, SimpleObject, ID};

#[derive(SimpleObject)]
#[graphql(shareable)]
pub struct Deposit {
    pub id: ID,
    pub name: String,
}

#[derive(SimpleObject)]
pub struct Customer {
    pub id: ID,
    pub name: String,
    // Provided by external subgraph (graph-deposit)
    pub deposit: Option<Deposit>,
}

#[derive(InputObject)]
pub struct CreateCustomer {
    pub id: ID,
    pub name: String,
}