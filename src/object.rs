use std::collections::HashMap;
use serde_json::{Map, Value};
use std::error;

#[derive(Debug)]
pub struct Object {
    data: Map<String, Value>,
}

impl Object {
    pub fn new() -> Object {
        Object {
            data: Map::new(),
        }
    }

    pub fn insert(&mut self, keys: &Vec<String>, val: &str) -> Result<(), Box<dyn error::Error>> {
        insert_field(&mut self.data, keys, val)?;
    
        Ok(())
    }

    pub fn from(contents: &HashMap<Vec<String>, String>) -> Result<Object, Box<dyn error::Error>> {
        let mut data = Map::new();
    
        for (keys, val) in contents {
            insert_field(&mut data, keys, val)?;
        }

        Ok(Object { data })
    }

    pub fn data(&self) -> &Map<String, Value> {
        &self.data
    }
}

fn insert_field(map: &mut Map<String, Value>, keys: &Vec<String>, val: &str) -> Result<(), Box<dyn error::Error>> {
    if keys.len() > 1 {
        match map.get_mut(keys.get(0).unwrap()) {
            Some(Value::Object(map)) => {
                insert_field(map, &keys[1..].to_vec(), val)?;
            },
            _ => {
                let mut child = Map::new();
                insert_field(&mut child, &keys[1..].to_vec(), val)?;

                let v = Value::Object(child);
                map.insert(keys.get(0).unwrap().to_string(), v);
            }
        }
    } else {
        map.insert(keys.get(0).unwrap().to_string(), Value::String(val.to_string()));
    }

    Ok(())
}
