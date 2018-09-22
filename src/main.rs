extern crate config;

fn main() {

    let bot_token = read_token_from_file("credentials");

    println!("{:?}", bot_token);

}

fn read_token_from_file(file: &str) -> String {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name(file)).unwrap();
    let mut bot_token = String::new();
    bot_token = settings.get_str("bot_token").unwrap();
    bot_token
}

#[cfg(test)]
mod tests {

    #[test]
    fn read_token() {
        use super::*;

        let expected_token = "test-bot-token";

        let test_file = "src/test/resources/test-credentials";
        let read_token = read_token_from_file(test_file);

        assert_eq!(expected_token, &*read_token);
    }
}