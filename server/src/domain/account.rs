use super::common::ID;

#[derive(Debug)]
pub struct Account {
    pub id: ID,
    pub username: String,
    pub email: String,
}
