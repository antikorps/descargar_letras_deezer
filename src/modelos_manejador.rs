use std::path::PathBuf;
use serde::Serialize;

pub struct ManejadorDescargas {
    pub cliente: reqwest::Client,
    pub destino: PathBuf,
    pub max: u8,
    pub url: String,
    pub analisis: Option<Analisis>,
}

#[derive(Clone, Serialize)]
pub struct Analisis {
    pub grupo: String,
    pub disco: String,
    pub año: String,
    #[serde(skip_serializing)]
    pub persistir: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ausencias: Vec<String>,
    pub canciones: Vec<Cancion>,
    #[serde(skip_serializing)]
    pub token: String,
}

#[derive(Clone, Serialize)]
pub struct Cancion {
    #[serde(skip_serializing)]
    pub id_cancion: String,
    #[serde(skip_serializing)]
    pub id_letras: i64,
    pub titulo: String,
    pub letra: String,
}

pub struct Letra {
    pub letra: String,
    pub indice: u16,
}
