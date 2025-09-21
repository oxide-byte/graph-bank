use async_graphql::{InputObject, SimpleObject, ID};

#[derive(SimpleObject)]
pub struct Customer {
    pub id: ID,
    pub name: String,
}

#[derive(InputObject)]
pub struct CreateCustomer {
    pub id: ID,
    pub name: String,
}