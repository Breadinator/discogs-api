use serde::Deserialize;

pub mod community_rating;
pub mod master;
pub mod master_versions;
pub mod release;
pub mod release_rating_by_user;
pub mod release_stats;

pub trait Endpoint<'de> {
    type Parameters;
    type ReturnType: Deserialize<'de>;
    /// Builds actual the endpoint
    fn get_endpoint(params: Self::Parameters) -> String;

    fn get_endpoint_with_auth(params: Self::Parameters, personal_access_token: &str) -> String;
}
