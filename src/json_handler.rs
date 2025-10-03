//! This module is used for read and writing the json data used for the overlays and the app
use std::{
    fs::{self, File},
    io::{BufReader, Read},
    path::Path,
};

use serde_json::{json, Value};

use crate::{config::Config, constants::{self, get_config_path}};


/// Reads the config json and returns the value of the requested key as `String`
///
/// # Arguments
/// * `key: &str` - The key to be read from the json file
///
/// # Returns
/// * `String` - The data at the desired key
pub fn read_config_string(key: &str) -> String {
    read_json(key, &constants::get_config_path()).to_string()
}

/// Reads the config json and returns the value of the requested key as `Vec<Value>`
///
/// # Arguments
/// * `key: &str` - The key to be read from the json file
///
/// # Returns
/// * `Vec<Value>` - The data at the desired key
pub fn read_config_array(key: &str) -> Vec<Value> {
    let vec = read_json(key, &constants::get_config_path()).as_array().unwrap_or(&Vec::new()).to_vec();
    println!("{}", vec.len());
    vec

    // read_json(key, &constants::get_config_path()).as_array().unwrap_or(&Vec::new()).iter().map(|v| v.as_str().unwrap_or_default().to_string()).collect()
}

/// Reads the json at the supplied path and returns the value of the requested key
///
/// # Arguments
/// * `key: &str` - The key to be read from the json file
/// * `path: String` - The path to the json file
///
/// #Returns
/// * 'Value' - The data at the desired key
pub fn read_json(key: &str, path: &str) -> Value {
    let json_data: Value = open_json(path);

    read_json_from_ref(key, &json_data)
}

pub fn read_json_as_value(path: &str) -> Value {
    open_json(path).clone()
}

/// Reads the supplied `Value` and returns the value of the provided key
///
/// # Arguments
/// * `key: &str` - The key to be read from the `Value` reference
/// * `json: &Value` - The reference to be read 
///
/// #Returns
/// * 'Value' - The data at the desired key
pub fn read_json_from_ref(key: &str, json: &Value) -> Value {
    json.get(key).unwrap().clone()
}

/// Opens the json file with the supplied path
///
/// # Arguments
/// * `path: String` - The path to the JSON file to read
///
/// # Returns
/// * `Value` - Contains the JSON data
///
/// # Examples
/// ```ignore
/// open_json("random_path/config.json");
/// ```
fn open_json(path: &str) -> Value {
    let json_data: Value;

    // Checks to make sure that the JSON file is there, if it isn't it makes it
    if Path::new(&path).exists() {
        let mut reader: BufReader<File> = BufReader::new(File::open(&path).unwrap());

        let mut buffer: Vec<u8> = Vec::new();

        reader
            .read_to_end(&mut buffer)
            .map_err(|e| e.to_string())
            .unwrap();

        // If the file is a "Resource Not Found" file, return a blank vector
        if buffer.len() == 0 {
            return Value::default();
        }

        json_data = {
            let file_content: String = fs::read_to_string(&path).expect("File not found");
            serde_json::from_str::<Value>(&file_content).expect("Error serializing to JSON")
        };
    } else {
        json_data = init_json(path);
    }

    // Returns the json data
    json_data
}

/// This function is called if the JSON being read doesn't exist
///
/// It after making the file it will try to read the file and then return that value
///
/// # Arguments
/// * `path: String` - The path to the JSON file to read
///
/// # Returns
/// * `Value` - Contains the JSON data
pub fn init_json(path: &str) -> Value {
    // Creating the directories
    let _ = std::fs::create_dir_all(Path::new(&path).parent().unwrap());

    // Initializes the json_data variable
    let json_data: Value = get_default_config();

    // Creating the JSON file
    fs::write(
        &path,
        serde_json::to_string_pretty(&json_data).expect(
            "Error 
    serializing to JSON",
        ),
    )
    .expect("Error writing file");

    // Trying to open the JSON again
    open_json(path)
}

/// Writes to the JSON file at the supplied path
///
/// # Arguments
/// * `path: String` - Path to the JSON file
/// * `json_key: String` - Key to write to
/// * `value: String` ` Value to write to the key`
///
/// # Examples
/// ```ignore
/// write_json("random_path/config.json", "profile", "NewProfile");
/// ```
pub fn write_json(path: &str, json_key: &str, value: Value) {
    // Cloning the data because a borrow won't work in this case
    let mut json_data = open_json(path);

    json_data[json_key] = value;

    fs::write(
        path,
        serde_json::to_string_pretty(&json_data).expect("Error serializing to JSON"),
    )
    .expect("Error writing file");
}

/// Write to a file from a `Value`
/// 
/// # Argument
pub fn write_json_from_value(path: &str, value: &Value) {
    fs::write(
        path,
        serde_json::to_string_pretty(&value).expect("Error serializing to json"),
    )
    .expect("Failed to write to file.");
}

