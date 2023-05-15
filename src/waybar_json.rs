use serde::Serialize;
use std::collections::HashMap;

use crate::meteorology::Observations;

// example
// {"text": "$text", "alt": "$alt", "tooltip": "$tooltip", "class": "$class", "percentage": $percentage }.

fn weather_codes(key: i32) -> &'static str {
    let map: HashMap<i32, &'static str> = HashMap::from([
        (113, "☀️"),
        (116, "⛅️"),
        (119, "☁️"),
        (122, "☁️"),
        (143, "🌫"),
        (176, "🌦"),
        (179, "🌧"),
        (182, "🌧"),
        (185, "🌧"),
        (200, "⛈"),
        (227, "🌨"),
        (230, "❄️"),
        (248, "🌫"),
        (260, "🌫"),
        (263, "🌦"),
        (266, "🌦"),
        (281, "🌧"),
        (284, "🌧"),
        (293, "🌦"),
        (296, "🌦"),
        (299, "🌧"),
        (302, "🌧"),
        (305, "🌧"),
        (308, "🌧"),
        (311, "🌧"),
        (314, "🌧"),
        (317, "🌧"),
        (320, "🌨"),
        (323, "🌨"),
        (326, "🌨"),
        (329, "❄️"),
        (332, "❄️"),
        (335, "❄️"),
        (338, "❄️"),
        (350, "🌧"),
        (353, "🌦"),
        (356, "🌧"),
        (359, "🌧"),
        (362, "🌧"),
        (365, "🌧"),
        (368, "🌨"),
        (371, "❄️"),
        (374, "🌧"),
        (377, "🌧"),
        (386, "⛈"),
        (389, "🌩"),
        (392, "⛈"),
        (395, "❄️"),
    ]);

    map.get(&key).unwrap_or(&"⛅️")
}

#[derive(Debug, Default, Serialize)]
pub struct Waybar {
    #[serde(skip_serializing_if = "String::is_empty")]
    text: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    alt: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    tooltip: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    class: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    percentage: String,
}

impl Waybar {
    pub fn set_text(self, text: String) -> Self {
        Self {
            text,
            ..Default::default()
        }
    }
}

pub fn get_icon(observations: &Observations) -> &'static str {
    const RAIN: &str = "Accumulated Rainfall (10 min.)";
    const TEMPERATURE: &str = "Air Temperature (1.2m)";
    
    let rain = observations.get_value(RAIN);
    let temperature = observations.get_value(TEMPERATURE);

    if rain == Some(0.0) && temperature > Some(30.0) {
        weather_codes(113)
    }
    else if rain == Some(0.0) {
        weather_codes(116)
    }
    else {
        weather_codes(179)
    }
}
