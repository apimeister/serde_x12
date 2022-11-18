mod de;
mod error;
mod ser;

use std::fmt::Display;

pub use de::*;
pub use error::*;
pub use ser::*;

#[derive(Debug, Clone)]
pub enum PathOperation {
    Push(PathItem),
    Pop,
}

#[derive(Debug, Clone, Default)]
pub struct Path {
    pub elem: Vec<PathItem>,
    pub next_op: Option<PathOperation>,
}

impl Path {
    pub fn push(&self, name: String, vec_position: Option<u64>, leaf: bool) -> Path {
        let mut x = self.clone();
        x.elem.push(PathItem {
            name,
            vec_position,
            leaf,
        });
        x
    }
    pub fn push_item(&self, item: &PathItem) -> Path {
        let mut x = self.clone();
        x.elem.push(item.clone());
        x
    }
    pub fn pop(&self) -> Path {
        let mut x = self.clone();
        x.elem.pop();
        x
    }
    pub fn is_leaf(&self) -> bool {
        if self.elem.is_empty() {
            true
        } else {
            self.elem.last().unwrap().leaf
        }
    }
    pub fn next_op(&self, operation: PathOperation) -> Path {
        let mut x = self.clone();
        x.next_op = Some(operation);
        x
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::from("");
        for elem in &self.elem {
            if elem.vec_position.is_none() {
                result = format!("{result}.{}", elem.name);
            } else {
                result = format!("{result}.{}.{}", elem.name, elem.vec_position.unwrap());
            }
        }
        write!(f, "{}", result)
    }
}

#[derive(Debug, Clone)]
pub struct PathItem {
    /// defaults to empty string
    pub name: String,
    pub vec_position: Option<u64>,
    /// defaults to true
    pub leaf: bool,
}

impl Default for PathItem {
    fn default() -> Self {
        Self {
            name: Default::default(),
            vec_position: None,
            leaf: true,
        }
    }
}

pub trait Reflect {
    fn get_type_name() -> String;
    fn get_path(current_path: &Path, next_segment: &str, last_path: &Path) -> Path;
}
