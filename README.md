# WildFly Container Versions

A library for managing WildFly container versions deployed at https://hub.docker.com/r/jboss/wildfly and
https://quay.io/repository/wildfly/wildfly.

The library contains a struct describing WildFly container versions:

```rust
use semver::Version;

/// Describes a WildFly container version
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct WildFlyContainer {
    pub identifier: u16,
    pub version: Version,
    pub short_version: String,
    pub core_version: Version,
    pub suffix: String,
    pub repository: String,
    pub platforms: Vec<String>,
}
```

In addition the library provides functions to parse expressions that represent single and multiple versions,
version ranges, enumerations, or a combination of all of them. 

These expressions follow this [BNF](https://bnfplayground.pauliankline.com/?bnf=%3Cexpression%3E%20%3A%3A%3D%20%3Cexpression%3E%20%22%2C%22%20%3Celement%3E%20%7C%20%3Celement%3E%0A%3Celement%3E%20%3A%3A%3D%20%3Cmultiplier%3E%20%22x%22%20%3Crange%3E%20%7C%20%3Cmultiplier%3E%20%22x%22%20%3Cversion%3E%20%7C%20%3Crange%3E%20%7C%20%3Cversion%3E%0A%3Crange%3E%20%3A%3A%3D%20%3Cversion%3E%20%22..%22%20%3Cversion%3E%20%7C%20%22..%22%20%3Cversion%3E%20%7C%20%3Cversion%3E%20%22..%22%20%7C%20%22..%22%0A%3Cmultiplier%3E%20%3A%3A%3D%20%3Cnonzero_number%3E%20%7C%20%3Ctwo_digit_number%3E%0A%3Cversion%3E%20%3A%3A%3D%20%3Cmajor%3E%20%7C%20%3Cmajor%3E%20%22.%22%20%3Cminor%3E%0A%3Cmajor%3E%20%3A%3A%3D%20%3Ctwo_digit_number%3E%20%7C%20%3Cthree_digit_number%3E%0A%3Cminor%3E%20%3A%3A%3D%20%3Cnonzero_number%3E%20%7C%20%3Ctwo_digit_number%3E%0A%3Cthree_digit_number%3E%20%3A%3A%3D%20%3Cnonzero_number%3E%20%3Cnumber%3E%20%3Cnumber%3E%0A%3Ctwo_digit_number%3E%20%3A%3A%3D%20%3Cnonzero_number%3E%20%3Cnumber%3E%0A%3Cnumber%3E%20%3A%3A%3D%20%220%22%20%7C%20%221%22%20%7C%20%222%22%20%7C%20%223%22%20%7C%20%224%22%20%7C%20%225%22%20%7C%20%226%22%20%7C%20%227%22%20%7C%20%228%22%20%7C%20%229%22%0A%3Cnonzero_number%3E%20%3A%3A%3D%20%221%22%20%7C%20%222%22%20%7C%20%223%22%20%7C%20%224%22%20%7C%20%225%22%20%7C%20%226%22%20%7C%20%227%22%20%7C%20%228%22%20%7C%20%229%22%0A&name=WildFly%20Container%20Versions):

```
<expression> ::= <expression> "," <element> | <element>
<element> ::= <multiplier> "x" <range> | <multiplier> "x" <version> | <range> | <version>
<range> ::= <version> ".." <version> | ".." <version> | <version> ".." | ".."
<multiplier> ::= <nonzero_number> | <two_digit_number>
<version> ::= <major> | <major> "." <minor>
<major> ::= <two_digit_number> | <three_digit_number>
<minor> ::= <nonzero_number> | <two_digit_number>
<three_digit_number> ::= <nonzero_number> <number> <number>
<two_digit_number> ::= <nonzero_number> <number>
<number> ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
<nonzero_number> ::= "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
```

Here are some examples

- 10
- 26.1
- 23..33
- 3x35
- 25..
- ..26.1
- ..
- 20,25..29,2x31,3x32,4x33..35

## Supported Versions

- 10
- 10.1
- 11
- 12
- 13
- 14
- 15
- 16
- 17
- 18
- 19
- 19.1.0
- 20
- 21
- 22
- 23
- 24
- 25
- 26
- 26.1.3
- 27
- 28
- 29
- 30
- 31
- 32
- 33
- 34
- 35
