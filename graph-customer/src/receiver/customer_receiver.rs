use async_graphql::ID;
use crate::domain::customer::Customer;

pub(crate) fn get_customer(id: ID) -> Option<Customer> {
    if id == "1" {
        Some(Customer {
            id,
            name: String::from("Name"),
        })
    } else {
        None
    }
}