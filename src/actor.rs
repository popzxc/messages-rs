pub trait Actor {
    fn created(&self) {}
    fn started(&self) {}
    fn stopped(&self) {}
}
