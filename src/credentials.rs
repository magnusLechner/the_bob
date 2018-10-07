use config;

#[derive(Debug)]
pub struct Credentials {
    pub client_id: String,
    pub client_secret: String,
    pub bot_token: String
}

pub fn read_credentials_from_file(file: &str) -> Credentials {
    let mut settings_reader = config::Config::default();
    settings_reader.merge(config::File::with_name(file)).unwrap();

    let client_id = settings_reader.get_str("client_id").unwrap();
    let client_secret = settings_reader.get_str("client_secret").unwrap();
    let bot_token = settings_reader.get_str("bot_token").unwrap();

    Credentials {
        client_id,
        client_secret,
        bot_token
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn read_credentials_from_file() {
        use super::*;

        // Arrange
        let expected_client_id = "test-client-id";
        let expected_client_secret = "test-client-secret";
        let expected_bot_token = "test-bot-token";

        let test_file = "src/resources/test/test-credentials";

        // Act
        let settings = read_credentials_from_file(test_file);

        // Assert
        let client_id = settings.client_id.as_str();
        let client_secret = settings.client_secret.as_str();
        let bot_token = settings.bot_token.as_str();

        assert_eq!(expected_client_id, client_id);
        assert_eq!(expected_client_secret, client_secret);
        assert_eq!(expected_bot_token, bot_token);
    }
}