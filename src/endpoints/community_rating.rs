use super::Endpoint;

pub struct CommunityRating;

impl<'de> Endpoint<'de> for CommunityRating {
    type Parameters = isize;
    type ReturnType = crate::data_types::ReleaseRating;

    #[inline(always)]
    fn get_endpoint(params: Self::Parameters) -> String {
        format!("/releases/{params}/rating")
    }

    #[inline(always)]
    fn get_endpoint_with_auth(params: Self::Parameters, personal_access_token: &str) -> String {
        format!("/releases/{params}/rating?token={personal_access_token}")
    }
}

#[cfg(test)]
mod tests {
    use super::CommunityRating;

    #[test]
    fn basic() {
        let id = 27651927;
        let _data = crate::get::<CommunityRating>(id).unwrap();
    }
}
