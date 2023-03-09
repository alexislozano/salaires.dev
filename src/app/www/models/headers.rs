use axum::http::HeaderMap;

pub struct HXTriggerName(String);

impl From<HeaderMap> for HXTriggerName {
    fn from(headers: HeaderMap) -> Self {
        let str = match headers.get("hx-trigger-name") {
            Some(value) => match value.to_str() {
                Ok(v) => v,
                Err(_) => "",
            },
            _ => "",
        };

        Self(String::from(str))
    }
}

impl<'a> From<&'a HXTriggerName> for &'a str {
    fn from(header: &'a HXTriggerName) -> Self {
        header.0.as_str()
    }
}
