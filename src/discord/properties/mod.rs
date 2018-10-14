use config::{self, Config};

#[derive(Debug)]
pub struct DiscordProperties {
    pub header: Config,
    pub api: Config
}

impl DiscordProperties {
    pub fn get_header_value(&self, header_key: &str) -> String {
        let header_value = self.header.get_str(header_key).unwrap();
        header_value
    }

    pub fn get_api_value(&self, api_key: &str) -> String {
        let api_value = self.api.get_str(api_key).unwrap();
        api_value
    }
}

pub fn load() -> DiscordProperties {
    let discord_header_location = "src/resources/discord_header";
    let discord_api_location = "src/resources/discord_api";
    load_property_files(discord_header_location, discord_api_location)
}

fn load_property_files(header_file: &str, api_file: &str) -> DiscordProperties {
    let header = read_config(header_file);
    let api = read_config(api_file);

    DiscordProperties {
        header,
        api
    }
}

fn read_config(file: &str) -> Config {
    let mut config = config::Config::default();
    config.merge(config::File::with_name(file)).unwrap();
    config
}