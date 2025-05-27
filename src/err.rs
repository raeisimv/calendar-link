use std::error::Error;

pub type MyErr = Box<dyn Error>;
pub type MyResult<T = (), E = MyErr> = Result<T, E>;
