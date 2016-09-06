use std::fmt;
use std::error;
use std::error::Error as StdError;
use iron;
use std::default::Default;
use rustless;

use rustless::server::status;
use rustless::batteries::swagger;
use rustless::{Nesting};
use serde_json;

use connection;
use models::the_dead;

#[derive(Debug)]
pub struct UnauthorizedError;

impl error::Error for UnauthorizedError {
    fn description(&self) -> &str {
        return "UnauthorizedError";
    }
}

impl fmt::Display for UnauthorizedError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(formatter)
    }
}

pub fn start() {
    let mut app = rustless::Application::new(rustless::Api::build(|api| {
        api.prefix("api");
        api.version("v1", rustless::Versioning::Path);

        api.mount(swagger::create_api("api-docs"));

        api.error_formatter(|err, _media| {
            match err.downcast::<UnauthorizedError>() {
                Some(_) => {
                    return Some(rustless::Response::from(
                        status::StatusCode::Unauthorized,
                        Box::new("Please provide correct `token` parameter")
                    ))
                },
                None => None
            }
        });

        api.get("deads", |endpoint| {
            endpoint.summary("Send a list of deceased people.");
            endpoint.desc("List of deceased people acquired from scientific police website.");
            endpoint.handle(|client, params| {
                let conn = connection::db_conn();
                let deads = the_dead::TheDead::latest(&conn);
                let deads_s = serde_json::to_value(&deads);
                return client.json(&&deads_s);
            })
        });

    }));

    swagger::enable(&mut app, swagger::Spec {
        info: swagger::Info {
            title: "Body Count API".to_string(),
            description: Some("body count: a list or total of casualties.".to_string()),
            contact: Some(swagger::Contact {
                name: "Stefan Yohansson".to_string(),
                url: Some("https://github.com/StefanYohansson".to_string()),
                ..Default::default()
            }),
            license: Some(swagger::License {
                name: "MIT".to_string(),
                url: "http://opensource.org/licenses/MIT".to_string()
            }),
            ..Default::default()
        },
        ..Default::default()
    });

    let mut chain = iron::Chain::new(app);
    // @TODO(snotr): get secret from env
    chain.link(::rustless::batteries::cookie::new("secretsecretsecretsecretsecretsecretsecret".as_bytes()));

    iron::Iron::new(chain).http("0.0.0.0:4000").unwrap();
    println!("On 4000");
}
