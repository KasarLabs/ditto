pub mod test_config;
pub mod test_data;

use std::fs::OpenOptions;
use std::io::Write;

use anyhow::Context;
use chrono::Local;
use jsonrpsee::core::client::{ClientT, self};
use jsonrpsee::http_client::HttpClient;
use colored::Colorize;

pub struct ClientInfo<'a> {
    client: &'a HttpClient,
    name: &'a str,
    display_test: &'a str,
    display_response: &'a str,
    display_path: &'a str,
}

impl <'a> ClientInfo<'a> {
    pub fn new(client: &'a HttpClient, name: &'a str, test: &'a str, response: &'a str, path: &'a str) -> Self{
        ClientInfo { 
            client: client,
            name: name, 
            display_test: test, 
            display_response: response, 
            display_path: path 
        }
    }
}

pub async fn client_response<'a, A>(client: &ClientInfo<'a>, cmd: &str, args: &Vec<String>) -> anyhow::Result<A>
where
    A: jsonrpsee::core::DeserializeOwned
{
    let response: Result<A, client::Error> = client.client.request(cmd, args.clone()).await;

    match response {
        Ok(response) => Ok(response),
        Err(e) => handle_client_error(client, e),
    }
}

fn handle_client_error<'a, A>(client: &ClientInfo<'a>, e: client::Error) -> anyhow::Result<A> 
where
    A: jsonrpsee::core::DeserializeOwned,
{
    let err_context = err_context(client);
    let err_log = err_log(client);

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("./test.log")
        .with_context(|| "Failed to open log file at ../../test/test.log")
        .unwrap();

    write!(file, "{}", err_log)?;

    Err(e).with_context(|| err_context)
}

fn err_context(client: &ClientInfo) -> String {
    format!(
        "\
            Error waiting for rpc call response from {} in test {}\n\n\
            {}: {}\n\n\
            {}: {}\
        ",
        client.name,
        client.display_path.underline().blue(),
        "RPC call".white().bold(),
        client.display_test,
        "Response format".white().bold(),
        client.display_response
    )
}

fn err_log(client: &ClientInfo) -> String {
    let now = Local::now().format("%d-%m-%Y %H:%M:%S").to_string();

    format!(
        "\
            {}\n\
            Error waiting for rpc call response from {} in test {}\n\n\
            RPC call: {}\n\n\
            Response format: {}\n\n\
        ",
        now,
        client.name,
        client.display_path,
        client.display_test,
        client.display_response
    )
}