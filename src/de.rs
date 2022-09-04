use crate::{Error, Path, PathOperation, Reflect};
use json_dotpath::DotPaths;
use serde::de::DeserializeOwned;
use serde_json::Map;
use serde_json::Value;

pub fn from_str<T>(s: &str) -> Result<T, Error>
where
    T: DeserializeOwned + Reflect,
{
    let mut json_value = Value::Object(Map::new());

    //split on segment
    let lines = s.split('~').collect::<Vec<&str>>();
    let mut current_path = Path::default();
    let mut last_path = current_path.clone();
    for line in lines {
        //parse single line
        let parts = line.split('*').collect::<Vec<&str>>();
        let segment_name = parts[0].trim();
        //skip emtpy lines
        if segment_name.len() == 0 {
            continue;
        }
        // find path for segment
        #[cfg(feature = "debug")]
        println!("segment: {segment_name}");
        #[cfg(feature = "debug")]
        let name = T::get_type_name();
        current_path = T::get_path(&current_path, segment_name, &last_path);
        let leaf = current_path.is_leaf();
        #[cfg(feature = "debug")]
        println!("type_name: {name} {} {leaf}", current_path);
        //process value
        for (idx, part) in parts.iter().enumerate() {
            //skip first entry, because it is the segment name
            if idx == 0 {
                continue;
            }
            let path_name = format!("{current_path}._{:02}",idx);
            // only set value if it is not empty
            if part.len() > 0 {
                _ = json_value.dot_set(&path_name[1..], Value::String(part.to_string()));
            }
        }
        last_path = current_path.clone();
        if leaf {
            current_path.elem.pop();
        }
        match &current_path.next_op {
            Some(PathOperation::Pop) => {
                current_path = current_path.pop();
                current_path.next_op = None;
            }
            Some(PathOperation::Push(item)) => {
                current_path = current_path.push_item(item);
                current_path.next_op = None;
            }
            None => {}
        };
    }
    let str = serde_json::to_string_pretty(&json_value).unwrap();
    #[cfg(feature = "debug")]
    println!("final json: {} ", str);
    let result: Result<T, _> = serde_json::from_str(str.as_str());
    match result {
        Ok(val) => Ok(val),
        Err(err) => Err(Error::Message(format!("failed: {:?}", err))),
    }
}