/// Recursively reads a JSON value and writes a new value to the specified key path.
/// No IO means it won't write to file system
///
/// # Arguments
/// * `json` - The JSON value to be modified.
/// * `keys` - A dot-separated string path specifying the keys/indexes to traverse. Array indices should be wrapped in square brackets, `arrayKey[0].nestedKey`.
/// * `value` - The new value to write at the final key.
///
/// # Returns
/// * `Value` - The modified JSON with the new value inserted.
///
/// # Example
/// ```ignore
/// use serde_json::{json, Value};
///
/// let json = json!({"key": [{"nestedKey": "oldValue"}]});
///
/// let value = Value::String("newValue".to_string());
///
/// let new_json = write_nested_json_no_io(json, "key[0].nestedKey".to_string(), value);
///
/// assert_eq!(new_json, json!({"key": [{"nestedKey": "newValue"}]}));
/// ```
pub fn write_nested_json_no_io(mut json: Value, keys: String, value: Value) -> Value {
    // Makes the key variable to keep track of characters
    let mut key = String::new();

    // Iterates through every char while keeping track of the index
    for (i, char) in keys.chars().enumerate() {
        match char {
            // If char is a '.', set json[key] equal to the next nested key
            '.' => {
                json[key] = write_nested_json_no_io(
                    json[&key].clone(),
                    keys.clone().split_at(i + 1).1.to_owned(),
                    value,
                );
                break;
            }
            // If char is a '['
            '[' => {
                // Get the char from the string as a usize
                let mut key = String::new();

                for char in keys.get(i..).unwrap().chars() {
                    if char != ']' {
                        key.push(char);
                    } else if char == ']' {
                        break;
                    }
                }

                let i_key = keys.get(i + 1..i + 2).unwrap().parse::<usize>().unwrap();

                // If the key doesn't exist, push the value and set the json equal to the new Vec
                if json.as_array().unwrap().len() == 0 || json.as_array().unwrap().len() - 1 < i_key
                {
                    let mut json_vec = json.as_array().unwrap().to_owned();

                    json_vec.push(value);

                    json = Value::Array(json_vec);
                } else {
                    // If the key exists, set it equal to the next nested value
                    json[i_key] = write_nested_json_no_io(
                        json[i_key].clone(),
                        keys.clone().split_at(i + 3).1.to_owned(),
                        value,
                    );
                }

                // Escape the loop
                break;
            }
            // If char is a ']' do nothing
            ']' => (),
            // If char is anything else, add it to the key
            _ => key.push(char),
        }

        // If i is the last character, or if the next character is ']' and i is the second to last character
        // Write the inputted value to the json
        if i == keys.len() - 1
            || (keys.get(i..i + 1).unwrap() == "]".to_string() && i == keys.len() - 2)
        {
            json[key.clone()] = value.clone();
        }
    }

    // Returns the json object
    json
}

/// Writes to the config json
///
/// # Arguments
/// * `key: &str` - Key to write to
/// * `value: &str` - Value to write to the key
pub fn write_config(key: &str, value: Value) {
    write_json(
        &constants::get_config_path(),
        key,
        value,
    );
}

/// Writes to the config using a complete `Config`
/// 
/// # Arguments
/// * `value: Value` - Configuration to set the config to
pub fn write_config_all(value: &Value) {
   write_json_from_value(&constants::get_config_path(), value);
}

/// Iterate over a json object and return a Vec of key values
///
/// # Arguments
/// * `json_key: &str` - Key to search for
/// * `json: &Value` - Reference to json object to be search
///
/// # Returns
/// 'Vec<String>' Contains all the found values
pub fn iterate_json(json_key: &str, json: &Value) -> Vec<String> {
    let mut entries: Vec<String> = Vec::new();

    if json.is_array() {
        for value in json.as_array().unwrap().to_vec() {
            for v in iterate_json_map(json_key, &value) {
                entries.push(v);
            }
        }
    } else {
        for v in iterate_json_map(json_key, json) {
            entries.push(v);
        }
    }

    entries
}

/// Iterates over a json object
///
/// # Arguments
/// * `json_key`: &str` - Key to search for
/// * `json: &Value` - Reference to json object to be searched
///
/// # Returns
/// `Vec<String>` Contains all the found values
fn iterate_json_map(json_key: &str, json: &Value) -> Vec<String> {
    let mut entries: Vec<String> = Vec::new();

    for value in json.as_object().unwrap() {
        let (key, v) = value;
        if key == json_key {
            entries.push(v.to_string().replace("\"", ""));
        } else if v.is_object() {
            for val in iterate_json(json_key, &v) {
                entries.push(val);
            }
        }
    }

    entries
}

/// Resets the client config
pub fn reset_config() {
    let default_json = get_default_config();
    
    write_json_from_value(&get_config_path(), &default_json);
}

/// Default settings for the config
fn get_default_config() -> Value {
    json!({
        "emailAddress": "me@example.com",
        "emailPassword": "1243124231",
        "emailSMTPHost": "smtp.example.com",
        "emailSMTPPort": 465,
        "recipientAddress": "person@example.com",
        "checkIntervalMinutes": 15,
        "ipAddress": "127.0.0.1"
    })
}


pub trait ToConfig {
    /// Converts a JSON value to a `Config` instance.
    ///
    /// # Arguments
    /// * `value` - The JSON value to convert.
    ///
    /// # Returns
    /// A `Config` instance created from the JSON value.
    fn to_config(&self) -> Config;
}

impl ToConfig for serde_json::Value {
    fn to_config(&self) -> Config {
        // Extract each field from the JSON value, providing defaults if necessary
        let email_address = self.get("emailAddress").and_then(|v| v.as_str()).unwrap_or_default().to_string();
        let email_password = self.get("emailPassword").and_then(|v| v.as_str()).unwrap_or_default().to_string();
        let email_smtp_host = self.get("emailSMTPHost").and_then(|v| v.as_str()).unwrap_or_default().to_string();
        let email_smtp_port = self.get("emailSMTPPort").and_then(|v| v.as_u64()).unwrap_or(587) as u16;
        let recipient_address = self.get("recipientAddress").and_then(|v| v.as_str()).unwrap_or_default().to_string();
        let check_interval_minutes = self.get("checkIntervalMinutes").and_then(|v| v.as_u64()).unwrap_or(5);
        let ip_address = self.get("ipAddress").and_then(|v| v.as_str()).unwrap_or_default().to_string();

        Config::new(
            email_address,
            email_password,
            email_smtp_host,
            email_smtp_port,
            recipient_address,
            check_interval_minutes,
            ip_address,
        )
    }
}
