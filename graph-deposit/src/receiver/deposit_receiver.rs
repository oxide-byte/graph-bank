use crate::domain::deposit::Deposit;
use async_graphql::ID;
use tracing::info;

pub fn get_deposit(id: ID) -> Option<Deposit> {
    info!("Getting deposit {:?}", id);
    if id == "DEPOSIT_1" {
        Some(Deposit {
            id,
            name: String::from("Deposit for Customer 1"),
        })
    } else {
        None
    }
}