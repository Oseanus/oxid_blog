#[derive(Clone, Debug)]
pub struct Config {
    host_name: String,
    port: u32,
    user_name: String,
    password: String,
}

impl Config {
    pub fn new(host_name: String, port: u32, user_name: String, password: String) -> Config {
        Config {
            host_name,
            port,
            user_name,
            password,
        }
    }

    pub fn get_auth_string(&self) -> String {
        let mongodb = String::from("mongodb://");
        String::from(mongodb + &self.user_name + ":" + &self.password + "@" + &self.host_name + ":" + &self.port.to_string())
    }
}