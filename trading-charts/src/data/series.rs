use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}

#[derive(Serialize, Deserialize)]
pub struct Series<Dat: Serialize + Clone, Opt: Serialize + Clone> {
    id:      Option<String>,
    r#type:  String,
    data:    Vec<Dat>,
    options: Option<Opt>,
}

impl<Dat: Serialize + Clone, Opt: Serialize + Clone> Series<Dat, Opt> {
    pub fn new(r#type: impl AsRef<str>) -> Self {
        Self {
            id:      None,
            r#type:  r#type.as_ref().to_string(),
            data:    Vec::new(),
            options: None,
        }
    }

    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    pub fn set_id(&mut self, id: String) {
        self.id.replace(id);
    }

    #[allow(dead_code)]
    pub fn get_type(&self) -> &str {
        &self.r#type
    }

    #[allow(dead_code)]
    pub fn options(&self) -> Option<&Opt> {
        self.options.as_ref()
    }

    #[allow(dead_code)]
    pub fn options_mut(&mut self) -> Option<&mut Opt> {
        self.options.as_mut()
    }

    pub fn set_options(&mut self, options: Opt) {
        self.options.replace(options);
    }

    #[allow(dead_code)]
    pub fn data(&self) -> &Vec<Dat> {
        &self.data
    }

    #[allow(dead_code)]
    pub fn data_mut(&mut self) -> &mut Vec<Dat> {
        &mut self.data
    }

    #[allow(dead_code)]
    pub fn get_data(&self, index: usize) -> Option<&Dat> {
        self.data.get(index)
    }

    #[allow(dead_code)]
    pub fn get_data_mut(&mut self, index: usize) -> Option<&mut Dat> {
        self.data.get_mut(index)
    }

    #[allow(dead_code)]
    pub fn set_data(&mut self, data: Vec<Dat>) {
        self.data = data;
    }

    #[allow(dead_code)]
    pub fn push_data(&mut self, data: Dat) {
        self.data.push(data);
    }

    #[allow(dead_code)]
    pub fn remove_data(&mut self, index: usize) {
        self.data.remove(index);
    }

    #[allow(dead_code)]
    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    #[allow(dead_code)]
    pub fn update_data(&mut self, index: usize, data: Dat) -> bool {
        self.data.get_mut(index).map(|candle| *candle = data).is_some()
    }

    pub fn to_value(&self) -> Result<JsValue, JsValue> {
        Ok(to_value(self)?)
    }
}
