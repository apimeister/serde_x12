use std::collections::HashMap;
use rebound::{Field, FieldKind, Type};
use rebound::ty::CommonTypeInfo;
use serde::de::DeserializeOwned;
use serde_json::Map;
use crate::Error;
use serde::{Deserialize, Serialize};
use rebound::Reflected;
use json_dotpath::DotPaths;
use serde_json::Value;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeInfo {
    #[serde(skip_serializing_if="String::is_empty")]
    name: String,
    #[serde(skip_serializing_if="String::is_empty")]
    _type: String,
    is_vec: bool,
    is_option: bool,
    #[serde(skip_serializing_if="HashMap::is_empty")]
    fields: HashMap<String,TypeInfo>,
}



fn find_segment_path(search_name: &str, ti: &TypeInfo, last_path: PathBuf) -> PathBuf {
    let mut result = last_path.clone();
    if ti.name != search_name {
        let mut cur = ti;
        for field in cur.fields.values() {

        }
    }else{
        result.push(ti.name.clone());
    }
    result
}

pub fn from_str<T>(s: &str) -> Result<T, Error>
where
    T: DeserializeOwned + Reflected,
{
    let mut json_value = Value::Object(Map::new());
    let ty = Type::from::<T>();
    let type_tree = describe_type(&ty.name());
    println!("type: {}",serde_json::to_string(&type_tree).unwrap());

    let mut object_path = PathBuf::new();
    //split on segment
    let lines = s.split('~').collect::<Vec<&str>>();
    for line in lines {
        //parse single line
        let parts = line.split('*').collect::<Vec<&str>>();
        let segment_name = parts[0];
        // get segment from type tree
        let fields: Vec<String> = vec![];
        // println!("reading segment: {}",segment_name);
        //get path of segment
        // let type_info = ti.clone().fields.get(segment_name).unwrap();
        // println!("reading segment: {} {:?}",segment_name,type_info);
        // _ = json_value.dot_set(segment_name, Value::Object(Map::new()));
        let mut idx = 1;
        loop {
            match parts.get(idx) {
                Some(val) => {
                    match fields.get(idx-1) {
                        Some(name) => {
                            _ = json_value.dot_set(&format!("{}",name), Value::String(val.to_string()));  
                            println!("{segment_name} {name} => {val}");      
                        },
                        None => {
                            println!("field missing on idx {} val: {} => fields: {:?}",idx-1,val.to_string(),fields);
                        }
                    }
                }
                None => break,
            }
            idx += 1;
        }
    }
    _ = json_value.dot_remove("\n");
    let str = serde_json::to_string(&json_value).unwrap();
    println!("final json{} ",str);
    let result: Result<T,_> = serde_json::from_str(str.as_str());
    match result {
        Ok(val) => Ok(val),
        Err(err) => Err(Error::Message(format!("failed: {:?}",err))),
    }
}

pub fn describe_type(type_name: &str) -> TypeInfo {
    let mut ti = TypeInfo { name: "".to_string(), _type: "".to_string(), is_vec: false, is_option:false,fields: HashMap::new() };
    unsafe{
        let ty = Type::from_name(type_name).unwrap();
        ti._type = ty.name().to_string();
        if let Type::Struct(info) = ty {
            info.fields().iter().for_each(|field| {
                let result = parse_type(1, field);
                ti.fields.insert(result.name.clone(), result);
            });
        } else {
            assert!(false, "Reflected struct not a Type::Struct")
        }
    }
    ti
}

fn parse_type(level: usize, field: &Field) -> TypeInfo {
    let ty = field.ty();
    let field_name = match field.kind(){
        FieldKind::Named { name } => name.to_string(),
        FieldKind::Tuple { idx } => format!("{}", idx),
        _ => unimplemented!(),
    };
    let type_str = ty.name();
    // println!("{} field_name: {:?} {}", level, field_name, type_str);
    if type_str == "alloc::string::String" {
        return TypeInfo { name: field_name, _type: type_str, is_vec: false, is_option:false, fields: HashMap::new() };
    }
    if type_str == "core::option::Option<alloc::string::String>" {
        return TypeInfo { name: field_name, _type: type_str, is_vec: false, is_option:true, fields: HashMap::new() };
    }
    if type_str.starts_with("core::option::Option") {
        //is vector
        let inner_type = type_str[21 .. type_str.find('>').unwrap()].to_string();
        println!("option of {}", inner_type);
        let mut ti = TypeInfo{ name: field_name, _type: inner_type.clone(), is_vec: false, is_option:true,fields: HashMap::new() };
        let t;
        unsafe{
            t = Type::from_name(&inner_type).unwrap();
        }
        match t {
            Type::Struct(info) => {
                let l2 = level + 1;
                info.fields().iter().for_each(|field| {
                    let result = parse_type(l2, field);
                    ti.fields.insert(result.name.clone(), result);
                });
            },
            Type::Array(_info) => {
                println!("found array")
            },
            _ => {
                println!("found unknown");
            }
        }
        return ti;
    }
    if type_str.starts_with("alloc::vec::Vec") {
        //is vector
        let inner_type = type_str[16 .. type_str.find(',').unwrap()].to_string();
        println!("vector of {}", inner_type);
        let mut ti = TypeInfo{ name: field_name, _type: inner_type.clone(), is_vec: true, is_option:false, fields: HashMap::new() };
        let t;
        unsafe{
            t = Type::from_name(&inner_type).unwrap();
        }
        match t {
            Type::Struct(info) => {
                let l2 = level + 1;
                info.fields().iter().for_each(|field| {
                    let result = parse_type(l2, field);
                    ti.fields.insert(result.name.clone(), result);
                });
            },
            Type::Array(_info) => {
                println!("found array")
            },
            _ => {
                println!("found unknown");
            }
        }
        return ti;
    }
    let mut ti = TypeInfo { name: field_name, _type: type_str, is_vec: false, is_option:false, fields: HashMap::new() };
    match ty {
        Type::Struct(info) => {
            println!("struct of {}", ty.name());
            let l2 = level + 1;
            info.fields().iter().for_each(|field| {
                let result = parse_type(l2, field);
                ti.fields.insert(result.name.clone(), result);
            });
        },
        _ => {
            println!("found unknown");
        }
    }
    return ti;
}