use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use regex::Regex;

pub fn crear_cliente() -> Client {
    let mut headers = HeaderMap::new();
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    headers.insert(
        reqwest::header::USER_AGENT,
        HeaderValue::from_static(
            "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/116.0",
        ),
    );

    Client::builder()
        .default_headers(headers)
        .build()
        .expect("no se ha podido crear el construir el cliente de reqwest")
}

pub fn normalizar_cadena(cadena: String) -> String {
    let c = unidecode::unidecode(&cadena);
    let exp_reg_w = Regex::new(r"\W").unwrap();
    let d = exp_reg_w.replace_all(&c, "_");
    let exp_reg_guiones = Regex::new(r"_{2,}").unwrap();
    return exp_reg_guiones.replace(&d, "_").to_lowercase().to_string();
}