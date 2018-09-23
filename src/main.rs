mod setting;

fn main() {

    let settings = setting::read_settings_from_file("credentials");

    println!("{:?}", settings);
}