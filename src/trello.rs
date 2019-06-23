use crate::action::Action;
use crate::config::Config;
use crate::error::ClipperError;
use crate::utils::get_string_from_conf;
use reqwest::{Client, Error, Response, Url};
use std::process;

const API_KEY_CONF: &str = "trello_api_key";
const API_TOKEN_CONF: &str = "trello_api_token";
const LIST_ID_CONF: &str = "trello_list_id";

fn post_resource(url: &str, params: &[(&str, &str)]) -> Result<Response, Error> {
    let url = Url::parse_with_params(url, params).unwrap();
    let client = Client::new();
    client.post(url).send()?.error_for_status()
}

#[derive(PartialEq, Debug)]
pub struct TrelloConfig {
    api_key: String,
    api_token: String,
    list_id: String,
}

impl TrelloConfig {
    pub fn try_from_conf(conf: &Config) -> Result<Self, ClipperError> {
        let conf = conf.get_config();
        let required: Vec<&str> = vec![API_KEY_CONF, API_TOKEN_CONF, LIST_ID_CONF];

        let missing: Vec<String> = required
            .into_iter()
            .filter(|x| !conf.contains_key(*x))
            .map(String::from)
            .collect();
        if !missing.is_empty() {
            return Err(ClipperError::MissingConfigKeys(missing));
        }
        Ok(TrelloConfig {
            api_key: get_string_from_conf(conf, API_KEY_CONF),
            api_token: get_string_from_conf(conf, API_TOKEN_CONF),
            list_id: get_string_from_conf(conf, LIST_ID_CONF),
        })
    }
}

pub struct Trello {
    conf: TrelloConfig,
}

impl Trello {
    pub fn new(conf: Config) -> Self {
        match TrelloConfig::try_from_conf(&conf) {
            Ok(conf) => Trello { conf },
            Err(e) => {
                eprintln!("{}", e.to_string());
                process::exit(0);
            }
        }
    }

    pub fn create_card(&self, url: impl AsRef<str>) {
        post_resource(
            "https://api.trello.com/1/cards",
            &[
                ("key", &self.conf.api_key),
                ("token", &self.conf.api_token),
                ("idList", &self.conf.list_id),
                ("keepFromSource", "all"),
                ("urlSource", url.as_ref()),
            ],
        )
        .unwrap();
    }
}

impl Action for Trello {
    fn perform(&self, selection: &str) {
        self.create_card(selection);
    }
}
