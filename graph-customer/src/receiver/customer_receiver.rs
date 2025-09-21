use crate::domain::customer::Customer;
use async_graphql::ID;

pub fn get_customer(id: ID) -> Option<Customer> {
    println!("Getting customer {:?}", id);
    if id == "1" {
        Some(Customer {
            id,
            name: String::from("Name")
        })
    } else {
        None
    }
}