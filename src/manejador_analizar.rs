use crate::modelos_manejador;
use crate::modelos_respuestas_deezer;
use crate::deezer_obtener_autorizacion;
use regex::Regex;

pub async fn analizar_disco(url: &String, cliente: &reqwest::Client) -> Result<modelos_manejador::Analisis,String>{
    let respuesta;
    match cliente.get(url).send().await {
        Err(error) => {
            let mensaje_error = format!("ERROR: no se ha podido realizar la petición de análisis: {error}");
            return Err(mensaje_error);
        }
        Ok(ok) => {
            respuesta = ok;
        }
    }
    if respuesta.status().as_u16() != 200 {
        let mensaje_error = format!("ERROR: la respuesta de análisis ha devuelto un status code incorrecto: {}", respuesta.status());
        return Err(mensaje_error);
    }

    let contenido: String;
    match respuesta.text().await {
        Err(error) => {
            let mensaje_error = format!("ERROR: no se ha podido leer la respuesta del análisis: {error}");
            return Err(mensaje_error);
        }
        Ok(ok) => {
            contenido = ok;
        }
    }

    let regex_min = Regex::new("\n").unwrap();
    let contenido_min = regex_min.replace_all(&contenido, "");
    
    let regex_data = Regex::new(".*?window.__DZR_APP_STATE__ = (.*?)<.script>.*").unwrap();
    let deezer_html_json = regex_data.replace_all(&contenido_min, "$1");

    if !deezer_html_json.starts_with("{") {
        return Err("ERROR: la expresión regular para obtener la información del disco parece no funcionar porque se esperaba que el resultado comenzara con un corchete".to_string());
    }
    if !deezer_html_json.ends_with("}") {
        return Err("ERROR: la expresión regular para obtener la información del disco parece no funcionar porque se esperaba que el resultado finalizara con un corchete".to_string());
    }

    let html_json;
    let deserializacion: Result<modelos_respuestas_deezer::DeezerHTMLData, serde_json::Error> = serde_json::from_str(&deezer_html_json);
    match deserializacion {
        Err(error) => {
            let mensaje_error = format!("ERROR: no se ha podido deserializar el json encontrado en el html: {error}");
            return Err(mensaje_error);
        }
        Ok(ok) => {
            html_json = ok;
        }
    }

    let grupo = html_json.data.art_name;
    let disco = html_json.data.alb_title;
    let info_publicacion: Vec<&str> = html_json.data.original_release_date.split("-").collect();
    let año = info_publicacion[0].to_string();
    let mut completo = true;
    let mut persistir = false;
    let mut ausencias = Vec::new();

    let mut canciones = Vec::new();
    for cancion in html_json.songs.data {
        let titulo = cancion.sng_title;
        let id_cancion = cancion.sng_id;
        let id_letras =  cancion.lyrics_id;
        if id_letras == 0 {
            completo = false;
            ausencias.push(titulo);
            continue;
        }
        persistir = true;
        
        canciones.push(modelos_manejador::Cancion{
            id_letras,
            titulo,
            id_cancion,
            letra: "".to_string(),
        })
    }
    if canciones.is_empty() {
        let mensaje_error = format!("ERROR: el disco {disco} de {grupo} no tiene ninguna letra para descargar");
        return Err(mensaje_error);
    }

    if !completo {
        let falta_canciones = ausencias.join(", ");
        println!("ATENCIÓN: el disco {disco} de {grupo} no tiene letras para todas las canciones. Faltan: {falta_canciones}");
    }
    
    match deezer_obtener_autorizacion::obtener_api_token(cliente).await {
        Err(error) => {
            return Err(error)
        }
        Ok(token) => {
            
            Ok(modelos_manejador::Analisis{
                persistir,
                ausencias,
                grupo,
                disco,
                año,
                canciones,
                token
            })
        }
    }
}