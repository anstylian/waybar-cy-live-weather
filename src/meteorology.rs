use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Meteorology {
    copyright: String,
    stations: Stations,
    observations: Vec<Observations>,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Stations {
    station: Vec<Station>,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Station {
    station_code: String,
    station_latitude: f32,
    station_longitude: f32,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Observations {
    station_code: String,
    date_time: String,
    observation: Vec<Observation>,
}

#[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Observation {
    observation_name: String,
    observation_value: f32,
    observation_unit: String,
}

impl Meteorology {
    pub fn get_stations_name(&self) -> Vec<&str> {
        self.stations
            .station
            .iter()
            .map(|s| s.station_code.as_str())
            .collect()
    }

    pub fn get_station_options(&self, station: String) -> Option<&Observations> {
        self.observations
            .iter()
            .find(|&o| o.station_code == station)
    }
}

impl Observations {
    pub fn get_value(&self, name: &str) -> Option<f32> {
        self.observation.iter().find_map(|o| {
            if o.observation_name == name {
                Some(o.observation_value)
            } else {
                None
            }
        })
    }
}
