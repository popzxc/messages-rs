use crate::Actor;

pub trait Service: Actor {
    const NAME: &'static str;
}
