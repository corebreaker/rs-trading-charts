use super::Candle;
use js_sys::Reflect;
use serde::Serialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}

pub(crate) struct Series {
    id:      Option<String>,
    r#type:  String,
    data:    Vec<Candle>,
    options: JsValue,
}

impl Series {
    pub(crate) fn new(r#type: String) -> Self {
        Self {
            id: None,
            r#type,
            data: Vec::new(),
            options: JsValue::NULL,
        }
    }

    pub(crate) fn sample() -> Self {
        let mut data = vec![
            Candle::new(String::from("2024-02-01"), 10., 10.63, 9.49, 9.55),
            Candle::new(String::from("2024-02-02"), 9.55, 10.30, 9.42, 9.94),
            Candle::new(String::from("2024-02-03"), 9.94, 10.17, 9.92, 9.78),
            Candle::new(String::from("2024-02-04"), 9.78, 10.59, 9.18, 9.51),
            Candle::new(String::from("2024-02-05"), 9.51, 10.46, 9.10, 10.17),
            Candle::new(String::from("2024-02-06"), 10.17, 10.96, 10.16, 10.47),
            Candle::new(String::from("2024-02-07"), 10.47, 11.39, 10.40, 10.81),
            Candle::new(String::from("2024-02-08"), 10.81, 11.60, 10.30, 10.75),
            Candle::new(String::from("2024-02-09"), 10.75, 11.60, 10.49, 10.93),
            Candle::new(String::from("2024-02-10"), 10.93, 11.53, 10.76, 10.96),
            Candle::new(String::from("2024-02-11"), 10.96, 11.00, 10.50, 10.55),
        ];

        let mut open = 10.55;
        for i in 12..=30 {
            let high = open + random() * (12.5 - open);
            let low = 8.5 + random() * (open - 8.5);
            let close = low + random() * (high - low);

            data.push(Candle::new(format!("2024-02-{:02}", i), open, high, low, close));
            open = close;
        }

        Self {
            id: None,
            r#type: String::from("candlestick"),
            data,
            options: JsValue::NULL,
        }
    }

    pub(crate) fn with_options(self, options: JsValue) -> Self {
        Self {
            options,
            ..self
        }
    }

    pub(crate) fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    pub(crate) fn set_id(&mut self, id: String) {
        self.id.replace(id);
    }

    pub(crate) fn get_type(&self) -> &str {
        &self.r#type
    }

    pub(crate) fn options(&self) -> &JsValue {
        &self.options
    }

    pub(crate) fn options_mut(&mut self) -> &mut JsValue {
        &mut self.options
    }

    pub(crate) fn set_options(&mut self, options: JsValue) {
        self.options = options;
    }

    pub(crate) fn data(&self) -> &Vec<Candle> {
        &self.data
    }

    pub(crate) fn data_mut(&mut self) -> &mut Vec<Candle> {
        &mut self.data
    }

    pub(crate) fn get_data(&self, index: usize) -> Option<&Candle> {
        self.data.get(index)
    }

    pub(crate) fn get_data_mut(&mut self, index: usize) -> Option<&mut Candle> {
        self.data.get_mut(index)
    }

    pub(crate) fn set_data(&mut self, data: Vec<Candle>) {
        self.data = data;
    }

    pub(crate) fn push_data(&mut self, data: Candle) {
        self.data.push(data);
    }

    pub(crate) fn remove_data(&mut self, index: usize) {
        self.data.remove(index);
    }

    pub(crate) fn clear_data(&mut self) {
        self.data.clear();
    }

    pub(crate) fn update_data(&mut self, index: usize, data: Candle) -> bool {
        self.data.get_mut(index).map(|candle| *candle = data).is_some()
    }

    pub(crate) fn to_value(&self) -> Result<JsValue, JsValue> {
        #[derive(Serialize)]
        struct Series<'a, 'b, 'c> {
            id:      Option<&'a String>,
            r#type:  &'b String,
            data:    &'c Vec<Candle>,
        }

        let series = Series {
            id:      self.id.as_ref(),
            r#type:  &self.r#type,
            data:    &self.data
        };

        let mut res = to_value(&series)?;
        if !self.options.is_null() {
            Reflect::set(&mut res, &JsValue::from_str("options"), &self.options)?;
        }

        Ok(res)
    }
}
