use crate::modelos_respuestas_deezer;

pub async fn obtener_api_token(cliente: &reqwest::Client) -> Result<String,String>{
    let respuesta;
    match cliente.get("https://auth.deezer.com/login/anonymous?jo=p&rto=c")
    .send()
    .await {
        Err(error) => {
            let mensaje_error = format!("no se ha podido realizar la petición para obtener el token anónimo de la api: {}", error);
            return Err(mensaje_error);
        }
        Ok(ok) => {
            respuesta = ok;
        }
    }
    if respuesta.status().as_u16() != 200 {
        let mensaje_error = format!("se ha obtenido un status code incorrecto en la petición para obtener el token anónimo de la api: {}", respuesta.status());
        return Err(mensaje_error);
    };

    let contenido: String;
    match respuesta.text().await {
        Err(error) => {
            let mensaje_error = format!("no se ha podido leer la respuesta para obtener el token anónimo de la api: {error}");
            return Err(mensaje_error);
        }
        Ok(ok) => {
            contenido = ok;
        }
    }

    let serializacion: Result<modelos_respuestas_deezer::DeezeApiLoginAnonimo,serde_json::Error> = serde_json::from_str(&contenido);
    match serializacion {
        Err(error) => {
            let mensaje_error = format!("no se ha podido deserializar el json de la respuesta del token anónimo: {error}");
            return Err(mensaje_error);
        }
        Ok(ok) => {
            return Ok(ok.jwt)
        }
    }
}
