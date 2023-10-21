#[derive(Debug, Clone)]
pub enum SortOrder {
    Asc,
    Desc,
}

impl AsRef<str> for SortOrder {
    fn as_ref(&self) -> &str {
        match self {
            Self::Asc => "asc",
            Self::Desc => "desc",
        }
    }
}
