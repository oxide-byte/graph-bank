use async_graphql::{InputObject, Object, SimpleObject, ID};
use tracing::info;

pub struct Customer {
    pub id: ID,
}

#[Object(extends)]
impl Customer {
    #[graphql(external)]
    async fn id(&self) -> &ID {
        &self.id
    }

    async fn deposit(&self) -> Option<Deposit> {
        info!("Getting deposit for customer: {:?}", self.id);
        if self.id == "CUSTOMER_1" {
            Some(Deposit {
                id: ID::from("DEPOSIT_1"),
                name: String::from("Deposit for Customer 1"),
            })
        } else {
            None
        }
    }
}


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