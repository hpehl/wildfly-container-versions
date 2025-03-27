//! A library for managing WildFly container versions deployed at
//! https://hub.docker.com/r/jboss/wildfly and https://quay.io/repository/wildfly/wildfly.
//!
//! The library contains a struct describing the WildFly container versions
//! and functions to parse version enumerations and ranges.

use anyhow::{bail, Result};
use lazy_static::lazy_static;
use regex::Regex;
use semver::Version;
use std::cmp::Ordering;
use std::collections::BTreeMap;

pub static DEVELOPMENT_VERSION: &str = "dev";
pub static DEVELOPMENT_TAG: &str = "development";

lazy_static! {
    static ref WILDFLY_DEV: WildFlyContainer = WildFlyContainer::new(Version::new(0, 0, 0), Version::new(0, 0, 0), "", "", vec![]);

    /// Static map with versions from 10 to 35
    pub static ref VERSIONS: BTreeMap<u16, WildFlyContainer> = {
        let mut m = BTreeMap::new();
        // @formatter:off
        m.insert(identifier(10, 0), WildFlyContainer::new(Version::new(10, 0, 0), Version::new(2, 0, 10), "Final", "docker.io/jboss/wildfly", vec![]));
        m.insert(identifier(10, 1), WildFlyContainer::new(Version::new(10, 1, 0), Version::new(2, 2, 0), "Final", "docker.io/jboss/wildfly", vec![]));
        m.insert(identifier(11, 0), WildFlyContainer::new(Version::new(11, 0, 0), Version::new(3, 0, 8), "Final", "docker.io/jboss/wildfly", vec![]));
        m.insert(identifier(12, 0), WildFlyContainer::new(Version::new(12, 0, 0), Version::new(4, 0, 0), "Final", "docker.io/jboss/wildfly", vec![]));
        m.insert(identifier(13, 0), WildFlyContainer::new(Version::new(13, 0, 0), Version::new(5, 0, 0), "Final", "docker.io/jboss/wildfly", vec![]));
        m.insert(identifier(14, 0), WildFlyContainer::new(Version::new(14, 0, 1), Version::new(6, 0, 2), "Final", "docker.io/jboss/wildfly", vec![]));
        m.insert(identifier(15, 0), WildFlyContainer::new(Version::new(15, 0, 1), Version::new(7, 0, 0), "Final", "docker.io/jboss/wildfly", vec![]));
        m.insert(identifier(16, 0), WildFlyContainer::new(Version::new(16, 0, 0), Version::new(8, 0, 0), "Final", "docker.io/jboss/wildfly", vec![]));
        m.insert(identifier(17, 0), WildFlyContainer::new(Version::new(17, 0, 1), Version::new(9, 0, 2), "Final", "docker.io/jboss/wildfly", vec![]));
        m.insert(identifier(18, 0), WildFlyContainer::new(Version::new(18, 0, 1), Version::new(10, 0, 3), "Final", "docker.io/jboss/wildfly", vec![]));
        m.insert(identifier(19, 0), WildFlyContainer::new(Version::new(19, 0, 0), Version::new(11, 0, 0), "Final", "docker.io/jboss/wildfly", vec![]));
        m.insert(identifier(19, 1), WildFlyContainer::new(Version::new(19, 1, 0), Version::new(11, 1, 1), "Final", "docker.io/jboss/wildfly", vec![]));
        m.insert(identifier(20, 0), WildFlyContainer::new(Version::new(20, 0, 1), Version::new(12, 0, 3), "Final", "docker.io/jboss/wildfly", vec![]));
        m.insert(identifier(21, 0), WildFlyContainer::new(Version::new(21, 0, 2), Version::new(13, 0, 3), "Final", "docker.io/jboss/wildfly", vec![]));
        m.insert(identifier(22, 0), WildFlyContainer::new(Version::new(22, 0, 1), Version::new(14, 0, 1), "Final", "docker.io/jboss/wildfly", vec![]));
        m.insert(identifier(23, 0), WildFlyContainer::new(Version::new(23, 0, 2), Version::new(15, 0, 1), "Final", "quay.io/wildfly/wildfly", vec![]));
        m.insert(identifier(24, 0), WildFlyContainer::new(Version::new(24, 0, 1), Version::new(16, 0, 1), "Final", "quay.io/wildfly/wildfly", vec![]));
        m.insert(identifier(25, 0), WildFlyContainer::new(Version::new(25, 0, 1), Version::new(17, 0, 3), "Final", "quay.io/wildfly/wildfly", vec![]));
        m.insert(identifier(26, 0), WildFlyContainer::new(Version::new(26, 0, 1), Version::new(18, 0, 4), "Final", "quay.io/wildfly/wildfly", vec![]));
        m.insert(identifier(26, 1), WildFlyContainer::new(Version::new(26, 1, 3), Version::new(18, 1, 2), "Final-jdk17", "quay.io/wildfly/wildfly", vec!["linux/amd64", "linux/arm64"]));
        m.insert(identifier(27, 0), WildFlyContainer::new(Version::new(27, 0, 1), Version::new(19, 0, 1), "Final-jdk19", "quay.io/wildfly/wildfly", vec!["linux/amd64", "linux/arm64"]));
        m.insert(identifier(28, 0), WildFlyContainer::new(Version::new(28, 0, 1), Version::new(20, 0, 2), "Final-jdk20", "quay.io/wildfly/wildfly", vec!["linux/amd64", "linux/arm64"]));
        m.insert(identifier(29, 0), WildFlyContainer::new(Version::new(29, 0, 1), Version::new(21, 1, 1), "Final-jdk20", "quay.io/wildfly/wildfly", vec!["linux/amd64", "linux/arm64"]));
        m.insert(identifier(30, 0), WildFlyContainer::new(Version::new(30, 0, 1), Version::new(22, 0, 2), "Final-jdk20", "quay.io/wildfly/wildfly", vec!["linux/amd64", "linux/arm64"]));
        m.insert(identifier(31, 0), WildFlyContainer::new(Version::new(31, 0, 1), Version::new(23, 0, 3), "Final-jdk20", "quay.io/wildfly/wildfly", vec!["linux/amd64", "linux/arm64"]));
        m.insert(identifier(32, 0), WildFlyContainer::new(Version::new(32, 0, 1), Version::new(24, 0, 1), "Final-jdk21", "quay.io/wildfly/wildfly", vec!["linux/amd64", "linux/arm64", "linux/s390x"]));
        m.insert(identifier(33, 0), WildFlyContainer::new(Version::new(33, 0, 2), Version::new(25, 0, 2), "Final-jdk21", "quay.io/wildfly/wildfly", vec!["linux/amd64", "linux/arm64", "linux/s390x", "linux/ppc64le"]));
        m.insert(identifier(34, 0), WildFlyContainer::new(Version::new(34, 0, 1), Version::new(26, 0, 1), "Final-jdk21", "quay.io/wildfly/wildfly", vec!["linux/amd64", "linux/arm64", "linux/s390x", "linux/ppc64le"]));
        m.insert(identifier(35, 0), WildFlyContainer::new(Version::new(35, 0, 1), Version::new(27, 0, 1), "Final-jdk21", "quay.io/wildfly/wildfly", vec!["linux/amd64", "linux/arm64", "linux/s390x", "linux/ppc64le"]));
        // @formatter:on
        m
    };
}

