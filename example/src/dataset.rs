use charts::{
    data::{Candlestick, Marker, UTCTimestamp},
    JsError,
};

use serde::Deserialize;
use wasm_bindgen::prelude::wasm_bindgen;
use std::{
    io::{Error, ErrorKind, Result},
    str::FromStr,
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}

#[derive(Deserialize, Clone)]
pub(super) struct Dataset {
    data_up:   Vec<Candlestick>,
    data_down: Vec<Candlestick>,
    markers:   Vec<Marker>,

    #[serde(default)]
    counter: usize,

    #[serde(default)]
    generated: bool,
}

impl Dataset {
    pub(super) fn new() -> Self {
        let data_up = make_dataset();
        let data_down = make_dataset();
        let markers = make_markers(&data_up, 0);
        let counter = 0;

        Self {
            data_up,
            data_down,
            markers,
            counter,
            generated: true,
        }
    }

    pub(super) fn load(kind: &str) -> Result<Dataset> {
        let data = match kind {
            "data1" => include_str!("../data/data1.json"),
            "data2" => include_str!("../data/data2.json"),
            _ => return Err(Error::new(ErrorKind::InvalidInput, "Unknown data kind")),
        };

        let mut r: Dataset = serde_json::from_str(data).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        r.data_up.sort_by_key(|d| d.time());
        r.data_down.sort_by_key(|d| d.time());
        r.markers.sort_by_key(|d| d.time());

        Ok(r)
    }

    pub(super) fn inc(&mut self) {
        if !self.generated {
            *self = Dataset::new();

            return;
        }

        self.counter = (self.counter + 1) % 16;
        self.markers = make_markers(&self.data_down, self.counter);
        self.data_down = make_dataset();
    }

    pub(super) fn data_up(&self) -> &Vec<Candlestick> {
        &self.data_up
    }

    pub(super) fn data_down(&self) -> &Vec<Candlestick> {
        &self.data_down
    }

    pub(super) fn markers(&self) -> &Vec<Marker> {
        &self.markers
    }
}

fn make_data() -> std::result::Result<Vec<Candlestick>, JsError> {
    let mut data = vec![
        Candlestick::new(UTCTimestamp::from_str("2024-04-01")?, 10., 10.63, 9.49, 9.55),
        Candlestick::new(UTCTimestamp::from_str("2024-04-02")?, 9.55, 10.30, 9.42, 9.94),
        Candlestick::new(UTCTimestamp::from_str("2024-04-03")?, 9.94, 10.17, 9.92, 9.78),
        Candlestick::new(UTCTimestamp::from_str("2024-04-04")?, 9.78, 10.59, 9.18, 9.51),
        Candlestick::new(UTCTimestamp::from_str("2024-04-05")?, 9.51, 10.46, 9.10, 10.17),
        Candlestick::new(UTCTimestamp::from_str("2024-04-06")?, 10.17, 10.96, 10.16, 10.47),
        Candlestick::new(UTCTimestamp::from_str("2024-04-07")?, 10.47, 11.39, 10.40, 10.81),
        Candlestick::new(UTCTimestamp::from_str("2024-04-08")?, 10.81, 11.60, 10.30, 10.75),
        Candlestick::new(UTCTimestamp::from_str("2024-04-09")?, 10.75, 11.60, 10.49, 10.93),
        Candlestick::new(UTCTimestamp::from_str("2024-04-10")?, 10.93, 11.53, 10.76, 10.96),
        Candlestick::new(UTCTimestamp::from_str("2024-04-11")?, 10.96, 11.00, 10.50, 10.55),
    ];

    let mut open = 10.55;
    let mut i = 12usize;
    for m in 4usize..9 {
        let s = i;
        i = 1;

        for d in s..=28 {
            let high = open + random() * (12.5 - open);
            let low = 8.5 + random() * (open - 8.5);
            let close = low + random() * (high - low);

            data.push(Candlestick::new(
                UTCTimestamp::from_str(&format!("2024-{m:02}-{d:02}"))?,
                open,
                high,
                low,
                close,
            ));
            open = close;
        }
    }

    Ok(data)
}

fn make_markers(data: &Vec<Candlestick>, delta: usize) -> Vec<Marker> {
    let mut markers = vec![];
    let limit = data.len().saturating_sub(18);

    markers.push(Marker::buy(data[8 + delta].time()).with_text(String::from("B0")));
    markers.push(Marker::buy(data[11 + delta].time()).with_text(String::from("Yes, buy here")));
    markers.push(Marker::sell(data[16 + delta].time()).with_text(String::from("Sell here, sure ?")));
    markers.push(Marker::buy(data[19 + delta].time()).with_text(String::from("B1")));
    markers.push(Marker::sell(data[23 + delta].time()).with_text(String::from("S1")));
    markers.push(Marker::sell(data[26 + delta].time()).with_text(String::from("S2")));
    markers.push(Marker::sell(data[32 + delta].time()).with_text(String::from("S3")));
    markers.push(Marker::buy(data[35 + delta].time()).with_text(String::from("B2")));
    markers.push(Marker::buy(data[40 + delta].time()).with_text(String::from("B3")));
    markers.push(Marker::buy(data[42 + delta].time()).with_text(String::from("B4")));
    markers.push(Marker::sell(data[47 + delta].time()).with_text(String::from("S4")));
    markers.push(Marker::buy(data[48 + delta].time()).with_text(String::from("B5")));
    markers.push(Marker::sell(data[53 + delta].time()).with_text(String::from("S5")));
    markers.push(Marker::buy(data[60 + delta].time()).with_text(String::from("B6")));
    markers.push(Marker::buy(data[65 + delta].time()).with_text(String::from("B7")));
    markers.push(Marker::buy(data[68 + delta].time()).with_text(String::from("B7")));
    markers.push(Marker::sell(data[70 + delta].time()).with_text(String::from("S6")));
    markers.push(Marker::sell(data[72 + delta].time()).with_text(String::from("S7")));

    markers.push(Marker::buy(data[limit.saturating_sub(25) + delta].time()).with_text(String::from("Bx1")));
    markers.push(Marker::buy(data[limit.saturating_sub(22) + delta].time()).with_text(String::from("Bx2")));
    markers.push(Marker::buy(data[limit.saturating_sub(18) + delta].time()).with_text(String::from("Bx3")));
    markers.push(Marker::sell(data[limit.saturating_sub(17) + delta].time()).with_text(String::from("Sx1")));
    markers.push(Marker::sell(data[limit.saturating_sub(15) + delta].time()).with_text(String::from("Sx2")));
    markers.push(Marker::buy(data[limit.saturating_sub(12) + delta].time()).with_text(String::from("Bx4")));
    markers.push(Marker::sell(data[limit.saturating_sub(8) + delta].time()).with_text(String::from("Sx3")));
    markers.push(Marker::buy(data[limit.saturating_sub(5) + delta].time()).with_text(String::from("Bx5")));
    markers.push(Marker::buy(data[limit.saturating_sub(2) + delta].time()).with_text(String::from("Bx6")));
    markers.push(Marker::sell(data[limit + delta].time()).with_text(String::from("Sx4")));

    markers
}

fn make_dataset() -> Vec<Candlestick> {
    make_data().unwrap_or_else(|err| {
        err.log();

        vec![]
    })
}
