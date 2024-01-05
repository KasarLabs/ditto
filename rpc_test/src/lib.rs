pub mod test_config;
pub mod test_data;

use std::fs::OpenOptions;
use std::io::Write;

use anyhow::Context;
use chrono::Local;
use jsonrpsee::core::client::{ClientT, self};
use jsonrpsee::http_client::HttpClient;
use colored::Colorize;
use jsonrpsee::core::DeserializeOwned;

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
    A: DeserializeOwned
{
    let response: Result<A, client::Error> = client.client.request(cmd, args.clone()).await;

    match response {
        Ok(response) => Ok(response),
        Err(e) => handle_client_error(client, e),
    }
}

fn handle_client_error<A>(client: &ClientInfo, e: client::Error) -> anyhow::Result<A> 
where
    A: DeserializeOwned,
{
    if let Err(e) = log_error(client) {
        println!("{}", e.to_string());
    }

    Err(e).with_context(|| err_context(client))
}

fn log_error(client: &ClientInfo) -> anyhow::Result<()>
{
    let err_log = err_log(client);

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("./test.log")
        .with_context(|| "Failed to open log file at ../../test/test.log")?;

    write!(file, "{}", err_log).with_context(|| "Failed to write to log file at ../../test/test.log")
}

fn err_context(client: &ClientInfo) -> String {
    let name = client.name;
    let path = client.display_path;
    let rpc_header = "RPC call".white().bold();
    let rpc = client.display_test;
    let response_header = "Response format".white().bold();
    let response = client.display_response;

    format!(
        "\
            Error waiting for rpc call response from {name} in test {path}\n\n\
            {rpc_header}: {rpc}\n\n\
            {response_header}: {response}\
        "
    )
}

fn err_log(client: &ClientInfo) -> String {
    let time = Local::now().format("%d-%m-%Y %H:%M:%S").to_string();
    let name = client.name;
    let path = client.display_path;
    let rpc = client.display_test;
    let response = client.display_response;

    format!(
        "\
            {time}\n\
            Error waiting for rpc call response from {name} in test {path}\n\n\
            RPC call: {rpc}\n\n\
            Response format: {response}\n\n\
        "
    )
}