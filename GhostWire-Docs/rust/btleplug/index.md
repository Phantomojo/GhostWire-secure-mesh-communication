Crate btleplug Copy item path

<span class="sub-heading"><a href="../src/btleplug/lib.rs.html#14-155" class="src">Source</a>
</span>

Expand description

btleplug is a Bluetooth Low Energy (BLE) central module library for
Rust. It currently supports Windows 10, macOS (and possibly iOS) and
Linux (BlueZ). Android support is planned for the future.

## <a href="#usage" class="doc-anchor">§</a>Usage

An example of how to use the library to control some BLE smart lights:

``` {.rust .rust-example-rendered}
use btleplug::api::{bleuuid::uuid_from_u16, Central, Manager as _, Peripheral as _, ScanFilter, WriteType};
use btleplug::platform::{Adapter, Manager, Peripheral};
use rand::{Rng, rng};
use std::error::Error;
use std::thread;
use std::time::Duration;
use tokio::time;
use uuid::Uuid;

const LIGHT_CHARACTERISTIC_UUID: Uuid = uuid_from_u16(0xFFE9);

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let manager = Manager::new().await.unwrap();

    // get the first bluetooth adapter
    let adapters = manager.adapters().await?;
    let central = adapters.into_iter().nth(0).unwrap();

    // start scanning for devices
    central.start_scan(ScanFilter::default()).await?;
    // instead of waiting, you can use central.events() to get a stream which will
    // notify you of new devices, for an example of that see examples/event_driven_discovery.rs
    time::sleep(Duration::from_secs(2)).await;

    // find the device we're interested in
    let light = find_light(&central).await.unwrap();

    // connect to the device
    light.connect().await?;

    // discover services and characteristics
    light.discover_services().await?;

    // find the characteristic we want
    let chars = light.characteristics();
    let cmd_char = chars.iter().find(|c| c.uuid == LIGHT_CHARACTERISTIC_UUID).unwrap();

    // dance party
    let mut rng = rng();
    for _ in 0..20 {
        let color_cmd = vec![0x56, rng.random(), rng.random(), rng.random(), 0x00, 0xF0, 0xAA];
        light.write(&cmd_char, &color_cmd, WriteType::WithoutResponse).await?;
        time::sleep(Duration::from_millis(200)).await;
    }
    Ok(())
}

async fn find_light(central: &Adapter) -> Option<Peripheral> {
    for p in central.peripherals().await.unwrap() {
        if p.properties()
            .await
            .unwrap()
            .unwrap()
            .local_name
            .iter()
            .any(|name| name.contains("LEDBlue"))
        {
            return Some(p);
        }
    }
    None
}
```

# Modules<a href="#modules" class="anchor">§</a>

<a href="api/index.html" class="mod" title="mod btleplug::api">api</a>  
The `api` module contains the traits and types which make up btleplug’s
API. These traits have a different implementation for each supported
platform, but only one implementation can be found on any given
platform. These implementations are in the
[`platform`](platform/index.html "mod btleplug::platform") module.

<a href="platform/index.html" class="mod"
title="mod btleplug::platform">platform</a>  
The `platform` module contains the platform-specific implementations of
the various [`api`](api/index.html "mod btleplug::api") traits. Refer
for the `api` module for how to use them.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Error.html" class="enum"
title="enum btleplug::Error">Error</a>  
The main error type returned by most methods in btleplug.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.Result.html" class="type"
title="type btleplug::Result">Result</a>  
Convenience type for a result using the btleplug
[`Error`](enum.Error.html "enum btleplug::Error") type.
