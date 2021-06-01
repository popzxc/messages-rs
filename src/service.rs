use crate::prelude::{Actor, Address};

pub trait Service: Actor + Default {
    const NAME: &'static str;

    fn start_service() -> Address<Self> {
        Self::default().spawn()
    }
}
