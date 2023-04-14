mod imp;

use std::io::{BufRead, BufReader};

use clap::{Parser, Subcommand, ValueEnum};
use serde::Serialize;

/// Tool to explore and modify URLs programmatically.
#[derive(Debug, Parser)]
#[clap(author, version, about, color = clap::ColorChoice::Auto)]
struct Cli {
    /// URL to explore, if absent urlq reads from stdin line by line
    url: Option<url::Url>,
    #[command(subcommand)]
    action: Action,
    /// Output as newline delimited JSON
    #[clap(short, long)]
    json: bool,
}

#[derive(Debug, Subcommand)]
enum Action {
    /// Parts of the url to obtain
    Get { targets: Vec<UrlComponent> },
    /// Parts of the url to update
    Set {
        #[clap(value_parser = clap::value_parser!(SetAction))]
        actions: Vec<SetAction>,
    },
}

#[derive(Debug, Clone)]
struct SetAction {
    target: UrlComponent,
    value: String,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, ValueEnum, Serialize)]
#[serde(rename_all = "camelCase")]
enum UrlComponent {
    Fragment,
    Host,
    Password,
    Path,
    Port,
    Query,
    Scheme,
    User,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let Cli { action, json, url } = Cli::parse();
    if let Some(url) = url {
        transform_url(&action, url, json)?;
    } else {
        let stdin = BufReader::new(std::io::stdin().lock());
        for line in stdin.lines() {
            let url = line?.parse()?;
            transform_url(&action, url, json)?;
        }
    }
    Ok(())
}

fn transform_url(action: &Action, url: url::Url, json: bool) -> color_eyre::Result<()> {
    match action {
        Action::Get { targets } => {
            let map = extract_to_map(&url, targets);
            if json {
                serde_json::to_writer(std::io::stdout().lock(), &map)?;
            } else {
                let mut map = map.into_iter().peekable();
                while let Some((key, value)) = map.next() {
                    if key != "url" {
                        print!("{}", value);
                        if map.peek().is_some() {
                            print!(" ");
                        }
                    }
                }
            }
        }
        Action::Set { actions } => {
            let mut url = url;
            for action in actions {
                action.target.set(&mut url, &action.value);
            }
            if json {
                serde_json::to_writer(
                    std::io::stdout().lock(),
                    &extract_to_map(&url, UrlComponent::value_variants()),
                )?;
            } else {
                print!("{url}");
            }
        }
    }
    println!();
    Ok(())
}

impl UrlComponent {
    fn fetch(&self, url: &url::Url) -> Option<String> {
        match self {
            UrlComponent::Fragment => url.fragment().map(ToString::to_string),
            UrlComponent::Host => url.host_str().map(ToString::to_string),
            UrlComponent::Password => url.password().map(ToString::to_string),
            UrlComponent::Path => Some(url.path().to_owned()),
            UrlComponent::Port => url.port_or_known_default().map(|port| port.to_string()),
            UrlComponent::Query => url.query().map(ToString::to_string),
            UrlComponent::Scheme => Some(url.scheme().to_owned()),
            UrlComponent::User => Some(url.username().to_owned()),
        }
    }

    fn set(&self, url: &mut url::Url, value: &str) {
        match self {
            UrlComponent::Fragment => url.set_fragment(Some(value)),
            UrlComponent::Host => url
                .set_host(Some(value))
                .unwrap_or_else(|_| panic!("invalid host: {value:?}")),
            UrlComponent::Password => url
                .set_password(Some(value))
                .unwrap_or_else(|_| panic!("invalid password: {value:?}")),
            UrlComponent::Path => url.set_path(value),
            UrlComponent::Port => url
                .set_port(value.parse().ok())
                .unwrap_or_else(|_| panic!("invalid port: {value:?}")),
            UrlComponent::Query => url.set_query(Some(value)),
            UrlComponent::Scheme => url
                .set_scheme(value)
                .unwrap_or_else(|_| panic!("invalid scheme: {value:?}")),
            UrlComponent::User => url
                .set_username(value)
                .unwrap_or_else(|_| panic!("invalid user: {value:?}")),
        }
    }
}

fn extract_to_map(url: &url::Url, parts: &[UrlComponent]) -> indexmap::IndexMap<String, String> {
    let mut map = indexmap::IndexMap::new();
    map.insert("url".into(), url.to_string());
    for part in parts {
        if let Some(value) = part.fetch(url) {
            if value.is_empty() {
                continue;
            }
            map.insert(part.to_string(), value.into());
        }
    }
    map
}
