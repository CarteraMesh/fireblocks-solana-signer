use std::{
    borrow::Borrow,
    fmt::{Debug, Display, Formatter},
    str::FromStr,
};

#[derive(Clone, PartialEq, Eq)]
pub enum Asset {
    Sol,
    SolTest,
}

impl Debug for Asset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl AsRef<str> for Asset {
    #[allow(clippy::match_same_arms)]
    fn as_ref(&self) -> &str {
        match self {
            Self::Sol => "SOL",
            Self::SolTest => "SOL_TEST",
        }
    }
}

impl Borrow<str> for Asset {
    fn borrow(&self) -> &str {
        self.as_ref()
    }
}

impl Display for Asset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl From<Asset> for String {
    fn from(value: Asset) -> Self {
        format!("{value}")
    }
}

/// Convert a String to an [`Asset`]
impl FromStr for Asset {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "SOL" => Ok(Asset::Sol),
            "SOL_TEST" => Ok(Asset::SolTest),
            _ => Err(crate::Error::UnknownAsset(String::from(s))),
        }
    }
}

pub const SOL: Asset = Asset::Sol;
pub const SOL_TEST: Asset = Asset::SolTest;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asset_from_string() -> anyhow::Result<()> {
        let a = Asset::from_str("SOL")?;
        assert_eq!(a, SOL);

        let a = Asset::from_str("SOL_TEST")?;
        assert_eq!(a, SOL_TEST);

        assert_eq!(SOL.to_string(), "SOL");
        assert_eq!(SOL_TEST.to_string(), "SOL_TEST");
        Ok(())
    }
}
