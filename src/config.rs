use toml::{Value, Parser};

trait FromTOMLValue: Sized {
     fn from_toml_value(&Value) -> Result<Self, String>;
}

fn parse_table_field<T: FromTOMLValue>(value: &Value, key: &str) -> Result<T, String> {
  match *value {
    Value::Table(ref table) => match table.get(key) {
      Some(ref value) => T::from_toml_value(value),
      None => Err(format!("expect a field named '{}'", key)),
    },
    _ => Err("expect a table".to_string()),
  }
}

impl FromTOMLValue for String {
  fn from_toml_value(value: &Value) -> Result<Self, String> {
    match *value {
      Value::String(ref string) => {
        Ok(string.clone())
      },
      _ => Err("expect a string".to_string()),
    }
  }
}

impl FromTOMLValue for Vec<String> {
  fn from_toml_value(value: &Value) -> Result<Self, String> {
    match *value {
      Value::Array(ref array) => {
        let mut values: Vec<String> = vec![];        
        for v in array {
          if let Value::String(ref string) = *v {
            values.push(string.clone());
          } else {
            return Err("expect an all string array".to_string());
          }
        }
        Ok(values)
      },
      _ => Err("expect an array".to_string()),
    }
  }
}

#[derive(Debug, PartialEq)]
pub struct StorageConfig {
  pub path: String
}

impl FromTOMLValue for StorageConfig {
  fn from_toml_value(value: &Value) -> Result<Self, String> {
    Ok(StorageConfig {
      path: parse_table_field(value, "path").map_err(|msg| format!("[storage].path: {}", msg))?,
    })
  }
}

#[derive(Debug, PartialEq)]
pub struct ExtractConfig {
  pub globs: Vec<String>,
  pub out_dir: String,
}

impl FromTOMLValue for ExtractConfig {
  fn from_toml_value(value: &Value) -> Result<Self, String> {
    Ok(ExtractConfig {
      globs: parse_table_field(value, "globs").map_err(|msg| format!("[extract].globs: {}", msg))?,
      out_dir: parse_table_field(value, "out_dir").map_err(|msg| format!("[extract].out_dir: {}", msg))?,
    })
  }
}

#[derive(Debug, PartialEq)]
pub struct Config {
  pub storage: StorageConfig,
  pub extract: ExtractConfig,
}

impl Config {
  pub fn parse(src: &str) -> Result<Self, String> {
    let mut parser = Parser::new(src);
    let table = parser.parse().ok_or_else(|| {
      format!("{:?}", parser.errors.iter().fold(String::new(), |mut msg, e| {
        msg.push_str(&e.desc);
        msg
      }))
    })?;
    let value = &Value::Table(table);
    Ok(Config {
      storage: parse_table_field(value, "storage").map_err(|msg| format!("[storage]: {}", msg))?,
      extract: parse_table_field(value, "extract").map_err(|msg| format!("[extract]: {}", msg))?,
    })
  }
}

#[test]
fn test_config() {
  let text = r#"[storage]
path = "C:\\Program Files (x86)\\StarCraft II\\SC2Data"

[extract]
globs = [
  "*"
]
out_dir = "./files"
"#;
  let config = Config::parse(text).unwrap();
  assert_eq!(config, Config {
    storage: StorageConfig {
      path: "C:\\Program Files (x86)\\StarCraft II\\SC2Data".to_string(),
    },
    extract: ExtractConfig {
      globs: vec!["*".to_string()],
      out_dir: "./files".to_string(),
    }
  });
}
