use crate::error::Error;
use serde::{ser, Serialize};

pub struct Serializer {
    output: Vec<Vec<Option<String>>>,
}

pub fn to_string<T>(value: &T) -> Result<String, Error>
where
    T: Serialize,
{
    let mut serializer = Serializer { output: vec![] };
    value.serialize(&mut serializer)?;
    let mut final_string = "".to_string();
    for line in serializer.output {
        if line.len() == 1 {
            continue;
        }
        let mut x = line.clone();
        x.reverse();
        let mut length_of_line = x.len();
        for item in x {
            if item.is_none() {
                length_of_line -= 1;
            } else {
                break;
            }
        }
        let mut line = line.clone();
        line.truncate(length_of_line);
        //concatenate single line
        for item in line {
            match item {
                Some(s) => final_string.push_str(&s),
                None => final_string.push_str(""),
            }
            final_string.push('*');
        }
        final_string.pop();
        final_string.push_str("~\n");
    }
    Ok(final_string)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        #[cfg(feature = "debug")]
        println!("serialize_str: {}", v);
        match self.output.last_mut() {
            Some(el) => {
                el.push(Some(v.to_string()));
            }
            None => {
                self.output.push(vec![Some(v.to_string())]);
            }
        }
        Ok(())
    }
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        #[cfg(feature = "debug")]
        println!("serialize_none");
        match self.output.last_mut() {
            Some(el) => el.push(None),
            None => {
                self.output.push(vec![None]);
            }
        }
        Ok(())
    }

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        #[cfg(feature = "debug")]
        println!("serialize_char: {}", v);
        match self.output.last_mut() {
            Some(el) => el.push(Some(v.to_string())),
            None => {
                self.output.push(vec![Some(v.to_string())]);
            }
        }
        Ok(())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        #[cfg(feature = "debug")]
        println!("serialize_some");
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        #[cfg(feature = "debug")]
        println!("serialize_unit");
        //empty value
        match self.output.last_mut() {
            Some(el) => el.push(Some("".to_string())),
            None => {
                self.output.push(vec![Some("".to_string())]);
            }
        }
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        #[cfg(feature = "debug")]
        println!("serialize_unit_struct: {}", _name);
        match self.output.last_mut() {
            Some(el) => el.push(Some("".to_string())),
            None => {
                self.output.push(vec![Some("".to_string())]);
            }
        }
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        #[cfg(feature = "debug")]
        println!(
            "serialize_unit_variant: {} {} {}",
            _name, _variant_index, variant
        );
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        #[cfg(feature = "debug")]
        println!(
            "serialize_newtype_variant: {} {} {}",
            _name, _variant_index, _variant
        );
        value.serialize(&mut *self)?;
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        #[cfg(feature = "debug")]
        println!("serialize_seq: {:?}", _len);
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        #[cfg(feature = "debug")]
        println!("serialize_tuple: {}", _len);
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        #[cfg(feature = "debug")]
        println!("serialize_map: {:?}", _len);
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        #[cfg(feature = "debug")]
        println!("serialize_struct: {} {}", name, _len);
        self.output.push(vec![Some(name.to_string())]);
        Ok(self)
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        #[cfg(feature = "debug")]
        println!("SerializeSeq::serialize_element");
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        #[cfg(feature = "debug")]
        println!("SerializeSeq::end");
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        #[cfg(feature = "debug")]
        println!("SerializeTuple::serialize_element");
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        #[cfg(feature = "debug")]
        println!("SerializeTuple::end");
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        #[cfg(feature = "debug")]
        println!("SerializeMap::serialize_key");
        key.serialize(&mut **self)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        #[cfg(feature = "debug")]
        println!("SerializeMap::serialize_value");
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        #[cfg(feature = "debug")]
        println!("SerializeMap::end");
        Ok(())
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        #[cfg(feature = "debug")]
        println!("serialize_field: {}", _key);
        let _ = value.serialize(&mut **self);
        Ok(())
    }

    fn skip_field(&mut self, _key: &'static str) -> Result<(), Self::Error> {
        if let Some(el) = self.output.last_mut() {
            el.push(None);
        };
        #[cfg(feature = "debug")]
        println!("skip_field: {}", _key);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}
