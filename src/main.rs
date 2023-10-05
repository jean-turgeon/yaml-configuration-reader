mod configuration;

use crate::configuration::{ConfigurationClient, YamlConfigurations};


fn main() {
    let config_name:String = String::from("test_configs");
    let config_file:String = String::from("src/configs.yaml");

    // Fetch and read the configuration file
    let client:ConfigurationClient = ConfigurationClient::new(config_name, config_file).unwrap();
    let configurations:YamlConfigurations = client.get_configs();

    println!("Retrieve 'application.consumer.protocol': {}", client.get_config("application.consumer.protocol".to_string()));

    // Print out all key, value pairs
    for (key, value) in configurations.map {
        println!("Key:'{}':  Value:'{}'", key, value);
    }
}
