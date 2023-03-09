use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Matcher {
    pub prefix: String,
    pub prefix_rewrite: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Route {
    pub matcher: Matcher,
    pub route_cluster: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, strum_macros::Display)]
pub enum ServiceType {
    #[default]
    HTTP,
    HTTPS,
    TCP,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceConfig {
    pub server_type: ServiceType,
    pub cert_str: Option<String>,
    pub key_str: Option<String>,
    pub routes: Vec<Route>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiService {
    pub listen_port: i32,
    pub service_config: ServiceConfig,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct StaticConifg {
    pub access_log: Option<String>,
    pub database_url: Option<String>,
    pub api_port: String,
    pub config_file_path: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub static_config: StaticConifg,
    pub api_service_config: Vec<ApiService>,
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_output_serde() {
        let route = Route {
            route_cluster: String::from("/"),
            matcher: Matcher {
                prefix: String::from("ss"),
                prefix_rewrite: String::from("ssss"),
            },
        };
        let api_service = ApiService {
            listen_port: 4486,
            service_config: ServiceConfig {
                routes: vec![route],
                server_type: Default::default(),
                cert_str: Default::default(),
                key_str: Default::default(),
            },
        };
        let t = vec![api_service];
        let yaml = serde_yaml::to_string(&t).unwrap();
        println!("{}", yaml);
    }
}