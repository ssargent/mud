use std::error;

pub trait BusinessLogic {
    type Item;

    fn get_by_code(&self, code: &str) -> Result<Self::Item, Box<dyn error::Error>>;
    fn create_or_update(&self, item: &Self::Item) -> Result<Self::Item, Box<dyn error::Error>>;
    fn delete(&self, code: &str) -> Result<(), Box<dyn error::Error>>;
}
