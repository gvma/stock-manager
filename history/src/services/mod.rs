mod database;
//mod event_queue;

pub use database::*;
//pub use event_queue::*;

pub trait Connect<T> {
    async fn connect() -> T;
}