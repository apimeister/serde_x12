use crate::Error;

pub fn from_str<'a, T>(s: &'a str) -> Result<T, Error>
where
    T: Default + x12_types::util::Parser<&'a str, T, nom::error::Error<&'a str>>,
{
    let result = T::parse(s);
    match result {
        Ok(val) => Ok(val.1),
        Err(err) => Err(Error::Message(format!("failed: {:?}", err))),
    }
}
