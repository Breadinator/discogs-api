use super::Endpoint;

pub struct Release;

impl<'de> Endpoint<'de> for Release {
    type Parameters = isize;
    type ReturnType = crate::data_types::Release;

    #[inline(always)]
    fn get_endpoint(params: isize) -> String {
        format!("/releases/{params}")
    }

    #[inline(always)]
    fn get_endpoint_with_auth(params: Self::Parameters, personal_access_token: &str) -> String {
        format!("/releases/{params}?token={personal_access_token}")
    }
}

#[cfg(test)]
mod tests {
    use super::Release;

    #[test]
    fn basic() {
        let id = 27651927;
        let _data = crate::get::<Release>(id).unwrap();
    }

    #[test]
    fn with_auth() {
        let id = 27651927;
        let pat = std::env::var("discogs-pat")
            .expect("expected personal access token in env var `discogs-pat`");
        let _data = crate::get_with_auth::<Release>(id, &pat).unwrap();
    }
}
