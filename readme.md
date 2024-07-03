# wte-mt-rx-parser

[![WTE-MT-RX Parser on crates.io][cratesio-image]][cratesio]
[![WTE-MT-RX Parser on docs.rs][docsrs-image]][docsrs]
[![GitHub last commit][ghcommit-image]][ghcommit]

[cratesio-image]: https://img.shields.io/crates/v/wte-mt-rx-parser.svg
[cratesio]: https://crates.io/crates/wte-mt-rx-parser
[docsrs-image]: https://docs.rs/wte-mt-rx-parser/badge.svg
[docsrs]: https://docs.rs/wte-mt-rx-parser
[ghcommit-image]: https://img.shields.io/github/last-commit/roby2014/wte-mt-rx-parser
[ghcommit]: https://github.com/roby2014/wte-mt-rx-parser/

This Rust crate aims to provide functionality for parsing messages received by the [WTE MT-RX-3 AIS, 406 + 121.5 alerting receivers](https://www.wte.co.nz/uploads/9/9/8/6/99862766/mt-rx-3_406_epirb_receiver-manual_v2-62.pdf). These receivers are designed to decode various types of emergency and maritime communication signals, including:
- 406MHz EPIRBs, PLBs and ELT beacons operating across the frequencies 406.020 to 406.045 MHz.
- AIS based SART alerting devices operating on both 161.975MHz and 162.025MHz.
- 121.5MHz or 243MHz man-overboard devices and emergency homing transmitters with the use of directional antennas.

## Features

Provides simple parsing utilities for:
- 406 Message Serial Output
    - MT protocol **structured** messages (`MT1UUUNNNTFHHHHHHHHHHHHHHHSS112233N4445566WYYYY`)
    - MT protocol **raw** messages (`MT6UUUNNNRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRYYYY`)
- RSS frequency / alerts (`SS,1,NNN<CR>` / `SS,A,NNN<CR>`)

Notes:
- *For parsing AIS messages (NMEA format), refer to other crates, such as [nmea-parser](https://github.com/zaari/nmea-parser).*
- *Refer to MT-RX-3 user manual for more information ([#references](#references)).*

## Usage

```toml
[dependencies]
wte-mt-rx-parser = "0.1.4"
```

```rs
fn main() {
    let samples = vec![
        "MT1001000AL400C592753572B323433212S1723756E4706",
        "MT6001001FFFE2FA00E0000CBAB959DB0903788C71B79F84B",
        "SS,A,123",
        "SS,1,123",
    ];

    for s in samples {
        println!("{:?}", wte_mt_rx_parser::parse(s));
    }
```

Output:
```
Ok(MtStructured(MtStructured { header: "MT1", id: "001", sequence_number: 0, message_type: Alert, format_flag: 'L', beacon: [52, 48, 48, 67, 53, 57, 50, 55, 53, 51, 53, 55, 50, 66, 51], signal_strength: ['2', '3'], lat_degrees: 43, lat_minutes: 32, lat_seconds: 12, n: 'S', long_degrees: 172, long_minutes: 37, long_seconds: 56, w: 'E', checksum: 18182 }))
    
Ok(MtRaw(MtRaw { header: "MT6", id: "001", sequence_number: 1, data: [70, 70, 70, 69, 50, 70, 65, 48, 48, 69, 48, 48, 48, 48, 67, 66, 65, 66, 57, 53, 57, 68, 66, 48, 57, 48, 51, 55, 56, 56, 67, 55, 49, 66, 55, 57], checksum: 63563 }))
    
Ok(Rss(Rss { rss_type: Alert, nnn: 123 }))

Ok(Rss(Rss { rss_type: Frequency, nnn: 123 }))
```

## Contributing

If you find any issues or have suggestions for improvement, please feel free to open an issue.

## References
- [MT-RX-3 User Manual](https://www.wte.co.nz/uploads/9/9/8/6/99862766/mt-rx-3_406_epirb_receiver-manual_v2-62.pdf)

## License
