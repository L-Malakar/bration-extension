# <img src="https://github.com/L-Malakar/bration-extension/blob/main/image/icon-128.png" width="40" valign="middle"/> Bration-Extension

**Your Screen. Your Data. Everything Else — Blocked.**

An open-source ad, tracker, and popup blocker for browsers, powered by Brave's own [`adblock-rust`](https://github.com/brave/adblock-rust) engine, compiled to Web Assembly. No accounts. No telemetry. No data ever leaves your machine.

[![Available on Firefox Add-ons](https://img.shields.io/badge/Firefox-Add--on-FF7139?logo=firefoxbrowser&logoColor=white)](https://addons.mozilla.org/en-US/firefox/addon/bration-extension/)
[![License: MPL 2.0](https://img.shields.io/badge/License-MPL%202.0-blue.svg)](https://opensource.org/licenses/MPL-2.0)

---
[![Banner](https://github.com/L-Malakar/bration-extension/blob/main/image/banner.png)](https://github.com/L-Malakar)
## ✨ Features

- **Network-level blocking** — a stacked filter list (EasyList, EasyPrivacy, AdGuard Base, AdGuard Annoyances, plus dedicated scam/malvertising-redirect lists) checked against every request via a Rust engine compiled to WebAssembly
- **Cosmetic filtering** — hides ad containers and sponsored banners that load as page content, not just blockable network requests
- **Popup blocking** — automatically closes ad-triggered popup tabs
- **Forced-redirect protection** — stops cross-site scripts from hijacking your navigation to scam pages
- **Per-site control** — turn blocking off for one specific site without affecting any other tab or domain
- **Dashboard** — a WhatsApp-style view of every domain you've visited (30-day history), with a full breakdown of exactly what got blocked and where

## 🔒 Privacy

Everything is stored locally via your browser's own storage. Nothing is sent to any external server, no account is required, and the extension's manifest explicitly declares **zero data collection**.

## 🧠 How it works

```
adblock-rust (Rust)
      │
      ▼  compiled to WebAssembly
bration_extinction_core_bg.wasm
      │
      ├── Network blocking     → webRequest interception
      ├── Cosmetic filtering   → CSS injection via content script
      └── Popup/redirect guard → webNavigation heuristics
```

The Rust core lives in [`bration-extension-core/`](./bration-extension-core) and is compiled automatically via GitHub Actions on every push — see [`.github/workflows/build-wasm.yml`](./.github/workflows/build-wasm.yml).

## 🛠️ Building from source

```bash
# Requires Rust + wasm-pack
cd bration-extension-core
wasm-pack build --target web --release
```

This produces the same `wasm-core/` output bundled with the extension. The build also runs automatically on every push via GitHub Actions — no local setup required if you just want to verify the build.

## 📦 Installing

- **Firefox**: install from [addons.mozilla.org](https://addons.mozilla.org/en-US/firefox/addon/bration-extension/)
- **Chrome / Edge**: coming soon

## 🐛 Reporting bugs

Please open an [issue](https://github.com/L-Malakar/bration-extension/issues/new) on this repo — include your browser version and the site where you ran into a problem.

## 🗺️ Roadmap

- [ ] Chrome / Edge support (Manifest V3)
- [ ] Firefox for Android
- [ ] Connected web dashboard (history across visits, remote controls)
- [ ] Expanded YouTube-specific filtering

---

Developed independently by [**L. Malakar**](https://github.com/L-Malakar).
