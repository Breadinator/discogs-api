use super::Endpoint;

pub struct ReleaseRatingByUser;

#[derive(Debug, Clone, Copy)]
pub struct Params<'a> {
    pub release_id: isize,
    pub username: &'a str,
}

impl<'de> Endpoint<'de> for ReleaseRatingByUser {
    type Parameters = Params<'de>;
    type ReturnType = serde_json::Value;

    #[inline(always)]
    fn get_endpoint(params: Self::Parameters) -> String {
        format!("/releases/{}/rating/{}", params.release_id, params.username)
    }

    #[inline(always)]
    fn get_endpoint_with_auth(params: Self::Parameters, personal_access_token: &str) -> String {
        format!(
            "/releases/{0}/rating/{1}?token={personal_access_token}",
            params.release_id, params.username
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn empty_username() {
        let params = super::Params {
            release_id: 27651927,
            username: "",
        };
        let resp = crate::get::<super::ReleaseRatingByUser>(params);
        if let Err(crate::Error::DiscogsError(err)) = resp {
            assert_eq!(&err.message, "The requested resource was not found.");
        } else {
            panic!("Failed in the wrong way.\n{resp:?}");
        }
    }
}
