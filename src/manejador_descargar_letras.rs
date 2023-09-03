use crate::modelos_manejador;
use crate::modelos_respuestas_deezer;
use reqwest::Client;

impl modelos_manejador::ManejadorDescargas {
    pub async fn descargar(&mut self) {
        let analisis: &modelos_manejador::Analisis;
        match &self.analisis {
            None => {
                return
            }
            Some(ok) => {
                analisis = ok
            }
        }

        let mut nuevo_analisis = analisis.clone();

        let mut indice: u16 = 0;
        for lote in analisis.canciones.chunks(self.max as usize) {
            
            let mut futures = Vec::new();
    
            for cancion in lote {
                indice += 1;
                if &cancion.id_letras == &0 {
                    continue
                }
                futures.push(descargar_cancion(&self.cliente, &cancion.id_cancion, &analisis.token, indice))
            }
            let resultado = futures::future::join_all(futures).await;
            
            for r in resultado {
                match r {
                    Err(error) => {
                        eprintln!("{error}");
                    }
                    Ok(ok) => {
                        let i = ok.indice as usize - 1; 
                        nuevo_analisis.canciones[i].letra = ok.letra
                    }
                }
            }
        }

        self.analisis = Some(nuevo_analisis);        
    }
}


async fn descargar_cancion(cliente: &Client, id_cancion: &String, token: &String, indice: u16) -> Result<modelos_manejador::Letra, String>{
    let bearer = format!("Bearer {}", token);
        
    let json_string = format!(
        r#"{{
            "operationName": "SynchronizedTrackLyrics",
            "variables": {{
                "trackId": "{}"
            }},
            "query": "query SynchronizedTrackLyrics($trackId: String!) {{\n  track(trackId: $trackId) {{\n    ...SynchronizedTrackLyrics\n    __typename\n  }}\n}}\n\nfragment SynchronizedTrackLyrics on Track {{\n  id\n  lyrics {{\n    ...Lyrics\n    __typename\n  }}\n  album {{\n    cover {{\n      small: urls(pictureRequest: {{width: 100, height: 100}})\n      medium: urls(pictureRequest: {{width: 264, height: 264}})\n      large: urls(pictureRequest: {{width: 800, height: 800}})\n      explicitStatus\n      __typename\n    }}\n    __typename\n  }}\n  __typename\n}}\n\nfragment Lyrics on Lyrics {{\n  id\n  copyright\n  text\n  writers\n  synchronizedLines {{\n    ...LyricsSynchronizedLines\n    __typename\n  }}\n  __typename\n}}\n\nfragment LyricsSynchronizedLines on LyricsSynchronizedLine {{\n  lrcTimestamp\n  line\n  lineTranslated\n  milliseconds\n  duration\n  __typename\n}}"
        }}"#,
        id_cancion
    );

    let respuesta;
    match cliente.post("https://pipe.deezer.com/api")
        .header("authorization", bearer)
        .body(json_string)        
        .send()
        .await {
            Err(error) => {
                let mensaje_error = format!("no se ha podido realizar la petición para la cancion {id_cancion}: {error}");
                return Err(mensaje_error);
            }
            Ok (ok) => {
                respuesta = ok
            }
    }
    if respuesta.status().as_u16() != 200 {
        let mensaje_error = format!("la petición para la cancion {id_cancion} ha obtenido un status code incorrecto: {}", respuesta.status());
        return Err(mensaje_error)
    };

    let contenido: String;
    match respuesta.text().await {
        Err(error) => {
            let mensaje_error = format!("no se ha podido leer la respuesta de la canción {id_cancion}: {error}");
            return Err(mensaje_error);
        }
        Ok(ok) => {
            contenido = ok;
        }
    }
    let letra_data;
    let deserializacion: Result<modelos_respuestas_deezer::DeezerLetraData, serde_json::Error> = serde_json::from_str(&contenido);
    match deserializacion {
        Err(error) => {
            let mensaje_error = format!("no se ha podido serializar el json de la respuesta de {id_cancion}: {error}");
            return Err(mensaje_error);
        }
        Ok(ok) => {
            letra_data = ok;
        }
    }
    let letra = letra_data.data.track.lyrics.text;
    Ok(modelos_manejador::Letra{
        letra,
        indice
    })
}