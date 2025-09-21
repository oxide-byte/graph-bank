use crate::domain::deposit::Deposit;
use async_graphql::ID;

pub(crate) fn get_deposit(id: ID) -> Option<Deposit> {
    println!("Getting deposit {:?}", id);
    if id == "1" {
        Some(Deposit {
            id,
            name: String::from("Name"),
        })
    } else {
        None
    }
}