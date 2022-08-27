use crate::{Error, Path, PathOperation, Reflect};
use json_dotpath::DotPaths;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Map;
use serde_json::Value;
use std::collections::HashMap;

pub fn from_str<T>(s: &str) -> Result<T, Error>
where
    T: DeserializeOwned + Reflect,
{
    let mut json_value = Value::Object(Map::new());

    //split on segment
    let lines = s.split('~').collect::<Vec<&str>>();
    let mut last_path = Path::default();
    for line in lines {
        //parse single line
        let parts = line.split('*').collect::<Vec<&str>>();
        let segment_name = parts[0];
        // find path for segment
        println!("segment: {segment_name}");
        let name = T::get_type_name();
        last_path = T::get_path(&last_path, segment_name);
        let leaf = last_path.is_leaf();
        println!("type_name: {name} {} {leaf}", last_path);
        //process value
        for (idx, part) in parts.iter().enumerate() {
            //skip first entry, because it is the segment name
            if idx == 0 {
                continue;
            }
            let path_name = format!("{last_path}._{:02}",idx);
            println!("{path_name}: {part}");
            _ = json_value.dot_set(&path_name[1..], Value::String(part.to_string()));
        }
        // _ = json_value.dot_set(&format!("{}",name), Value::String(val.to_string()));
        // println!("{segment_name} {name} => {val}");
        if leaf {
            last_path.elem.pop();
        }
        match &last_path.next_op {
            Some(PathOperation::Pop) => {
                last_path = last_path.pop();
                last_path.next_op = None;
            }
            Some(PathOperation::Push(item)) => {
                last_path = last_path.push_item(item);
                last_path.next_op = None;
            }
            None => {}
        };
    }
    let str = serde_json::to_string_pretty(&json_value).unwrap();
    println!("final json: {} ", str);
    let result: Result<T, _> = serde_json::from_str(str.as_str());
    match result {
        Ok(val) => Ok(val),
        Err(err) => Err(Error::Message(format!("failed: {:?}", err))),
    }
}
