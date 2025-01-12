use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// Estructura para manejar un evento
#[derive(Serialize, Deserialize)]
pub struct Event {
    name: String,
    votes: i32,
}

// Exponer una función inicial para crear eventos desde un texto
#[wasm_bindgen]
pub fn initialize_events(events_data: &str) -> Result<JsValue, JsValue> {
    // Dividir eventos por líneas
    let events: Vec<Event> = events_data
        .lines()
        .map(|line| Event {
            name: line.to_string(),
            votes: 0,
        })
        .collect();

    // Retornar los eventos como JSON
    serde_json::to_string(&events)
        .map(JsValue::from_str)
        .map_err(|err| JsValue::from_str(&err.to_string()))
}

// Exponer una función para votar
#[wasm_bindgen]
pub fn vote(events_json: &str, event_index: usize, vote: i32) -> Result<JsValue, JsValue> {
    // Parsear eventos desde JSON
    let mut events: Vec<Event> = serde_json::from_str(events_json)
        .map_err(|err| JsValue::from_str(&err.to_string()))?;

    // Aplicar voto si el índice es válido
    if let Some(event) = events.get_mut(event_index) {
        event.votes += vote;
    } else {
        return Err(JsValue::from_str("Índice de evento inválido"));
    }

    // Retornar los eventos actualizados como JSON
    serde_json::to_string(&events)
        .map(JsValue::from_str)
        .map_err(|err| JsValue::from_str(&err.to_string()))
}
