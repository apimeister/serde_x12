use crate::Error;

pub fn from_str<'a, T>(s: &'a str) -> Result<T, Error>
where
    T: Default + nom::Parser<&'a str, T, nom::error::Error<&'a str>>,
{
    let mut x = T::default();
    let result = x.parse(s);
    match result {
        Ok(val) => Ok(val.1),
        Err(err) => Err(Error::Message(format!("failed: {:?}", err))),
    }
}
