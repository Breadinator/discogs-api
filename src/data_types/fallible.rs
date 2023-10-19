use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Fallible<R> {
    Ok(R),
    FailureMessage { message: String },
}

impl<R> Fallible<R> {
    #[must_use]
    pub fn ok(self) -> Option<R> {
        if let Self::Ok(r) = self {
            Some(r)
        } else {
            None
        }
    }

    #[must_use]
    pub fn err(self) -> Option<String> {
        if let Self::FailureMessage { message } = self {
            Some(message)
        } else {
            None
        }
    }

    pub fn to_result(&self) -> Result<&R, &String> {
        Result::from(self)
    }

    pub fn unwrap(&self) -> &R {
        self.to_result().unwrap()
    }
}

impl<'a, R> From<&'a Fallible<R>> for Result<&'a R, &'a String> {
    fn from(value: &'a Fallible<R>) -> Self {
        match value {
            Fallible::Ok(r) => Ok(r),
            Fallible::FailureMessage { message } => Err(message),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Fallible;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct OkVariant {
        pub abc: i32,
        pub foo: Vec<String>,
    }

    #[test]
    fn ok() {
        let json = r#"{"abc": 123, "foo": ["b", "a", "r"]}"#;
        let parsed = serde_json::from_str::<Fallible<OkVariant>>(json)
            .unwrap()
            .ok()
            .expect("expected `Fallible::Ok` but got `Fallible::FailureMessage`");
        assert_eq!(parsed.abc, 123);
        assert_eq!(&parsed.foo, &["b", "a", "r"]);
    }

    #[test]
    fn failure() {
        let json = r#"{"message": "foo"}"#;
        let parsed = serde_json::from_str::<Fallible<OkVariant>>(json)
            .unwrap()
            .err()
            .expect("expected `Fallible::FailureMessage` but got `Fallible::Ok`");
        assert_eq!(&parsed, "foo");
    }
}
