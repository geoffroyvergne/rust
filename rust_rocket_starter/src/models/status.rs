#[derive(Debug, Serialize, Deserialize)]
pub struct CustomStatus {
    pub message: String,
    pub code: u16
}