/// Describes a WildFly container version
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct WildFlyContainer {
    port_offset: u16,

    /// A unique identifier `<major><minor>`
    pub identifier: u16,

    /// The semantic version
    pub version: Version,

    /// The short version as "`<major>.<minor>`"
    pub short_version: String,

    /// The WildFly core version
    pub core_version: Version,

    /// A suffix like "Final-jdk21"
    pub suffix: String,

    /// The container repository
    pub repository: String,

    /// The supported platforms
    pub platforms: Vec<String>,
}

impl WildFlyContainer {
    pub fn new(
        version: Version,
        core_version: Version,
        suffix: &str,
        source_repository: &str,
        platforms: Vec<&str>,
    ) -> Self {
        Self {
            identifier: identifier(version.major as u16, version.minor as u16),
            port_offset: (version.major * 10 + version.minor) as u16,
            short_version: format!("{}.{}", version.major, version.minor),
            version,
            core_version,
            suffix: suffix.to_string(),
            repository: source_repository.to_string(),
            platforms: platforms.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn image_name(&self) -> String {
        if self.is_dev() {
            "https://github.com/wildfly/wildfly.git".to_string()
        } else {
            format!("{}:{}.{}", self.repository, self.version, self.suffix)
        }
    }

    pub fn is_dev(&self) -> bool {
        self.identifier == 0
    }

    pub fn http_port(&self) -> u16 {
        8000 + self.port_offset
    }

    pub fn management_port(&self) -> u16 {
        9000 + self.port_offset
    }

    /// Turns an enumeration of WildFly versions like "10,23..26,28,34,dev"
    /// into an array of [WildFlyContainer]s.
    pub fn enumeration(enumeration: &str) -> Result<Vec<WildFlyContainer>> {
        let mut result: Vec<WildFlyContainer> = vec![];
        let mut errors: Vec<String> = vec![];
        enumeration.split(',').for_each(|segment| {
            if segment.contains("..") {
                match Self::range(segment) {
                    Ok(interval) => result.extend(interval),
                    Err(e) => errors.push(e.to_string()),
                }
            } else {
                match Self::lookup_by_short_version(segment) {
                    Ok(w) => result.push(w),
                    Err(e) => errors.push(e.to_string()),
                }
            }
        });
        if errors.is_empty() {
            result.sort_by(|a, b| a.identifier.cmp(&b.identifier));
            result.dedup();
            Ok(result)
        } else if errors.len() > 1 {
            bail!(format!("\n{}", errors.join("\n")))
        } else {
            bail!(errors.first().unwrap().to_string())
        }
    }

    /// Turns a range of WildFly versions like "20.1..29" or "25.." or "..26.1" or "..",
    /// but not "..dev" or "dev.." into an array of [WildFlyContainer]s.
    pub fn range(range: &str) -> Result<Vec<WildFlyContainer>> {
        if range.contains("..") {
            let parts = range.split("..").collect::<Vec<&str>>();
            if parts.len() == 2 {
                if !(parts[0] == DEVELOPMENT_VERSION || parts[1] == DEVELOPMENT_TAG) {
                    let from = match parts[0] {
                        "" => Some(VERSIONS.first_key_value().unwrap().1.clone()),
                        _ => Self::lookup_by_short_version(parts[0]).ok(),
                    };
                    let to = match parts[1] {
                        "" => Some(VERSIONS.last_key_value().unwrap().1.clone()),
                        _ => Self::lookup_by_short_version(parts[1]).ok(),
                    };
                    match (from, to) {
                        (Some(f), Some(t)) => match f.identifier.cmp(&t.identifier) {
                            Ordering::Equal => Ok(vec![f]),
                            Ordering::Less => Ok(VERSIONS
                                .range(f.identifier..=t.identifier)
                                .map(|(_, w)| w.clone())
                                .collect()),
                            Ordering::Greater => {
                                bail!(format!("{} is greater than {}", f.identifier, t.identifier))
                            }
                        },
                        (None, _) => {
                            bail!(format!("from '{}' is not valid", parts[0]))
                        }
                        (_, None) => {
                            bail!(format!("to '{}' is not valid", parts[1]))
                        }
                    }
                } else {
                    bail!(format!("'dev' is not allowed in '{}'", range))
                }
            } else {
                bail!(format!("'{}'", range))
            }
        } else {
            bail!(format!("'{}'", range))
        }
    }

    /// Looks up a single [WildFlyContainer] version like "dev" or "22" or "26.1".
    pub fn lookup_by_short_version(short_version: &str) -> Result<WildFlyContainer> {
        if short_version == "dev" {
            Ok(WILDFLY_DEV.clone())
        } else {
            let re = Regex::new(r"^(?<major>[0-9]{2})\.?(?<minor>[0-9])?$")?;
            match re.captures(short_version) {
                Some(c) => {
                    let major: u16 = c["major"].parse()?;
                    let minor: u16 = c
                        .name("minor")
                        .map_or(0, |m| m.as_str().parse().unwrap_or(0));
                    match VERSIONS.get(&identifier(major, minor)) {
                        Some(wildfly) => Ok(wildfly.clone()),
                        None => bail!(format!("unknown version {}", short_version)),
                    }
                }
                None => bail!(format!("invalid version '{}'", short_version)),
            }
        }
    }

    pub fn lookup_by_identifier(identifier: u16) -> Result<WildFlyContainer> {
        match VERSIONS.get(&identifier) {
            Some(wildfly) => Ok(wildfly.clone()),
            None => bail!(format!("unknown version {}", identifier)),
        }
    }
}

impl Ord for WildFlyContainer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.identifier.cmp(&other.identifier)
    }
}

