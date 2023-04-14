use clap::ValueEnum;

use crate::{SetAction, UrlComponent};

impl std::str::FromStr for SetAction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((target, value)) = s.split_once('=') {
            match <UrlComponent as ValueEnum>::from_str(target, true) {
                Ok(target) => Ok(SetAction {
                    target,
                    value: value.to_owned(),
                }),
                Err(error) => Err(error),
            }
        } else {
            Err(format!(r#"should be "url_part=value" got {s:?}"#))
        }
    }
}

impl std::fmt::Display for UrlComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            UrlComponent::Fragment => "fragment",
            UrlComponent::Host => "host",
            UrlComponent::Password => "password",
            UrlComponent::Path => "path",
            UrlComponent::Port => "port",
            UrlComponent::Query => "query",
            UrlComponent::Scheme => "scheme",
            UrlComponent::User => "user",
        };
        write!(f, "{s}")
    }
}
