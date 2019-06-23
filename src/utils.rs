use toml::value::Table;

pub fn get_string_from_conf(conf: &Table, conf_name: &str) -> String {
    conf.get(conf_name).unwrap().as_str().unwrap().to_string()
}