impl PartialOrd for WildFlyContainer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn identifier(major: u16, minor: u16) -> u16 {
    major * 10 + minor
}

// ------------------------------------------------------ tests

#[cfg(test)]
mod wildfly_tests {
    use crate::{WildFlyContainer, VERSIONS};

    #[test]
    fn invalid_lookup() {
        assert!(WildFlyContainer::lookup_by_short_version("").is_err());
        assert!(WildFlyContainer::lookup_by_short_version("  ").is_err());
        assert!(WildFlyContainer::lookup_by_short_version("foo").is_err());
        assert!(WildFlyContainer::lookup_by_short_version(".").is_err());
        assert!(WildFlyContainer::lookup_by_short_version("a.b").is_err());
        assert!(WildFlyContainer::lookup_by_short_version("0").is_err());
        assert!(WildFlyContainer::lookup_by_short_version("9").is_err());
        assert!(WildFlyContainer::lookup_by_short_version("99").is_err());
        assert!(WildFlyContainer::lookup_by_short_version("123").is_err());
        assert!(WildFlyContainer::lookup_by_short_version("1.2.3").is_err());
        assert!(WildFlyContainer::lookup_by_short_version("0.").is_err());
        assert!(WildFlyContainer::lookup_by_short_version(".0").is_err());
        assert!(WildFlyContainer::lookup_by_short_version("9.").is_err());
        assert!(WildFlyContainer::lookup_by_short_version(".9").is_err());
        assert!(WildFlyContainer::lookup_by_short_version(".123").is_err());
        assert!(WildFlyContainer::lookup_by_short_version("123.").is_err());
        assert!(WildFlyContainer::lookup_by_short_version("1.1").is_err());
        assert!(WildFlyContainer::lookup_by_short_version("10.10").is_err());
        assert!(WildFlyContainer::lookup_by_short_version("99").is_err());
    }

