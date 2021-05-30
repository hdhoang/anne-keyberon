Many thanks to Obins and generations of AnnePro reverse-engineers/devs:

- OpenAnnePro spearheaded by @Codetector1374 https://github.com/OpenAnnePro
- @Blucky87 https://github.com/Blucky87/AnneProCLI
- @hi-a https://hi-a.github.io/annepro-key/
- @ah- https://github.com/ah-/anne-key
- @fcoury https://github.com/fcoury/node-anne-pro and https://github.com/fcoury/electron-anne-pro
- @metr1xx https://github.com/metr1xx/anne-pro-community-app
- @msvisser https://github.com/msvisser/qmk_firmware/tree/anne_pro/keyboards/anne_pro and https://github.com/msvisser/AnnePro-mac
- @josecostamartins https://github.com/josecostamartins/qmk_firmware/commits/anne_pro
- @dwhinham https://github.com/dwhinham/qmk_firmware/commits/anne_pro
- @kprinssu https://github.com/kprinssu/anne-keyboard-windows

Created from knurling-rs/app-template, license & support info below.

Please observe the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct) within our community.

On HexCore (née Obins) Anne Pro (2018 with ST MCUs), we have achieved RTT logging over SWD probe:

```powershell
> Executing task: cargo run --package anne-keyberon --bin format <

   Compiling anne-keyberon v0.1.0 (B:\code\anne\anne-keyberon)
    Finished dev [optimized + debuginfo] target(s) in 1.32s
     Running `probe-run --chip STM32L151C8 target\thumbv7m-none-eabi\debug\format`
(HOST) INFO  flashing program (6.83 KiB)
(HOST) INFO  success!
────────────────────────────────────────────────────────────────────────────────
0 INFO  s=S1 { x: 42, y: S2 { z: 43 } }
└─ format::__cortex_m_rt_main @ src\bin\format.rs:23
1 INFO  x=42
└─ format::__cortex_m_rt_main @ src\bin\format.rs:25
────────────────────────────────────────────────────────────────────────────────
Error: debug information is missing. Likely fixes:
1. compile the Rust code with `debug = 1` or higher. This is configured in the `profile.{release,bench}` sections of Cargo.toml (`profile.{dev,test}` default to `debug = 2`)
2. use a recent version of the `cortex-m` crates (e.g. cortex-m 0.6.3 or newer). Check versions in Cargo.lock
3. if linking to C code, compile the C code with the `-g` flag

Caused by:
    Do not have unwind info for the given address.
error: process didn't exit successfully: `probe-run --chip STM32L151C8 target\thumbv7m-none-eabi\debug\format` (exit code: 1)
```

Next steps:
- [ ] Port stm32-rs/stm32l1xx-hal to PAC 0.13.0
- [ ] "use a recent version of the `cortex-m` crates (e.g. cortex-m 0.6.3 or newer). Check versions in Cargo.lock"
- [ ] Import RTIC
- [ ] Import keyberon
- [ ] Reconstruct BT-chip UART+protocol from ah-/anne-key
- [ ] Deal with the LED chip, which is not working on my 2 keyboards, for any Key firmware

## Support

`app-template` is part of the [Knurling] project, [Ferrous Systems]' effort at
improving tooling used to develop for embedded systems.

If you think that our work is useful, consider sponsoring it via [GitHub
Sponsors].

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.

[Knurling]: https://knurling.ferrous-systems.com
[Ferrous Systems]: https://ferrous-systems.com/
[GitHub Sponsors]: https://github.com/sponsors/knurling-rs
