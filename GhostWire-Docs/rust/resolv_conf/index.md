Crate resolv\_conf Copy item path

<span class="sub-heading"><a href="../src/resolv_conf/lib.rs.html#1-906" class="src">Source</a>
</span>

Expand description

The crate simply parses `/etc/resolv.conf` file and creates a config
object

# <a href="#examples" class="doc-anchor">§</a>Examples

## <a href="#parsing-a-config-from-a-string" class="doc-anchor">§</a>Parsing a config from a string

``` {.rust .rust-example-rendered}
extern crate resolv_conf;

use std::net::{Ipv4Addr, Ipv6Addr};
use resolv_conf::{ScopedIp, Config, Network};

fn main() {
    let config_str = "
options ndots:8 timeout:8 attempts:8

domain example.com
search example.com sub.example.com

nameserver 2001:4860:4860::8888
nameserver 2001:4860:4860::8844
nameserver 8.8.8.8
nameserver 8.8.4.4

options rotate
options inet6 no-tld-query

sortlist 130.155.160.0/255.255.240.0 130.155.0.0";

    // Parse the config.
    let parsed_config = Config::parse(&config_str).expect("Failed to parse config");

    // We can build configs manually as well, either directly or with Config::new()
    let mut expected_config = Config::new();
    expected_config.nameservers = vec![
        ScopedIp::V6(Ipv6Addr::new(0x2001, 0x4860, 0x4860, 0, 0, 0, 0, 0x8888), None),
        ScopedIp::V6(Ipv6Addr::new(0x2001, 0x4860, 0x4860, 0, 0, 0, 0, 0x8844), None),
        ScopedIp::V4(Ipv4Addr::new(8, 8, 8, 8)),
        ScopedIp::V4(Ipv4Addr::new(8, 8, 4, 4)),
    ];
    expected_config.sortlist = vec![
        Network::V4(Ipv4Addr::new(130, 155, 160, 0), Ipv4Addr::new(255, 255, 240, 0)),
        Network::V4(Ipv4Addr::new(130, 155, 0, 0), Ipv4Addr::new(255, 255, 0, 0)),
    ];
    expected_config.debug = false;
    expected_config.ndots = 8;
    expected_config.timeout = 8;
    expected_config.attempts = 8;
    expected_config.rotate = true;
    expected_config.no_check_names = false;
    expected_config.inet6 = true;
    expected_config.ip6_bytestring = false;
    expected_config.ip6_dotint = false;
    expected_config.edns0 = false;
    expected_config.single_request = false;
    expected_config.single_request_reopen = false;
    expected_config.no_tld_query = true;
    expected_config.use_vc = false;
    expected_config.set_domain(String::from("example.com"));
    expected_config.set_search(vec![
        String::from("example.com"),
        String::from("sub.example.com")
    ]);

    // We can compare configurations, since resolv_conf::Config implements Eq
    assert_eq!(parsed_config, expected_config);
}
```

## <a href="#parsing-a-file" class="doc-anchor">§</a>Parsing a file

``` {.rust .rust-example-rendered}
use std::io::Read;
use std::fs::File;

extern crate resolv_conf;

fn main() {
    // Read the file
    let mut buf = Vec::with_capacity(4096);
    let mut f = File::open("/etc/resolv.conf").unwrap();
    f.read_to_end(&mut buf).unwrap();

    // Parse the buffer
    let cfg = resolv_conf::Config::parse(&buf).unwrap();

    // Print the config
    println!("---- Parsed /etc/resolv.conf -----\n{:#?}\n", cfg);
}
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.AddrParseError.html" class="struct"
title="struct resolv_conf::AddrParseError">AddrParseError</a>  
An error which can be returned when parsing an IP address.

<a href="struct.Config.html" class="struct"
title="struct resolv_conf::Config">Config</a>  
Represent a resolver configuration, as described in `man 5 resolv.conf`.
The options and defaults match those in the linux `man` page.

<a href="struct.DomainIter.html" class="struct"
title="struct resolv_conf::DomainIter">DomainIter</a>  
An iterator returned by
[`Config.get_last_search_or_domain`](struct.Config.html#method.get_last_search_or_domain)

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Family.html" class="enum"
title="enum resolv_conf::Family">Family</a>  
The internet protocol family that is prefered. This option is commonly
found on openbsd.

<a href="enum.Lookup.html" class="enum"
title="enum resolv_conf::Lookup">Lookup</a>  
The databases that should be searched during a lookup. This option is
commonly found on openbsd.

<a href="enum.Network.html" class="enum"
title="enum resolv_conf::Network">Network</a>  
A network, that is an IP address and a mask

<a href="enum.ParseError.html" class="enum"
title="enum resolv_conf::ParseError">ParseError</a>  
Error while parsing resolv.conf file

<a href="enum.ScopedIp.html" class="enum"
title="enum resolv_conf::ScopedIp">ScopedIp</a>  
Represent an IP address. This type is similar to `std::net::IpAddr` but
it supports IPv6 scope identifiers.
