use super::Endpoint;

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
        let _page: String;
        let _per_page: String;
        let _released: String;

        if let Some(page) = self.page {
            _page = page.to_string();
            params.push(("page", _page.as_str()));
        }
        if let Some(per_page) = self.per_page {
            _per_page = per_page.to_string();
            params.push(("per_page", _per_page.as_str()));
        }
        if let Some(format) = self.format.as_deref() {
            params.push(("format", format));
        }
        if let Some(label) = self.label.as_deref() {
            params.push(("label", label));
        }
        if let Some(released) = self.released {
            _released = released.to_string();
            params.push(("released", &_released));
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

        crate::build_params(params.as_slice())
    }
}

impl Endpoint<'_> for MasterVersions {
    type Parameters = MasterVersionsParams;
    type ReturnType = crate::data_types::MasterVersions;

    fn get_endpoint(params: Self::Parameters) -> String {
        format!(
            "/masters/{0}/versions{1}",
            params.master_id,
            params.build(None)
        )
    }

    fn get_endpoint_with_auth(params: Self::Parameters, personal_access_token: &str) -> String {
        format!(
            "/masters/{0}/versions{1}",
            params.master_id,
            params.build(Some(personal_access_token))
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ID: isize = 3175560;

    #[test]
    fn basic() {
        let _data = crate::get::<MasterVersions>(MasterVersionsParams {
            master_id: ID,
            ..Default::default()
        })
        .unwrap();
    }

    #[test]
    fn with_auth() {
        let pat = std::env::var("discogs-pat")
            .expect("expected personal access token in env var `discogs-pat`");
        let _data = crate::get_with_auth::<MasterVersions>(
            MasterVersionsParams {
                master_id: ID,
                page: Some(2),
                per_page: Some(2),
                ..Default::default()
            },
            &pat,
        )
        .unwrap();
    }
}
