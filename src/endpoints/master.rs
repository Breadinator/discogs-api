use super::Endpoint;

pub struct Master;

impl<'de> Endpoint<'de> for Master {
    type Parameters = isize;
    type ReturnType = crate::data_types::Master;

    #[inline(always)]
    fn get_endpoint(master_id: Self::Parameters) -> String {
        format!("/masters/{master_id}")
    }

    #[inline(always)]
    fn get_endpoint_with_auth(master_id: Self::Parameters, personal_access_token: &str) -> String {
        format!("/masters/{master_id}?token={personal_access_token}")
    }
}

#[cfg(test)]
mod tests {
    use super::Master;

    #[test]
    fn basic() {
        let id = 3166419;
        let _data = crate::get::<Master>(id).unwrap();
    }

    #[test]
    fn with_auth() {
        let id = 3166419;
        let pat = std::env::var("discogs-pat")
            .expect("expected personal access token in env var `discogs-pat`");
        let _data = crate::get_with_auth::<Master>(id, &pat).unwrap();
    }
}
