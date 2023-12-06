#[macro_use] extern crate rocket;

#[get("/")]
pub fn index() -> Json<IndexResponse> {
    let uptime = obtener_tiempo_de_funcionamiento(); // Implementa esta función según tu lógica
    let server_time = Utc::now().to_rfc3339();
    let environment = std::env::var("RUN_ENV").unwrap_or_else(|_| "desconocido".to_string());
    
    Json(IndexResponse {
        message: "Bienvenido a la API de Traducción e Interpretación",
        server_time,
        uptime,
        environment,
        rust_version: "1.50.0", // Ejemplo, usa la versión real
    })
}

#[derive(Serialize)]
struct IndexResponse {
    message: &'static str,
    server_time: String,
    uptime: String,
    environment: String,
    rust_version: &'static str,
}

fn obtener_tiempo_de_funcionamiento() -> String {
    let uptime = Instant::now().duration_since(*START_TIME);
    format_duration(uptime)
}


fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}


