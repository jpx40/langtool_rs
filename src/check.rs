pub struct Regwest {
    pub text: Option<String>,
    pub data: Option<String>,
    pub language: String,
    pub username: Option<String>,
    pub api_key: Option<String>,
    pub dicts: Option<String>,
}
impl Regwest {
    pub fn new(language: &str) -> Regwest {
        Regwest {
            text: None,
            data: None,
            language: language.to_string(),
            username: None,
            api_key: None,
            dicts: None,
        }
    }
    pub fn set_text(&mut self, text: String) {
        self.text = Some(text);
    }
    pub fn set_data(&mut self, data: String) {
        self.data = Some(data);
    }
    pub fn set_api_key(&mut self, api_key: String) {
        self.api_key = Some(api_key);
    }
}
