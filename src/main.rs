use clap::Parser;

mod manejador_crear;
mod manejador_procesar;
mod manejador_analizar;
mod manejador_descargar_letras;
mod manejador_persistir_letras;
mod deezer_obtener_autorizacion;
mod modelos_manejador;
mod modelos_respuestas_deezer;
mod utilidades;

/// Descarga las letras de un disco de Deezer en formato .txt
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ArgumentosCLI {
    /// url directa del disco (separar por comas si se introducen varias)
    #[arg(short, long, use_value_delimiter = true)] // Habilita el uso de un delimitador (coma) para múltiples valores
    url: Vec<String>,

    /// ruta completa al directorio en el que se guardarán los archivos resultantes
    #[arg(short, long, default_value = "")]
    destino: String,

    /// número máximo de descargas simultáneas
    #[arg(short, long, default_value_t = 3)]
    max: u8,
}
#[tokio::main]
async fn main() {
    let argumentos = ArgumentosCLI::parse();
    for url in argumentos.url {
        let mut manejador = manejador_crear::crear_manejador_descargas(url, argumentos.destino.clone(), argumentos.max);
        manejador.analizar().await;
        manejador.descargar().await;
        manejador.persistir_letras();
    }
    // let mut manejador = manejador_crear::crear_manejador_descargas(argumentos);
    // for url in manejador.url {

    // }
}
