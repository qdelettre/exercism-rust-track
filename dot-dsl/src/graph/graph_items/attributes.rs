#[macro_export]
macro_rules! impl_with_attrs {
    ($t:ty) => {
        pub fn with_attrs(mut self, attrs: &[(&'a str, &'a str)]) -> Self {
            attrs.iter().for_each(|(k, v)| {
                self.attrs.entry(k.to_string()).or_insert(v.to_string());
            });
            self
        }

        pub fn get_attr(&self, name: &str) -> Option<&str> {
            let to_find = name.to_string();
            if let Some(i) = self.attrs.get(&to_find) {
                Some(i.as_str())
            } else {
                None
            }
        }
    };
}