    #[test]
    fn lookup() {
        assert!(WildFlyContainer::lookup_by_short_version("dev").is_ok());
        assert!(WildFlyContainer::lookup_by_short_version("10").is_ok());
        assert!(WildFlyContainer::lookup_by_short_version("25").is_ok());
        assert!(WildFlyContainer::lookup_by_short_version("25.0").is_ok());
        assert!(WildFlyContainer::lookup_by_short_version("26.1").is_ok());
        assert!(WildFlyContainer::lookup_by_short_version("34").is_ok());
    }

    #[test]
    fn invalid_range() {
        assert!(WildFlyContainer::range("").is_err());
        assert!(WildFlyContainer::range("  ").is_err());
        assert!(WildFlyContainer::range(".").is_err());
        assert!(WildFlyContainer::range("...").is_err());
        assert!(WildFlyContainer::range("foo").is_err());
        assert!(WildFlyContainer::range("dev").is_err());
        assert!(WildFlyContainer::range("..dev").is_err());
        assert!(WildFlyContainer::range("dev..").is_err());
        assert!(WildFlyContainer::range("dev..dev").is_err());
        assert!(WildFlyContainer::range("10..dev").is_err());
        assert!(WildFlyContainer::range("dev..20").is_err());
        assert!(WildFlyContainer::range("20..10").is_err());
        assert!(WildFlyContainer::range("10..20..30").is_err());
    }

