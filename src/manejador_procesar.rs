use crate::manejador_analizar;
use crate::modelos_manejador;

impl modelos_manejador::ManejadorDescargas {
    pub async fn analizar(&mut self) {
        match manejador_analizar::analizar_disco(&self.url, &self.cliente).await {
            Err(error) => {
                eprintln!("{error}");
                return;
            }
            Ok(ok) => self.analisis = Some(ok),
        }
    }
}
