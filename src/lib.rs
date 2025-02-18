use anyhow::{bail, Result};
use lazy_static::lazy_static;
use regex::Regex;
use semver::Version;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::{Display, Formatter};

pub static DEVELOPMENT_VERSION: &str = "dev";
pub static DEVELOPMENT_TAG: &str = "development";

lazy_static! {
    static ref WILDFLY_DEV: WildFly = WildFly::new(ShortVersion::dev(), Version::new(0, 0, 0), "", "", vec![]);
    pub static ref VERSIONS: BTreeMap<ShortVersion, WildFly> = {
        let mut m = BTreeMap::new();
        // @formatter:off
        m.insert(ShortVersion::new(10, 0), WildFly::new(ShortVersion::new(10, 0), Version::new(10, 0, 0), "Final", "docker.io/jboss/wildfly", vec![]));
        m.insert(ShortVersion::new(10, 1), WildFly::new(ShortVersion::new(10, 1), Version::new(10, 1, 0), "Final", "docker.io/jboss/wildfly", vec![]));
        m.insert(ShortVersion::new(11, 0), WildFly::new(ShortVersion::new(11, 0), Version::new(11, 0, 0), "Final", "docker.io/jboss/wildfly", vec![]));
        m.insert(ShortVersion::new(12, 0), WildFly::new(ShortVersion::new(12, 0), Version::new(12, 0, 0), "Final", "docker.io/jboss/wildfly", vec![]));
        m.insert(ShortVersion::new(13, 0), WildFly::new(ShortVersion::new(13, 0), Version::new(13, 0, 0), "Final", "docker.io/jboss/wildfly", vec![]));
        m.insert(ShortVersion::new(14, 0), WildFly::new(ShortVersion::new(14, 0), Version::new(14, 0, 1), "Final", "docker.io/jboss/wildfly", vec![]));
        m.insert(ShortVersion::new(15, 0), WildFly::new(ShortVersion::new(15, 0), Version::new(15, 0, 1), "Final", "docker.io/jboss/wildfly", vec![]));
        m.insert(ShortVersion::new(16, 0), WildFly::new(ShortVersion::new(16, 0), Version::new(16, 0, 0), "Final", "docker.io/jboss/wildfly", vec![]));
        m.insert(ShortVersion::new(17, 0), WildFly::new(ShortVersion::new(17, 0), Version::new(17, 0, 1), "Final", "docker.io/jboss/wildfly", vec![]));
        m.insert(ShortVersion::new(18, 0), WildFly::new(ShortVersion::new(18, 0), Version::new(18, 0, 1), "Final", "docker.io/jboss/wildfly", vec![]));
        m.insert(ShortVersion::new(19, 0), WildFly::new(ShortVersion::new(19, 0), Version::new(19, 0, 0), "Final", "docker.io/jboss/wildfly", vec![]));
        m.insert(ShortVersion::new(19, 1), WildFly::new(ShortVersion::new(19, 1), Version::new(19, 1, 0), "Final", "docker.io/jboss/wildfly", vec![]));
        m.insert(ShortVersion::new(20, 0), WildFly::new(ShortVersion::new(20, 0), Version::new(20, 0, 1), "Final", "docker.io/jboss/wildfly", vec![]));
        m.insert(ShortVersion::new(21, 0), WildFly::new(ShortVersion::new(21, 0), Version::new(21, 0, 2), "Final", "docker.io/jboss/wildfly", vec![]));
        m.insert(ShortVersion::new(22, 0), WildFly::new(ShortVersion::new(22, 0), Version::new(22, 0, 1), "Final", "docker.io/jboss/wildfly", vec![]));
        m.insert(ShortVersion::new(23, 0), WildFly::new(ShortVersion::new(23, 0), Version::new(23, 0, 2), "Final", "quay.io/wildfly/wildfly", vec![]));
        m.insert(ShortVersion::new(24, 0), WildFly::new(ShortVersion::new(24, 0), Version::new(24, 0, 0), "Final", "quay.io/wildfly/wildfly", vec![]));
        m.insert(ShortVersion::new(25, 0), WildFly::new(ShortVersion::new(25, 0), Version::new(25, 0, 1), "Final", "quay.io/wildfly/wildfly", vec![]));
        m.insert(ShortVersion::new(26, 0), WildFly::new(ShortVersion::new(26, 0), Version::new(26, 0, 1), "Final", "quay.io/wildfly/wildfly", vec![]));
        m.insert(ShortVersion::new(26, 1), WildFly::new(ShortVersion::new(26, 1), Version::new(26, 1, 3), "Final-jdk17", "quay.io/wildfly/wildfly", vec!["linux/amd64", "linux/arm64"]));
        m.insert(ShortVersion::new(27, 0), WildFly::new(ShortVersion::new(27, 0), Version::new(27, 0, 1), "Final-jdk19", "quay.io/wildfly/wildfly", vec!["linux/amd64", "linux/arm64"]));
        m.insert(ShortVersion::new(28, 0), WildFly::new(ShortVersion::new(28, 0), Version::new(28, 0, 1), "Final-jdk20", "quay.io/wildfly/wildfly", vec!["linux/amd64", "linux/arm64"]));
        m.insert(ShortVersion::new(29, 0), WildFly::new(ShortVersion::new(29, 0), Version::new(29, 0, 1), "Final-jdk20", "quay.io/wildfly/wildfly", vec!["linux/amd64", "linux/arm64"]));
        m.insert(ShortVersion::new(30, 0), WildFly::new(ShortVersion::new(30, 0), Version::new(30, 0, 1), "Final-jdk20", "quay.io/wildfly/wildfly", vec!["linux/amd64", "linux/arm64"]));
        m.insert(ShortVersion::new(31, 0), WildFly::new(ShortVersion::new(31, 0), Version::new(31, 0, 1), "Final-jdk20", "quay.io/wildfly/wildfly", vec!["linux/amd64", "linux/arm64"]));
        m.insert(ShortVersion::new(32, 0), WildFly::new(ShortVersion::new(32, 0), Version::new(32, 0, 1), "Final-jdk21", "quay.io/wildfly/wildfly", vec!["linux/amd64", "linux/arm64", "linux/s390x"]));
        m.insert(ShortVersion::new(33, 0), WildFly::new(ShortVersion::new(33, 0), Version::new(33, 0, 2), "Final-jdk21", "quay.io/wildfly/wildfly", vec!["linux/amd64", "linux/arm64", "linux/s390x", "linux/ppc64le"]));
        m.insert(ShortVersion::new(34, 0), WildFly::new(ShortVersion::new(34, 0), Version::new(34, 0, 1), "Final-jdk21", "quay.io/wildfly/wildfly", vec!["linux/amd64", "linux/arm64", "linux/s390x", "linux/ppc64le"]));
        m.insert(ShortVersion::new(35, 0), WildFly::new(ShortVersion::new(35, 0), Version::new(35, 0, 1), "Final-jdk21", "quay.io/wildfly/wildfly", vec!["linux/amd64", "linux/arm64", "linux/s390x", "linux/ppc64le"]));
        // @formatter:on
        m
    };
}

