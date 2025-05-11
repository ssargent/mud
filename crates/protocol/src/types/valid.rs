pub trait Valid {
    fn validate(&self) -> Result<(), Vec<String>>;
    fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }
}
