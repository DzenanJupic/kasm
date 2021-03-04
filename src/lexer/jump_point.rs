use std::str::FromStr;

#[derive(Clone, Debug, derive_more::Display)]
pub struct JumpPoint(pub String);

impl AsRef<str> for JumpPoint {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl FromStr for JumpPoint {
    type Err = ();

    fn from_str(s: &str) -> crate::Result<Self, Self::Err> {
        if !(s.starts_with('.') && !s.ends_with(':') && s.chars().count() > 1) {
            return Err(());
        } else {
            s
                .strip_prefix('.')
                .map(str::to_owned)
                .map(Self)
                .ok_or(())
        }
    }
}