// ------------------------------------------------------ short version

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct ShortVersion {
    pub major: u16,
    pub minor: u16,
    pub version: u16,
}

impl ShortVersion {
    pub fn new(major: u16, minor: u16) -> Self {
        Self {
            major,
            minor,
            version: major * 100 + minor * 10,
        }
    }

    pub fn port(&self) -> u16 {
        self.major * 10 + self.minor
    }

    fn dev() -> Self {
        Self {
            major: 0,
            minor: 0,
            version: 0,
        }
    }

    fn is_dev(&self) -> bool {
        self.version == 0
    }
}

impl Ord for ShortVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        self.version.cmp(&other.version)
    }
}

impl PartialOrd for ShortVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for ShortVersion {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.is_dev() {
            write!(f, "dev")
        } else if self.minor == 0 {
            write!(f, "{}", self.major)
        } else {
            write!(f, "{}.{}", self.major, self.minor)
        }
    }
}

// ------------------------------------------------------ wildfly

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct WildFly {
    pub short_version: ShortVersion,
    pub version: Version,
    pub suffix: String,
    pub repository: String,
    pub platforms: Vec<String>,
}

impl WildFly {
    pub fn new(
        short_version: ShortVersion,
        version: Version,
        suffix: &str,
        source_repository: &str,
        platforms: Vec<&str>,
    ) -> Self {
        Self {
            short_version,
            version,
            suffix: suffix.to_string(),
            repository: source_repository.to_string(),
            platforms: platforms.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn base_image(&self) -> String {
        if self.is_dev() {
            "https://github.com/wildfly/wildfly.git".to_string()
        } else {
            format!("{}:{}.{}", self.repository, self.version, self.suffix)
        }
    }

    pub fn is_dev(&self) -> bool {
        self.short_version.is_dev()
    }

    pub fn http_port(&self) -> u16 {
        8000 + self.short_version.port()
    }

    pub fn management_port(&self) -> u16 {
        9000 + self.short_version.port()
    }

    // "10,23..26,28,34,dev"
    pub fn enumeration(enumeration: &str) -> Result<Vec<WildFly>> {
        let mut result: Vec<WildFly> = vec![];
        let mut errors: Vec<String> = vec![];
        enumeration.split(',').for_each(|segment| {
            if segment.contains("..") {
                match Self::range(segment) {
                    Ok(interval) => result.extend(interval),
                    Err(e) => errors.push(e.to_string()),
                }
            } else {
                match Self::lookup(segment) {
                    Ok(w) => result.push(w),
                    Err(e) => errors.push(e.to_string()),
                }
            }
        });
        if errors.is_empty() {
            result.sort_by(|a, b| a.short_version.cmp(&b.short_version));
            result.dedup();
            Ok(result)
        } else if errors.len() > 1 {
            bail!(format!("\n{}", errors.join("\n")))
        } else {
            bail!(errors.first().unwrap().to_string())
        }
    }

    // "20.1..29" or "25.." or "..26.1" or "..", but not "..dev" or "dev.."
    pub fn range(range: &str) -> Result<Vec<WildFly>> {
        if range.contains("..") {
            let parts = range.split("..").collect::<Vec<&str>>();
            if parts.len() == 2 {
                if !(parts[0] == DEVELOPMENT_VERSION || parts[1] == DEVELOPMENT_TAG) {
                    let from = match parts[0] {
                        "" => Some(VERSIONS.first_key_value().unwrap().1.clone()),
                        _ => Self::lookup(parts[0]).ok(),
                    };
                    let to = match parts[1] {
                        "" => Some(VERSIONS.last_key_value().unwrap().1.clone()),
                        _ => Self::lookup(parts[1]).ok(),
                    };
                    match (from, to) {
                        (Some(f), Some(t)) => match f.short_version.cmp(&t.short_version) {
                            Ordering::Equal => Ok(vec![f]),
                            Ordering::Less => Ok(VERSIONS
                                .range(f.short_version..=t.short_version)
                                .map(|(_, w)| w.clone())
                                .collect()),
                            Ordering::Greater => bail!(format!(
                                "{} is greater than {}",
                                f.short_version, t.short_version
                            )),
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

    // "dev" or "22" or "26.1"
    pub fn lookup(version: &str) -> Result<WildFly> {
        if version == "dev" {
            Ok(WILDFLY_DEV.clone())
        } else {
            let re = Regex::new(r"^(?<major>[0-9]{2})\.?(?<minor>[0-9])?$")?;
            match re.captures(version) {
                Some(c) => {
                    let major: u16 = c["major"].parse()?;
                    let minor: u16 = c
                        .name("minor")
                        .map_or(0, |m| m.as_str().parse().unwrap_or(0));
                    match VERSIONS.get(&ShortVersion::new(major, minor)) {
                        Some(wildfly) => Ok(wildfly.clone()),
                        None => bail!(format!("unknown version {}", version)),
                    }
                }
                None => bail!(format!("invalid version '{}'", version)),
            }
        }
    }
}

impl Ord for WildFly {
    fn cmp(&self, other: &Self) -> Ordering {
        self.short_version.cmp(&other.short_version)
    }
}

impl PartialOrd for WildFly {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// ------------------------------------------------------ tests

#[cfg(test)]
mod wildfly_tests {
    use crate::{WildFly, VERSIONS};

    #[test]
    fn invalid_lookup() {
        assert!(WildFly::lookup("").is_err());
        assert!(WildFly::lookup("  ").is_err());
        assert!(WildFly::lookup("foo").is_err());
        assert!(WildFly::lookup(".").is_err());
        assert!(WildFly::lookup("a.b").is_err());
        assert!(WildFly::lookup("0").is_err());
        assert!(WildFly::lookup("9").is_err());
        assert!(WildFly::lookup("99").is_err());
        assert!(WildFly::lookup("123").is_err());
        assert!(WildFly::lookup("1.2.3").is_err());
        assert!(WildFly::lookup("0.").is_err());
        assert!(WildFly::lookup(".0").is_err());
        assert!(WildFly::lookup("9.").is_err());
        assert!(WildFly::lookup(".9").is_err());
        assert!(WildFly::lookup(".123").is_err());
        assert!(WildFly::lookup("123.").is_err());
        assert!(WildFly::lookup("1.1").is_err());
        assert!(WildFly::lookup("10.10").is_err());
        assert!(WildFly::lookup("99").is_err());
    }

    #[test]
    fn lookup() {
        assert!(WildFly::lookup("dev").is_ok());
        assert!(WildFly::lookup("10").is_ok());
        assert!(WildFly::lookup("25").is_ok());
        assert!(WildFly::lookup("25.0").is_ok());
        assert!(WildFly::lookup("26.1").is_ok());
        assert!(WildFly::lookup("34").is_ok());
    }

    #[test]
    fn invalid_range() {
        assert!(WildFly::range("").is_err());
        assert!(WildFly::range("  ").is_err());
        assert!(WildFly::range(".").is_err());
        assert!(WildFly::range("...").is_err());
        assert!(WildFly::range("foo").is_err());
        assert!(WildFly::range("dev").is_err());
        assert!(WildFly::range("..dev").is_err());
        assert!(WildFly::range("dev..").is_err());
        assert!(WildFly::range("dev..dev").is_err());
        assert!(WildFly::range("10..dev").is_err());
        assert!(WildFly::range("dev..20").is_err());
        assert!(WildFly::range("20..10").is_err());
        assert!(WildFly::range("10..20..30").is_err());
    }

    #[test]
    fn range_from_to() {
        if let Ok(interval) = WildFly::range("20..20") {
            assert_eq!(1, interval.len());
            assert_eq!(2000, interval[0].short_version.version);
        } else {
            panic!("Failed");
        }
        if let Ok(interval) = WildFly::range("10..10.1") {
            assert_eq!(2, interval.len());
            assert_eq!(1000, interval[0].short_version.version);
            assert_eq!(1010, interval.last().unwrap().short_version.version)
        } else {
            panic!("Failed");
        }
        if let Ok(interval) = WildFly::range("19.1..20") {
            assert_eq!(2, interval.len());
            assert_eq!(1910, interval[0].short_version.version);
            assert_eq!(2000, interval.last().unwrap().short_version.version)
        } else {
            panic!("Failed");
        }
        if let Ok(interval) = WildFly::range("19.1..26.1") {
            assert_eq!(9, interval.len());
            assert_eq!(1910, interval[0].short_version.version);
            assert_eq!(2610, interval.last().unwrap().short_version.version)
        } else {
            panic!("Failed");
        }
        if let Ok(interval) = WildFly::range("20..30") {
            assert_eq!(12, interval.len());
            assert_eq!(2000, interval[0].short_version.version);
            assert_eq!(3000, interval.last().unwrap().short_version.version)
        } else {
            panic!("Failed");
        }
    }

    #[test]
    fn range_from() {
        if let Ok(interval) = WildFly::range("26.1..") {
            assert_eq!(10, interval.len());
            assert_eq!(2610, interval[0].short_version.version);
            assert_eq!(3500, interval.last().unwrap().short_version.version)
        } else {
            panic!("Failed");
        }
        if let Ok(interval) = WildFly::range("30..") {
            assert_eq!(6, interval.len());
            assert_eq!(3000, interval[0].short_version.version);
            assert_eq!(3500, interval.last().unwrap().short_version.version)
        } else {
            panic!("Failed");
        }
        let last = VERSIONS.last_key_value().unwrap().0;
        if let Ok(interval) = WildFly::range(format!("{}..", last).as_str()) {
            assert_eq!(1, interval.len());
            assert_eq!(*last, interval[0].short_version);
        } else {
            panic!("Failed");
        }
    }

    #[test]
    fn range_to() {
        if let Ok(interval) = WildFly::range("..10") {
            assert_eq!(1, interval.len());
            assert_eq!(1000, interval[0].short_version.version);
        } else {
            panic!("Failed");
        }
        if let Ok(interval) = WildFly::range("..10.1") {
            assert_eq!(2, interval.len());
            assert_eq!(1000, interval[0].short_version.version);
            assert_eq!(1010, interval.last().unwrap().short_version.version)
        } else {
            panic!("Failed");
        }
        if let Ok(interval) = WildFly::range("..20") {
            assert_eq!(13, interval.len());
            assert_eq!(1000, interval[0].short_version.version);
            assert_eq!(2000, interval.last().unwrap().short_version.version)
        } else {
            panic!("Failed");
        }
    }

    #[test]
    fn range_all() {
        if let Ok(interval) = WildFly::range("..") {
            assert_eq!(VERSIONS.len(), interval.len());
            assert_eq!(
                *(VERSIONS.first_key_value().unwrap().0),
                interval[0].short_version
            );
            assert_eq!(
                *(VERSIONS.last_key_value().unwrap().0),
                interval.last().unwrap().short_version
            );
        } else {
            panic!("Failed");
        }
    }

    #[test]
    fn invalid_enumeration() {
        assert!(WildFly::enumeration("").is_err());
        assert!(WildFly::enumeration("  ").is_err());
        assert!(WildFly::enumeration(",").is_err());
        assert!(WildFly::enumeration("foo").is_err());
    }

    #[test]
    fn enumeration() {
        if let Ok(range) = WildFly::enumeration("23..26.1,dev,28,10,25,34") {
            assert_eq!(9, range.len());
            assert!(range[0].is_dev());
            assert_eq!(1000, range[1].short_version.version);
            assert_eq!(2300, range[2].short_version.version);
            assert_eq!(2400, range[3].short_version.version);
            assert_eq!(2500, range[4].short_version.version);
            assert_eq!(2600, range[5].short_version.version);
            assert_eq!(2610, range[6].short_version.version);
            assert_eq!(2800, range[7].short_version.version);
            assert_eq!(3400, range[8].short_version.version);
        } else {
            panic!("Failed");
        }
    }
}
