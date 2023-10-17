pub enum Params<'a> {
    Compiled(String),
    Pairs(Vec<(&'a str, &'a str)>),
}

impl<'a> Params<'a> {
    #[must_use]
    pub fn build(self) -> String {
        self.into()
    }
}

impl From<Params<'_>> for String {
    fn from(value: Params) -> String {
        match value {
            Params::Compiled(compiled) => compiled,
            Params::Pairs(pairs) => build_params(&pairs),
        }
    }
}

pub(crate) fn build_params(params: &[(&str, &str)]) -> String {
    if params.is_empty() {
        return String::new();
    }

    let len = params
        .iter()
        .fold(0, |acc, p| acc + 2 + p.0.len() + p.1.len());
    let mut out = String::with_capacity(len);

    let mut params_iter = params.iter();
    let param = params_iter.next().unwrap(); // fine to unwrap because already checked for empty
    out.push('?');
    out.push_str(param.0);
    out.push('=');
    out.push_str(param.1);

    for param in params_iter {
        out.push('&');
        out.push_str(param.0);
        out.push('=');
        out.push_str(param.1);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_params_raw() {
        let pairs = [("master_id", "3175560"), ("page", "1"), ("per_page", "25")];
        let built = build_params(&pairs);
        assert_eq!(&built, "?master_id=3175560&page=1&per_page=25");
    }

    #[test]
    fn test_build_params() {
        let params =
            Params::Pairs([("master_id", "3175560"), ("page", "1"), ("per_page", "25")].to_vec());
        let built = params.build();
        assert_eq!(&built, "?master_id=3175560&page=1&per_page=25");
    }
}
