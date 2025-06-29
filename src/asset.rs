use std::{
    borrow::Borrow,
    fmt::{Debug, Display, Formatter},
    str::FromStr,
};

#[derive(Clone, Default, PartialEq, Eq)]
pub enum Asset {
    Sol,
    #[default]
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
    fn test_asset() -> anyhow::Result<()> {
        // Test successful parsing
        let a = Asset::from_str("SOL")?;
        assert_eq!(a, SOL);

        let a = Asset::from_str("SOL_TEST")?;
        assert_eq!(a, SOL_TEST);

        // Test case insensitive parsing
        let a = Asset::from_str("sol")?;
        assert_eq!(a, SOL);

        let a = Asset::from_str("sol_test")?;
        assert_eq!(a, SOL_TEST);

        // Test mixed case parsing
        let a = Asset::from_str("SoL")?;
        assert_eq!(a, SOL);

        let a = Asset::from_str("Sol_Test")?;
        assert_eq!(a, SOL_TEST);

        // Test error conditions for from_str
        let result = Asset::from_str("INVALID");
        assert!(result.is_err());

        let result = Asset::from_str("BTC");
        assert!(result.is_err());

        let result = Asset::from_str("");
        assert!(result.is_err());

        // Test From<Asset> for String implementation
        let sol_string: String = SOL.into();
        assert_eq!(sol_string, "SOL");

        let sol_test_string: String = SOL_TEST.into();
        assert_eq!(sol_test_string, "SOL_TEST");

        // Test string representations
        assert_eq!(SOL.to_string(), "SOL");
        assert_eq!(SOL_TEST.to_string(), "SOL_TEST");
        assert_eq!(SOL_TEST, Asset::default());

        // Test Debug formatting
        assert_eq!(format!("{SOL:?}"), "SOL");
        assert_eq!(format!("{SOL_TEST:?}"), "SOL_TEST");

        // Test AsRef<str>
        assert_eq!(SOL.as_ref(), "SOL");
        assert_eq!(SOL_TEST.as_ref(), "SOL_TEST");

        // Test Borrow<str>
        let sol_borrowed: &str = SOL.borrow();
        assert_eq!(sol_borrowed, "SOL");

        let sol_test_borrowed: &str = SOL_TEST.borrow();
        assert_eq!(sol_test_borrowed, "SOL_TEST");

        Ok(())
    }
}
