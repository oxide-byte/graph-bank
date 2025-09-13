use async_graphql::{InputObject, SimpleObject, ID};

#[derive(SimpleObject)]
#[graphql(shareable)]
pub struct Deposit {
    pub id: ID,
    pub name: String,
}

#[derive(InputObject)]
pub struct CreateDeposit {
    pub id: ID,
    pub name: String,
}