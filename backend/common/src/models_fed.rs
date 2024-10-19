use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Serie {
    id: String,
    realtime_start: String,
    realtime_end: String,
    title: String,
    observation_start: String,
    observation_end: String,
    frequency: String,
    frequency_short: String,
    units: String,
    units_short: String,
    seasonal_adjustment: String,
    seasonal_adjustment_short: String,
    last_updated: String,
    popularity: u16,
    notes: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FedSeries {
    realtime_start: String,
    realtime_end: String,
    seriess: Vec<Serie>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObservationResult {
    realtime_start: String,
    realtime_end: String,
    observation_start: String,
    observation_end: String,
    units: String,
    output_type: i16,
    file_type: String,
    order_by: String,
    sort_order: String,
    offset: i32,
    limit: i32,
    count: i32,
    observations: Vec<Observation>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Observation {
    realtime_start: String,
    realtime_end: String,
    date: String,
    value: String,
}
