use std::result::Result;

pub fn id_result<T, E>(t: T) -> Result<T, E> {
    Ok(t)
}
