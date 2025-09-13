use crate::domain::customer::Customer;
use async_graphql::ID;

pub(crate) fn get_customer(id: ID) -> Option<Customer> {
    if id == "1" {
        Some(Customer {
            id,
            name: String::from("Name"),
            deposit: None,
        })
    } else {
        None
    }
}