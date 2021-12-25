pub use actix_threadpool::run;

pub type AsyncPoolError<T> = actix_threadpool::BlockingError<T>;
