use crate::ast_model::{Ast, ValeCode};
use reqwest::blocking::Response;

static BUILD_URI: &str = "https://us-central1-valesite.cloudfunctions.net/vale-build";
static RUN_URI: &str = "https://us-central1-valesite.cloudfunctions.net/vale-run";

pub fn build_vale(vale_code: ValeCode) -> Result<Ast, Box<dyn std::error::Error>> {
    let ast_json = request_vale_server(BUILD_URI.into(), vale_code);

    return ast_json;
}

pub fn run_vale(vale_code: ValeCode) -> Result<Ast, Box<dyn std::error::Error>> {
    let ast_json = request_vale_server(RUN_URI.into(), vale_code);

    return ast_json;
}

fn request_vale_server(url: &str, post_data: ValeCode) -> Result<Ast, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let res: Response = client.post(url).body(post_data.code).send()?;
    let result: Ast = res.json()?;

    Ok(result)
}
