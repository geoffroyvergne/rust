#[derive(Serialize, Deserialize)]
pub struct Status {
    pub message: String,
    pub code: i32
}

#[derive(Serialize, Deserialize)]
pub struct CustomStatus {
    pub message: String,
    pub code: i32
}