use std::{env::current_exe, path::PathBuf};

use crate::modelos_manejador;
use crate::utilidades;

pub fn crear_manejador_descargas(url: String, destino: String, max: u8) -> modelos_manejador::ManejadorDescargas {
    let mut ruta_destino = PathBuf::new();
    if destino == "" {
        let ruta_ejecutable =
            current_exe().expect("no se ha podido obtener la ruta del ejecutable");
        match ruta_ejecutable.parent() {
            None => {
                panic!("no se ha podido encontrar el directorio donde se encuentra el ejecutable")
            }
            Some(ok) => ruta_destino = ok.to_path_buf(),
        }
    }
    return modelos_manejador::ManejadorDescargas {
        analisis: None,
        cliente: utilidades::crear_cliente(),
        destino: ruta_destino,
        max,
        url: url.trim().to_string(),
    };
}
