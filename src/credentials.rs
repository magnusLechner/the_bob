use config;

#[derive(Debug)]
pub struct Credentials {
    pub client_id: String,
    pub client_secret: String,
    pub bot_token: String
}

pub fn read_credentials_from_file(file: &str) -> Credentials {
    let mut credentials = config::Config::default();
    credentials.merge(config::File::with_name(file)).unwrap();

    let client_id = credentials.get_str("client_id").unwrap();
    let client_secret = credentials.get_str("client_secret").unwrap();
    let bot_token = credentials.get_str("bot_token").unwrap();

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
        let credentials = read_credentials_from_file(test_file);

        // Assert
        let client_id = credentials.client_id.as_str();
        let client_secret = credentials.client_secret.as_str();
        let bot_token = credentials.bot_token.as_str();

        assert_eq!(expected_client_id, client_id);
        assert_eq!(expected_client_secret, client_secret);
        assert_eq!(expected_bot_token, bot_token);
    }
}