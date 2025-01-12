use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{StreamExt, SinkExt};
use tokio;

#[tokio::main]
async fn main() {
    // Conectar al servidor WebSocket
    let url = "ws://127.0.0.1:3030/chat";
    let (mut ws_stream, _) = connect_async(url).await.expect("Error al conectar");

    println!("Conectado al servidor WebSocket");

    // Enviar un mensaje de error, alerta o emergencia
    let msg_type = "error";  // Cambiar entre "error", "alerta", "emergencia"
    let msg = match msg_type {
        "error" => "¡Error! Algo salió mal.",
        "alerta" => "¡Alerta! Requiere atención.",
        "emergencia" => "¡Emergencia! Acción inmediata requerida.",
        _ => "Mensaje desconocido",
    };

    // Enviar el mensaje
    ws_stream.send(Message::Text(msg.to_string())).await.expect("Error al enviar mensaje");

    // Recibir mensajes desde el servidor
    while let Some(Ok(msg)) = ws_stream.next().await {
        match msg {
            Message::Text(txt) => println!("Mensaje recibido: {}", txt),
            _ => (),
        }
    }
}