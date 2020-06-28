#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Users {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}