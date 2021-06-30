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
❯ cargo rrb rtic_schedule
   Compiling anne-keyberon v0.1.0 (B:\code\anne\anne-keyberon)
    Finished release [optimized + debuginfo] target(s) in 1.36s
     Running `probe-run --chip STM32L151C8 target\thumbv7m-none-eabi\release\rtic_schedule`
(HOST) INFO  flashing program (7.61 KiB)
(HOST) INFO  success!
────────────────────────────────────────────────────────────────────────────────
0 INFO  Hello from init! Please wait for the scheduled task
└─ rtic_schedule::init @ src\bin\rtic_schedule.rs:54
1 INFO  idling...
└─ rtic_schedule::idle @ src\bin\rtic_schedule.rs:17
2 INFO  Hello world from turn=100!
└─ rtic_schedule::hello_world_task @ src\bin\rtic_schedule.rs:65
3 INFO  Hello world from turn=30!
└─ rtic_schedule::hello_world_task @ src\bin\rtic_schedule.rs:65
4 INFO  Hello world from turn=30!
└─ rtic_schedule::hello_world_task @ src\bin\rtic_schedule.rs:65
5 INFO  Hello world from turn=30!
└─ rtic_schedule::hello_world_task @ src\bin\rtic_schedule.rs:65
────────────────────────────────────────────────────────────────────────────────
stack backtrace:
   0: __wfi
        at ./asm/lib.rs:50:14
   1: rtic_schedule::idle
        at src\bin/rtic_schedule.rs:19:13
   2: main
        at src\bin/rtic_schedule.rs:13:1
   3: Reset
```

Next steps:
- [x] Port [stm32-rs/stm32l1xx-hal to PAC 0.13.0](https://github.com/hdhoang/stm32l1xx-hal/tree/dev-crate-update-v0.13.0), great thanks to @jyrama for heavylifting
- [x] "use a recent version of the `cortex-m` crates (e.g. cortex-m 0.6.3 or newer)" because that's Obins' bootloader code at `address 0x8002e86`
- [x] Import RTIC
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
