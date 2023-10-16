use super::Endpoint;

pub struct ReleaseStats;

impl<'de> Endpoint<'de> for ReleaseStats {
    type Parameters = isize;
    type ReturnType = serde_json::Value;

    #[inline(always)]
    fn get_endpoint(release_id: Self::Parameters) -> String {
        format!("/releases/{release_id}/stats")
    }

    #[inline(always)]
    fn get_endpoint_with_auth(release_id: Self::Parameters, personal_access_token: &str) -> String {
        format!("/releases/{release_id}/stats?token={personal_access_token}")
    }
}

#[cfg(test)]
mod tests {
    use super::ReleaseStats;

    #[test]
    fn basic() {
        let id = 27736512;
        let _data = dbg![crate::get::<ReleaseStats>(id).unwrap()];
    }
}
