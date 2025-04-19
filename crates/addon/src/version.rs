use std::fmt;
use std::fmt::{Debug, Display};

use serde::{Deserialize, Serialize};

/// A version used in Addons that is either a Vector [a, b, c] or SemVer String.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum AddonSemanticVersion {
    Vector([u32; 3]),
    SemVer(semver::Version),
}

impl Display for AddonSemanticVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AddonSemanticVersion::Vector(v) => write!(f, "{}.{}.{}", v[0], v[1], v[2]),
            AddonSemanticVersion::SemVer(v) => write!(f, "{}", v),
        }
    }
}