extern crate serde_json;

use std::collections::HashMap;
use std::error::Error;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "Command")]
pub enum Message {
    SetDataRequest {
        #[serde(rename = "Tag")]
        tag: Option<String>,
        #[serde(rename = "Datas")]
        datas: Vec<HashMap<String, Value>>,
    },
    GetDataRequest {
        #[serde(rename = "Tag")]
        tag: Option<String>,
        #[serde(rename = "Datas")]
        datas: Vec<String>,
    },
    GetDataResponse {
        #[serde(rename = "Tag")]
        tag: Option<String>,
        #[serde(rename = "Datas")]
        datas: Vec<HashMap<String, Value>>,
    },
    None,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Number(f64),
    String(String),
}

pub fn parse(json_str: &str) -> Result<Message, Box<dyn Error>> {
    let m: Message = serde_json::from_str(json_str)?;
    Ok(m)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_command_set_data_request() {
        let mut obj1 = HashMap::new();
        obj1.insert("SP1".to_owned(), Value::Number(34.5));
        let mut obj2 = HashMap::new();
        obj2.insert("SSA".to_owned(), Value::String("abc".to_owned()));

        let expected = Message::SetDataRequest{tag: Some("123".to_string()),
                                               datas: vec![obj1, obj2]};

        let json = r#"{"Command":"SetDataRequest", "Tag":"123",
                       "Datas": [{"SP1": 34.5}, {"SSA": "abc"}]
                      }"#;

        let message = parse(json).unwrap();

        assert_eq!(expected, message);
    }

    #[test]
    fn parse_command_get_data_request() {
        let expected = Message::GetDataRequest{tag: None,
                datas: vec!["SP1".to_string(), "SSA".to_string()]};

        let json = r#"{"Command":"GetDataRequest",
                       "Datas": ["SP1", "SSA"]
                      }"#;

        let message = parse(json).unwrap();

        assert_eq!(expected, message);
    }

    #[test]
    fn parse_command_get_data_response() {
        let mut obj1 = HashMap::new();
        obj1.insert("SP1".to_owned(), Value::Number(34.5));
        let mut obj2 = HashMap::new();
        obj2.insert("SSA".to_owned(), Value::String("abc".to_owned()));

        let expected = Message::GetDataResponse{tag: Some("123".to_string()),
                                                datas: vec![obj1, obj2]};

        let json = r#"{"Command":"GetDataResponse", "Tag":"123",
                       "Datas": [{"SP1": 34.5}, {"SSA": "abc"}]
                      }"#;

        let message = parse(json).unwrap();

        assert_eq!(expected, message);
    }
}
