use crate::modelos_manejador;
use crate::utilidades;
use std::fs;
use std::io::Write;

impl modelos_manejador::ManejadorDescargas {
    pub fn persistir_letras(&self) {
        let analisis:&modelos_manejador::Analisis;
        match &self.analisis {
            None => return,
            Some(ok) => {
                analisis = ok
            },
        }
        if !analisis.persistir {
            return
        }

        let nombre_archivo = format!(
            "{}_{}_{}",
            analisis.grupo, analisis.año, analisis.disco
        );

        let nombre_archivo_normalizado = utilidades::normalizar_cadena(nombre_archivo);

        // TXT
        let archivo_txt = format!("{}.txt", nombre_archivo_normalizado);
        let ruta_txt = self.destino.join(archivo_txt);

        let mut manejador_txt;
        match fs::File::create(ruta_txt) {
            Err(error) => {
                eprintln!("ERROR: no se ha podido crear el archivo: {error}");
                return
            }
            Ok(ok) => {
                manejador_txt = ok;
            }
        };
        for cancion in analisis.canciones.clone() {
            let contenido = format!(
                "{}\n{}\n{}\n\n",
                cancion.titulo,
                "=".repeat(cancion.titulo.len()),
                cancion.letra
            );
            match manejador_txt.write(contenido.as_bytes()){
                Err(error) => {
                    eprintln!("ERROR: no se ha podido incorporar la letra de {} por un error de escritura: {error}", cancion.titulo);
                    return;
                }
                Ok(_) => (),
            };
        }


        // JSON
        let archivo_json = format!("{}.json", nombre_archivo_normalizado);
        let ruta_json = self.destino.join(archivo_json);

        let mut manejador_json;
        match fs::File::create(ruta_json) {
            Err(error) => {
                eprintln!("ERROR: no se ha podido crear el archivo json: {error}");
                return
            }
            Ok(ok) => {
                manejador_json = ok;
            }
        };

        let cadena_json: String;
        match serde_json::to_string(&analisis) {
            Err(error) => {
                eprintln!("ERROR: no se ha podido serializar el contenido del archivo json: {error}");
                return
            }
            Ok(ok) => {
                cadena_json = ok;
            }
        }

        match manejador_json.write(cadena_json.as_bytes()){
            Err(error) => {
                eprintln!("no se ha podido crear el archivo json {error}");
                return;
            }
            Ok(_) => (),
        };

        println!("ÉXITO: letras guardadas para el disco {} de {}", analisis.disco, analisis.grupo);
    }
}
