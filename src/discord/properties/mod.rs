use config::{self, Config};

#[derive(Debug)]
pub struct DiscordProperties {
    header: Config,
    api: Config
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

pub fn load_discord_properties(header_file: &str, api_file: &str) -> DiscordProperties {
    let header_config = read_config(header_file);
    let api_config = read_config(api_file);

    DiscordProperties {
        header: header_config,
        api: api_config
    }
}

fn read_config(file: &str) -> Config {
    let mut config = config::Config::default();
    config.merge(config::File::with_name(file)).unwrap();

    config
}