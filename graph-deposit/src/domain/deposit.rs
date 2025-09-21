use async_graphql::{InputObject, Object, SimpleObject, ID};

pub struct Customer {
    pub id: ID,
}

#[Object(extends)]
impl Customer {
    #[graphql(external)]
    async fn id(&self) -> &ID {
        &self.id
    }

    async fn deposit(&self) -> Deposit {
        println!("Retrieve Sub Deposit");
        Deposit {
            id: self.id.clone(),
            name: String::from("Name_sub"),
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