use std::collections::HashMap;
use std::fs::File;
use std::convert::TryFrom;
use std::io::Read;



pub struct YamlConfigurations {
    pub name:String,
    pub map:HashMap<String, String>, // Key represent dot notation of the path to the value
}


// Read a YAML configuration file and provide a YamlConfigurations
// with the map of the value and keys as dot-notated keys.
pub struct ConfigurationClient {
    pub configurations:YamlConfigurations,
}

impl ConfigurationClient {

    pub fn new(name:String, filepath:String) -> Result<Self, Box<dyn std::error::Error>> {

        // Read YAML file into string variable
        let mut file: File = File::open(filepath)?;
        let mut content: String = String::new();
        file.read_to_string(&mut content)?;

        let configs:HashMap<String, String> = Self::read_yaml(content);
        let configurations:YamlConfigurations = YamlConfigurations {name:name, map:configs};

        return Ok(Self { configurations });
    }

    pub fn get_config(&self, name:String) -> String {
        return self.configurations.map.get(&name).unwrap().to_string();
    }

    pub fn get_configs(&self) -> YamlConfigurations {
        return YamlConfigurations {
            name:self.configurations.name.clone(),
            map:self.configurations.map.clone()
        };
    }

    fn read_yaml(content:String) -> HashMap<String, String> {

        // Initialize configuration object
        let mut configs:HashMap<String, String> = HashMap::new();

        const CAPACITY:usize = 12;

        let mut key_set: [&str; CAPACITY] = [""; CAPACITY];
        let mut position:usize =  0;
        let mut buffering:bool = false;
        let mut line_buffer:Vec<String> = Vec::new();

        for data in content.lines() {
            // Read line by line
            // data = &str

            let trimmed:&str = data.trim_start();

            if trimmed.len() > 0 {
                // Process non empty lines only
                if trimmed.starts_with("-") {
                    // Array of values
                    buffering = true;

                    let value:&str = &trimmed[1..].trim(); // Remove - prefix
                    line_buffer.push(value.to_string());
                }
                else {
                    if buffering {
                        // First finish with buffering before proceeding.
                        let value:String = line_buffer.iter().map(|x| x.to_string() + ",").collect::<String>();
                        let dot_notation:String = key_set[0..position+1].join(".");

                        configs.insert(dot_notation.clone(), value);

                        line_buffer.clear();
                        buffering = false;
                    }

                    // Simple node
                    let data_length:i32 = i32::try_from(data.len()).unwrap();
                    let trimmed_length:i32 = i32::try_from(trimmed.len()).unwrap();
                    let current_level:i32 = (data_length - trimmed_length)/2;

                    position =  usize::try_from(current_level).unwrap();

                    let (key, token_right) = trimmed.split_once(":").unwrap();
                    let value:&str = token_right.trim();

                    // Set/Update key
                    key_set[position] = key;

                    // Clean other cells to the end of the array
                    let mut i:usize = position + 1;
                    while i < CAPACITY {
                        key_set[i] = "";
                        i += 1;
                    }

                    let dot_notation:String = key_set[0..position+1].join(".");

                    configs.insert(dot_notation.clone(), value.to_string());
                }
            }
        }
        return configs;
    }
}
