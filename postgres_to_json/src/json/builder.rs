use crate::json::formatter::convert_str_to_json_item;

pub struct JsonBuilder {
    pub json: String,
    tab: u8,
}

impl JsonBuilder {
    pub fn init() -> JsonBuilder {
        JsonBuilder {
            json: "{\n".to_owned(),
            tab: 1,
        }
    }

    pub fn close(&mut self) -> String {
        self.json.push_str("\n}");
        return self.json.clone();
    }

    fn create_tabs(&self) -> String {
        let mut tabs = String::from("");
        for _ in 0..self.tab {
            tabs.push_str("\t");
        }

        tabs
    }

    pub fn create_item(&mut self, key: &str, value: String) {
        let tabs = self.create_tabs();
        self.json
            .push_str(format!("{}{},\n", tabs, convert_str_to_json_item(key, value)).as_str());
    }

    pub fn create_list(&mut self, list_name: String) {
        let tabs = self.create_tabs();
        self.json
            .push_str(format!("{}\"{}\": [\n", tabs, list_name).as_str());
        self.tab += 1;
    }

    pub fn create_unamed_obj(&mut self) {
        let tabs = self.create_tabs();
        self.json.push_str(format!("{}{{\n", tabs).as_str());
        self.tab += 1;
    }

    pub fn close_obj(&mut self) {
        self.tab -= 1;
        let tabs = self.create_tabs();
        self.json.push_str(format!("{}}},\n", tabs).as_str())
    }

    pub fn close_list(&mut self) {
        self.tab -= 1;
        let tabs = self.create_tabs();
        self.json.push_str(format!("{}],\n", tabs).as_str())
    }
}