    #[test]
    fn range_from_to() {
        if let Ok(interval) = WildFlyContainer::range("20..20") {
            assert_eq!(1, interval.len());
            assert_eq!(200, interval[0].identifier);
        } else {
            panic!("Failed");
        }
        if let Ok(interval) = WildFlyContainer::range("10..10.1") {
            assert_eq!(2, interval.len());
            assert_eq!(100, interval[0].identifier);
            assert_eq!(101, interval.last().unwrap().identifier)
        } else {
            panic!("Failed");
        }
        if let Ok(interval) = WildFlyContainer::range("19.1..20") {
            assert_eq!(2, interval.len());
            assert_eq!(191, interval[0].identifier);
            assert_eq!(200, interval.last().unwrap().identifier)
        } else {
            panic!("Failed");
        }
        if let Ok(interval) = WildFlyContainer::range("19.1..26.1") {
            assert_eq!(9, interval.len());
            assert_eq!(191, interval[0].identifier);
            assert_eq!(261, interval.last().unwrap().identifier)
        } else {
            panic!("Failed");
        }
        if let Ok(interval) = WildFlyContainer::range("20..30") {
            assert_eq!(12, interval.len());
            assert_eq!(200, interval[0].identifier);
            assert_eq!(300, interval.last().unwrap().identifier)
        } else {
            panic!("Failed");
        }
    }

    #[test]
    fn range_from() {
        if let Ok(interval) = WildFlyContainer::range("26.1..") {
            assert_eq!(10, interval.len());
            assert_eq!(261, interval[0].identifier);
            assert_eq!(350, interval.last().unwrap().identifier)
        } else {
            panic!("Failed");
        }
        if let Ok(interval) = WildFlyContainer::range("30..") {
            assert_eq!(6, interval.len());
            assert_eq!(300, interval[0].identifier);
            assert_eq!(350, interval.last().unwrap().identifier)
        } else {
            panic!("Failed");
        }
        let last = VERSIONS.last_key_value().unwrap().1;
        if let Ok(interval) = WildFlyContainer::range(format!("{}..", last.short_version).as_str())
        {
            assert_eq!(1, interval.len());
            assert_eq!(last.identifier, interval[0].identifier);
        } else {
            panic!("Failed");
        }
    }

    #[test]
    fn range_to() {
        if let Ok(interval) = WildFlyContainer::range("..10") {
            assert_eq!(1, interval.len());
            assert_eq!(100, interval[0].identifier);
        } else {
            panic!("Failed");
        }
        if let Ok(interval) = WildFlyContainer::range("..10.1") {
            assert_eq!(2, interval.len());
            assert_eq!(100, interval[0].identifier);
            assert_eq!(101, interval.last().unwrap().identifier)
        } else {
            panic!("Failed");
        }
        if let Ok(interval) = WildFlyContainer::range("..20") {
            assert_eq!(13, interval.len());
            assert_eq!(100, interval[0].identifier);
            assert_eq!(200, interval.last().unwrap().identifier)
        } else {
            panic!("Failed");
        }
    }

    #[test]
    fn range_all() {
        if let Ok(interval) = WildFlyContainer::range("..") {
            assert_eq!(VERSIONS.len(), interval.len());
            assert_eq!(
                *(VERSIONS.first_key_value().unwrap().0),
                interval[0].identifier
            );
            assert_eq!(
                *(VERSIONS.last_key_value().unwrap().0),
                interval.last().unwrap().identifier
            );
        } else {
            panic!("Failed");
        }
    }

    #[test]
    fn invalid_enumeration() {
        assert!(WildFlyContainer::enumeration("").is_err());
        assert!(WildFlyContainer::enumeration("  ").is_err());
        assert!(WildFlyContainer::enumeration(",").is_err());
        assert!(WildFlyContainer::enumeration("foo").is_err());
    }

    #[test]
    fn enumeration() {
        if let Ok(range) = WildFlyContainer::enumeration("23..26.1,dev,28,10,25,34") {
            assert_eq!(9, range.len());
            assert!(range[0].is_dev());
            assert_eq!(100, range[1].identifier);
            assert_eq!(230, range[2].identifier);
            assert_eq!(240, range[3].identifier);
            assert_eq!(250, range[4].identifier);
            assert_eq!(260, range[5].identifier);
            assert_eq!(261, range[6].identifier);
            assert_eq!(280, range[7].identifier);
            assert_eq!(340, range[8].identifier);
        } else {
            panic!("Failed");
        }
    }
}
