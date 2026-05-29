<p align="center">
  <img src=".github/banner.svg" alt="Nexum · keycard — Rust SDK + CLI for Status Keycards" width="100%" />
</p>

A Rust toolkit for **Status Keycards** — smart cards that hold keys in a secure element and sign over an APDU channel. Ships the core SDK, an Ethereum signer that plugs into [alloy](https://github.com/alloy-rs/alloy), and a CLI for hands-on card administration.

Nexum uses Keycards as a hardware-bound signing path for the [wallet](https://github.com/nxm-rs/wallet): the seed never leaves the card, the wallet talks to it over NFC on the phone or PC/SC on a desktop reader.

> **Pre-release.** APIs may change. Not yet on crates.io.

Looking for the org overview? See **[github.com/nxm-rs](https://github.com/nxm-rs)**. This repo was renamed from `nexum-keycard` to `keycard` on 2026-05-29; crate names (`nexum-keycard*`) are unchanged.

---

## Crates

| Crate | What it is |
|---|---|
| **[`nexum-keycard`](./nexum-keycard)** | Core SDK: SELECT, INIT, pairing, secure channel, derive, sign |
| **[`nexum-keycard-signer`](./nexum-keycard-signer)** | `alloy::signers::Signer` backed by a Keycard |
| **[`nexum-keycard-cli`](./nexum-keycard-cli)** | CLI for initialisation, pairing, key derivation, signing |

For NFC transport on mobile, see `rust/nexum-apdu-transport-nfc` in [`nxm-rs/wallet`](https://github.com/nxm-rs/wallet) — same `CardTransport` trait, different channel. To target a non-PC/SC environment, gate the `pcsc` feature off and write a `CardTransport` over your channel.

---

## Quickstart

```toml
nexum-keycard = { git = "https://github.com/nxm-rs/keycard", rev = "..." }
```

```rust
use nexum_keycard::{Keycard, PcscDeviceManager, CardExecutor, Error};

fn main() -> Result<(), Error> {
    let manager   = PcscDeviceManager::new()?;
    let readers   = manager.list_readers()?;
    let reader    = readers.iter().find(|r| r.has_card()).expect("no card present");
    let transport = manager.open_reader(reader.name())?;

    let mut executor = CardExecutor::new_with_defaults(transport);
    let mut keycard  = Keycard::new(&mut executor);

    let info = keycard.select_keycard()?;
    println!("applet {} · instance {}", info.version, info.instance_uid);

    if !info.initialized() {
        let secrets = keycard.init(None, None, None)?;
        println!("PIN: {}\nPUK: {}\nPAIRING: {}",
                 secrets.pin(), secrets.puk(), secrets.pairing_password());
    }
    Ok(())
}
```

The CLI mirrors the SDK — see [`nexum-keycard-cli/README.md`](./nexum-keycard-cli/README.md) for `init`, `pair`, `status`, `derive`, `sign`. Pairing material persists on disk in a user-config dir; treat it as sensitive.

---

## Contributing

Open an issue before non-trivial PRs. Conventional Commits, `Signed-off-by` (DCO), `cargo fmt`, `cargo clippy -- -D warnings`. Smart-card code lives close to crypto — keep the attack surface small. CLA in [`CLA.md`](./CLA.md).

## Security

See [SECURITY.md](https://github.com/nxm-rs/.github/blob/main/SECURITY.md). Secure-channel handshake, pairing persistence, and APDU framing findings via GitHub Security Advisories on this repo.

## License

AGPL-3.0-or-later. See [LICENSE](./LICENSE).

```
●  AGPL-3.0  ·  pre-release  ·  hardware-bound signing
```
