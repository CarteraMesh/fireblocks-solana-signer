use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemMessageInfo {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// A response from Fireblocks that communicates a message about the health
    /// of the process being performed. If this object is returned with data,
    /// you should expect potential delays or incomplete transaction statuses.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl SystemMessageInfo {
    pub fn new() -> SystemMessageInfo {
        SystemMessageInfo {
            r#type: None,
            message: None,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "WARN")]
    Warn,
    #[serde(rename = "BLOCK")]
    Block,
}

impl Default for Type {
    fn default() -> Type {
        Self::Warn
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_sys_message() {
        assert_eq!(Type::default(), Type::Warn);
        let sys = SystemMessageInfo::new();
        assert_eq!(None, sys.r#type);
        assert_eq!(None, sys.message);
    }
}
