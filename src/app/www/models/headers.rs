use axum::http::HeaderMap;

pub struct HXTriggerName(String);

impl TryFrom<HeaderMap> for HXTriggerName {
    type Error = ();

    fn try_from(headers: HeaderMap) -> Result<Self, Self::Error> {
        match headers.get("hx-trigger-name") {
            Some(value) => match value.to_str() {
                Ok(v) => Ok(Self(String::from(v))),
                Err(_) => Err(()),
            },
            _ => Err(()),
        }
    }
}

impl<'a> From<&'a HXTriggerName> for &'a str {
    fn from(header: &'a HXTriggerName) -> Self {
        header.0.as_str()
    }
}
