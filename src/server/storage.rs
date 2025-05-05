pub trait Storage {
    fn store_data(&mut self, data: &str);
}