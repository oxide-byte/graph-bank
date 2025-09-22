use crate::domain::customer::Customer;
use async_graphql::ID;
use tracing::info;

pub fn get_customer(id: ID) -> Option<Customer> {
    info!("Getting customer {:?}", id);
    if id == "CUSTOMER_1" {
        Some(Customer {
            id,
            name: String::from("Customer Name 1 WITH Depot")
        })
    } else if id == "CUSTOMER_2" {
        Some(Customer {
            id,
            name: String::from("Customer Name 2 NO Depot")
        })
    } else {
        None
    }
}