use super::Endpoint;
use crate::{client::build_params, Error};
use reqwest::Url;

pub struct MasterVersions;

#[derive(Debug, Clone, Default)]
pub struct MasterVersionsParams {
    pub master_id: isize,
    pub page: Option<usize>,
    pub per_page: Option<usize>,
    /// Filter by format, e.g. "Vinyl"
    pub format: Option<String>,
    /// Filter by label
    pub label: Option<String>,
    /// Filter by year released,
    pub released: Option<u16>,
    /// Filter by country of release
    pub country: Option<String>,
    pub sort: Option<MasterVersionsSort>,
    pub sort_order: Option<MasterVersionsSortOrder>,
}

#[derive(Debug, Clone)]
pub enum MasterVersionsSort {
    Released,
    Title,
    Format,
    Label,
    CatNo,
    Country,
}

impl AsRef<str> for MasterVersionsSort {
    fn as_ref(&self) -> &str {
        match self {
            Self::Released => "released",
            Self::Title => "title",
            Self::Format => "format",
            Self::Label => "label",
            Self::CatNo => "catno",
            Self::Country => "country",
        }
    }
}

#[derive(Debug, Clone)]
pub enum MasterVersionsSortOrder {
    Asc,
    Desc,
}

impl AsRef<str> for MasterVersionsSortOrder {
    fn as_ref(&self) -> &str {
        match self {
            Self::Asc => "asc",
            Self::Desc => "desc",
        }
    }
}

impl MasterVersionsParams {
    fn build(&self, personal_access_token: Option<&str>) -> String {
        let mut params = Vec::with_capacity(9);
        let page_owned: String;
        let per_page_owned: String;
        let released_owned: String;

        if let Some(page) = self.page {
            page_owned = page.to_string();
            params.push(("page", page_owned.as_str()));
        }
        if let Some(per_page) = self.per_page {
            per_page_owned = per_page.to_string();
            params.push(("per_page", per_page_owned.as_str()));
        }
        if let Some(format) = self.format.as_deref() {
            params.push(("format", format));
        }
        if let Some(label) = self.label.as_deref() {
            params.push(("label", label));
        }
        if let Some(released) = self.released {
            released_owned = released.to_string();
            params.push(("released", &released_owned));
        }
        if let Some(country) = self.country.as_deref() {
            params.push(("country", country));
        }
        if let Some(sort) = self.sort.as_ref().map(AsRef::as_ref) {
            params.push(("sort", sort));
        }
        if let Some(sort_order) = self.sort_order.as_ref().map(AsRef::as_ref) {
            params.push(("sort_order", sort_order));
        }
        if let Some(personal_access_token) = personal_access_token {
            params.push(("token", personal_access_token));
        }

        build_params(params.as_slice())
    }
}

impl Endpoint<'_> for MasterVersions {
    type Parameters = MasterVersionsParams;
    type ReturnType = crate::data_types::MasterVersions;

    fn build_url(base: &Url, params: Self::Parameters) -> Result<Url, Error> {
        base.join(&format!(
            "/masters/{0}/versions{1}",
            params.master_id,
            params.build(None)
        ))
        .map_err(|_| Error::UrlError)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Auth, Client};

    use super::*;

    const ID: isize = 3175560;

    #[test]
    fn basic() {
        let _data = Client::builder()
            .build()
            .unwrap()
            .get::<MasterVersions>(MasterVersionsParams {
                master_id: ID,
                ..Default::default()
            })
            .unwrap();
    }

    #[test]
    fn with_auth() {
        let pat = std::env::var("discogs-pat")
            .expect("expected personal access token in env var `discogs-pat`");
        let _data = Client::builder()
            .auth(Auth::Token(pat))
            .build()
            .unwrap()
            .get::<MasterVersions>(MasterVersionsParams {
                master_id: ID,
                page: Some(2),
                per_page: Some(2),
                ..Default::default()
            })
            .unwrap();
    }
}
