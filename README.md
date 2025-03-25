# WildFly Container Versions

A library for managing WildFly container versions deployed at https://hub.docker.com/r/jboss/wildfly and
https://quay.io/repository/wildfly/wildfly.

The library contains a struct describing the WildFly container versions

```rust
use semver::Version;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct WildFlyContainer {
    pub version: Version,
    pub core_version: Version,
    pub suffix: String,
    pub repository: String,
    pub platforms: Vec<String>,
}
```

and functions to parse version enumerations and ranges

```rust
use wildfly_container_versions::WildFlyContainer;

let versions = WildFlyContainer::enumeration("23..26.1,dev,28,10,25,34");
```
