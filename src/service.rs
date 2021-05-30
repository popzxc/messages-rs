use crate::actor::Actor;

pub trait Service: Actor {
    const NAME: &'static str;
}
