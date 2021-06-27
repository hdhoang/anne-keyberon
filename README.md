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
- @korken89 for the [Oxidized 2020 RTIC demos](https://github.com/korken89/oxidize2020-rtic/tree/master/examples)

Created from knurling-rs/app-template, license & support info below.

Please observe the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct) within our community.

On HexCore (née Obins) Anne Pro (2018 with ST MCUs), we have achieved RTT logging over SWD probe:

```powershell
anne-keyberon on  main [!?] is 📦 v0.1.0 via 🦀 v1.53.0
❯ cargo rb panic
    Finished dev [optimized + debuginfo] target(s) in 0.06s
     Running `probe-run --chip STM32L151C8 target\thumbv7m-none-eabi\debug\panic`
(HOST) INFO  flashing program (6.77 KiB)
(HOST) INFO  success!
────────────────────────────────────────────────────────────────────────────────
0 INFO  main
└─ panic::__cortex_m_rt_main @ src\bin\panic.rs:10
1 ERROR panicked at 'explicit panic'
└─ panic::__cortex_m_rt_main @ src\bin\panic.rs:12
────────────────────────────────────────────────────────────────────────────────
stack backtrace:
   0: HardFaultTrampoline
      <exception entry>
   1: lib::inline::__udf
        at ./asm/inline.rs:172:5
   2: __udf
        at ./asm/lib.rs:49:17
   3: cortex_m::asm::udf
        at C:\Users\ms\scoop\persist\rustup-msvc\.cargo\registry\src\github.com-1ecc6299db9ec823\cortex-m-0.7.2\src/asm.rs:43:5
   4: _defmt_panic
        at src/lib.rs:14:5
   5: defmt::export::panic
        at C:\Users\ms\scoop\persist\rustup-msvc\.cargo\registry\src\github.com-1ecc6299db9ec823\defmt-0.2.3\C:\Users\ms\scoop\persist\rustup-msvc\.cargo\registry\src\github.com-1ecc6299db9ec823\defmt-0.2.3\src/export.rs:233:14
   6: panic::__cortex_m_rt_main
        at src\bin/panic.rs:12:5
   7: main
        at src\bin/panic.rs:6:1
   8: Reset
        at C:\Users\ms\scoop\persist\rustup-msvc\.cargo\registry\src\github.com-1ecc6299db9ec823\cortex-m-rt-0.6.14\C:\Users\ms\scoop\persist\rustup-msvc\.cargo\registry\src\github.com-1ecc6299db9ec823\cortex-m-rt-0.6.14\src/lib.rs:526:15
   9: {"package":"panic-probe","tag":"defmt_error","data":"{}","disambiguator":"6502333312778159192"}
(HOST) ERROR error occurred during backtrace creation: debug information for address 0x8002e86 is missing. Likely fixes:
        1. compile the Rust code with `debug = 1` or higher. This is configured in the `profile.{release,bench}` sections of Cargo.toml (`profile.{dev,test}` default to `debug = 2`)
        2. use a recent version of the `cortex-m` crates (e.g. cortex-m 0.6.3 or newer). Check versions in Cargo.lock
        3. if linking to C code, compile the C code with the `-g` flag

Caused by:
    Do not have unwind info for the given address.
               the backtrace may be incomplete.
(HOST) ERROR the program panicked
error: process didn't exit successfully: `probe-run --chip STM32L151C8 target\thumbv7m-none-eabi\debug\panic` (exit code: 134)
```

Next steps:
- [x] Port [stm32-rs/stm32l1xx-hal to PAC 0.13.0](https://github.com/hdhoang/stm32l1xx-hal/tree/dev-crate-update-v0.13.0), great thanks to @jyrama for heavylifting
- [x] "use a recent version of the `cortex-m` crates (e.g. cortex-m 0.6.3 or newer)" because that's Obins' bootloader code at `address 0x8002e86`
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
