pub trait ArangoDocument {
    fn get_insert(&self, db: &str) -> String;
}
