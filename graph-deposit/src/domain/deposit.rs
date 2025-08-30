use async_graphql::{InputObject, SimpleObject, ID};

#[derive(SimpleObject)]
pub struct Deposit {
    pub id: ID,
    pub name: String,
}

#[derive(InputObject)]
pub struct CreateDeposit {
    pub id: ID,
    pub name: String,
}