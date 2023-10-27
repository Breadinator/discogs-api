use super::Endpoint;
use crate::Error;
use reqwest::Url;

pub struct Label;

impl Endpoint<'_> for Label {
    type Parameters = isize;
    type ReturnType = crate::data_types::Label;

    fn build_url(base: &Url, params: Self::Parameters) -> Result<Url, Error> {
        base.join(&format!("/labels/{params}"))
            .map_err(|_| Error::UrlError)
    }
}

#[cfg(test)]
mod tests {
    use super::Label;
    use crate::Client;

    #[test]
    fn basic() {
        let id = 478174;
        let data = Client::default().get::<Label>(id).unwrap();
        dbg!(data);
    }
}
