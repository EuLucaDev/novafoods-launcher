<p align="center">
  <a href="https://workflowind.agcunae.com/apps/">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="docs/brand/novafoods-logo-white.svg">
      <img alt="Nova Foods" src="docs/brand/novafoods-logo.svg" width="200">
    </picture>
  </a>
</p>

<h1 align="center">Nova Foods Launcher</h1>

<p align="center">
  <b>One place to install, update and open every app in the Nova Foods Apps, rebuilt in pure Rust.</b><br>
  Nova Photo, Nova Light, Nova Film, Nova Design, Nova Vector, Nova PDF, Nova Effect, Nova Word,<br>
  Nova Deck, Nova Grid, Nova CAD, Nova Sound and Nova Fig: the right build for your computer, kept up to date.
</p>

<p align="center">
  <img alt="100% Rust" src="https://img.shields.io/badge/100%25-Rust-b7410e?style=flat-square&logo=rust">
  <img alt="macOS · Windows · Linux · FreeBSD · Web" src="https://img.shields.io/badge/macOS%20%C2%B7%20Windows%20%C2%B7%20Linux%20%C2%B7%20FreeBSD%20%C2%B7%20Web-native-26282e?style=flat-square">
  <img alt="License: MIT OR Apache-2.0" src="https://img.shields.io/badge/license-MIT%20%2F%20Apache--2.0-3a3a3a?style=flat-square">
  <img alt="Status: early alpha" src="https://img.shields.io/badge/status-early%20alpha-d69e2e?style=flat-square">
</p>

<p align="center">
  <a href="https://workflowind.agcunae.com/inbox"><img alt="Talk to the Nova Foods IT team" src="https://img.shields.io/badge/Join%20us%20on%20Suporte-5865F2?style=for-the-badge&logo=discord&logoColor=white" height="40"></a>
</p>

<p align="center">
  <a href="https://github.com/EuLucaDev/novafoods-launcher/releases/latest"><b>Download Nova Foods Launcher</b></a> ·
  <a href="https://workflowind.agcunae.com/apps/">Nova Foods</a> ·
  <a href="https://workflowind.agcunae.com/apps">The Nova Foods Apps</a>
</p>

> [!NOTE]
> **Nova Foods is a community of artists from all walks of life.** Painters, photographers,
> filmmakers, illustrators, designers, animators, hobbyists, and people who picked up a pencil
> last week. If you make things, you're one of us. **[Come say hi on Suporte](https://workflowind.agcunae.com/inbox).**

<p align="center">
  <a href="#what-it-does">What it does</a> ·
  <a href="#features">Features</a> ·
  <a href="#downloads">Downloads</a> ·
  <a href="#build-from-source">Build from source</a> ·
  <a href="#the-craft-app-suite">Nova Foods Apps</a> ·
  <a href="https://workflowind.agcunae.com/inbox">Suporte</a>
</p>

<p align="center">
  <img src="docs/screenshots/novafoods-launcher-grid.png" alt="Nova Foods Launcher on macOS in the grid view (dark theme): a card for each Nova Foods App with its icon, status, version, release date and an Install or Open button" width="900">
</p>
<p align="center">
  <img src="docs/screenshots/novafoods-launcher-table.png" alt="The same apps in the table view (light theme), with columns for each app's status, installed and latest versions, and release date" width="900"><br>
  <sub>Switch between the grid and a sortable table.</sub>
</p>

## What it does

The Nova Foods apps each ship their own builds for every platform. Nova Foods Launcher saves you from
hunting through a dozen release pages: it lists every app in the suite, shows which ones you have
and which version, installs the right build for your operating system and processor, and tells
you when an update is out. It is a small native app that can sit in your menu bar or system tray.

## Features

<table>
  <tr>
    <td width="33%" valign="top">
      <h3>📦 Install in one click</h3>
      Picks the right download for your OS and CPU (DMG, MSI, AppImage, .deb, .rpm and more), checks it against the release's SHA-256 checksums and installs it.
    </td>
    <td width="33%" valign="top">
      <h3>🔄 Stay up to date</h3>
      Checks each app's GitHub releases and updates the ones you have. A notification tells you when there's something new.
    </td>
    <td width="33%" valign="top">
      <h3>🧭 Always at hand</h3>
      Opens any Nova Foods app from one window, or from the tray and menu-bar icon. Native egui, no Electron, no web view.
    </td>
  </tr>
</table>

The web version shows the same catalog in the browser, with the latest download for each app.

## Interface languages

The interface is in English, Brazilian Portuguese (Português (Brasil)), Czech (Čeština), French
(Français), German (Deutsch), Hungarian (Magyar), Japanese (日本語), Polish (Polski), Russian
(Русский), Simplified Chinese (简体中文) or Spanish (Español). **Auto**, the default, follows the
system or browser language; **Settings → Language** picks one and switches at once, and the choice
is kept between runs. Czech locales (`cs-CZ`, `cs_CZ.UTF-8`…) select Czech, German ones (`de-DE`,
`de-AT`, `de-CH`, `de_DE.UTF-8`…) German, French ones (`fr-FR`, `fr-CA`…) French, Hungarian ones
(`hu`, `hu-HU`, `hu_HU.UTF-8`…) Hungarian, Japanese ones (`ja`, `ja-JP`, `ja_JP.UTF-8`…) Japanese,
Polish ones (`pl`, `pl-PL`, `pl_PL.UTF-8`) Polish, Spanish ones (`es-ES`, `es-MX`, `es-419`…)
Spanish, and Portuguese ones (`pt-BR`, and `pt-PT` too, for now) Brazilian Portuguese. Simplified
Chinese locales (`zh-CN`, `zh-SG`, `zh-Hans`, and POSIX forms such as `zh_CN.UTF-8`) select
Chinese; Traditional Chinese ones don't. To try a language without changing the setting, start the
desktop app with `NOVAFOODS_LAUNCHER_LOCALE=pt-BR`, or add `?lang=ru` to the web address.

The CJK languages are drawn with the system's own CJK font (Hiragino Sans or PingFang on macOS,
Yu Gothic, Meiryo or Microsoft YaHei on Windows, Noto Sans CJK, IPA, Takao or WenQuanYi on Linux and
FreeBSD). Where the system has no font for one, and in the web version, the interface stays in
English and Settings says why. Product names and release notes stay as published; text without a
translation falls back to English.

## Where apps are installed

Nova Foods Launcher installs each app where your system normally keeps applications, using the
app's official release package, so an app installed by the launcher is the same as one you
installed by hand (and the launcher finds apps you installed yourself):

| System | Where | How |
|---|---|---|
| macOS | `/Applications/<App>.app` | Copied from the signed, notarized release DMG |
| Windows | `C:\Program Files\<App>` | The release MSI, through Windows Installer (with Start menu entry and Apps & features entry) |
| Linux | `/usr/bin`, `/usr/share/applications` | The release `.deb` (apt) or `.rpm` (dnf, zypper), with your system's password prompt. Without those, a Flatpak, or an AppImage in `~/Applications` |
| Linux, inside the launcher's Flatpak, with no Flatpak on the host | `~/.var/app/br.com.novafoods.novafoods-launcher/data/novafoods-launcher/apps/<app>` | The release tarball, unpacked into the launcher's own folder and run inside the same sandbox, with its settings in a `home/` folder beside it. Nothing is installed on the host, and each app's configuration stays in the sandbox |
| FreeBSD | `/usr/local` | The release tree, with a password prompt (polkit); `~/.local` when polkit isn't installed |

Nova Foods Launcher keeps itself current too: when a new version is out, **Update and restart**
downloads it, verifies it, puts it where this copy is installed (the `.app`, the Windows Installer
package or portable folder, the AppImage, or the `.deb`/`.rpm`) and reopens it.

Updates use the same format an app was installed with. Every download is checked against the
release's published SHA-256 before it is installed, and a running app is never replaced.

The launcher writes its log to `logs/novafoods-launcher.log` in its data folder
(`~/.local/share/novafoods-launcher` on Linux, `~/Library/Application Support/Nova Foods Launcher`
on macOS, `%LOCALAPPDATA%\Nova Foods Launcher` on Windows, `NovaFoodsLauncherData` beside the
portable zip's executable) and keeps the previous two runs as `novafoods-launcher.1.log` and
`novafoods-launcher.2.log`. `RUST_LOG` changes what it records (`debug`, `info,zbus=warn`, …).

## Downloads

Download the build for your computer from the [latest release](https://github.com/EuLucaDev/novafoods-launcher/releases/latest), or browse [all releases](https://github.com/EuLucaDev/novafoods-launcher/releases). Every release has a `SHA256SUMS.txt` to check the files against. `<ver>` is the version number, for example `0.1.0`.

**Windows** (10 and 11)

| File | What it is |
|---|---|
| `novafoods-launcher-<ver>-windows-x64.msi` | Installer for most PCs (Intel and AMD, 64-bit). Installs for the current user, no admin rights needed |
| `novafoods-launcher-<ver>-windows-arm64.msi` | Installer for Windows on Arm (Snapdragon and other Arm PCs) |
| `novafoods-launcher-<ver>-windows-x86.msi` | Installer for 32-bit Windows |
| `novafoods-launcher-<ver>-windows-{x64,arm64,x86}-portable.zip` | No install: unzip and run `novafoods-launcher.exe`. Keeps its settings in the folder |

**macOS** (11 Big Sur or later)

| File | What it is |
|---|---|
| `novafoods-launcher-<ver>-macos-universal.dmg` | Universal app for Apple silicon and Intel Macs, signed and notarized. Drag Nova Foods Launcher to Applications |

**Linux** (x86_64 and aarch64; glibc 2.35 or newer, e.g. Ubuntu 22.04+, Debian 12+, Fedora 36+)

| File | What it is |
|---|---|
| `novafoods-launcher-<ver>-linux-{x86_64,aarch64}.AppImage` | Runs on almost any distro without installing; `chmod +x` and run. The `.AppImage.zsync` beside it lets AppImageUpdate fetch only what changed |
| `novafoods-launcher-<ver>-linux-{x86_64,aarch64}.deb` | Debian, Ubuntu, Mint, Pop!_OS: `sudo apt install ./novafoods-launcher-…deb` |
| `novafoods-launcher-<ver>-linux-{x86_64,aarch64}.rpm` | Fedora, openSUSE, RHEL: `sudo dnf install ./novafoods-launcher-…rpm` |
| `novafoods-launcher-<ver>-linux-{x86_64,aarch64}.flatpak` | Flatpak bundle: `flatpak install --user novafoods-launcher-…flatpak`. Inside the sandbox the launcher installs apps as Flatpaks on the host through `flatpak-spawn --host`; where the host has no Flatpak it falls back to each app's release tarball, unpacked and run inside the sandbox with the app's settings kept in it. Either way it can only reach the host as far as that permission allows; the AppImage, .deb and .rpm builds are the recommended way to run the launcher on Linux |
| `novafoods-launcher-<ver>-linux-{x86_64,aarch64}.tar.gz` | Plain tree (`bin/`, `share/`) for everyone else and for scripts |

RISC-V (riscv64): `novafoods-launcher-<ver>-linux-riscv64.{deb,rpm,tar.gz}`, built against Ubuntu 24.04 (needs glibc 2.39+).

The tray icon uses the StatusNotifierItem standard: KDE, Xfce, Cinnamon and most other desktops show it; on GNOME install the AppIndicator extension.

**FreeBSD** (14, x86_64)

| File | What it is |
|---|---|
| `novafoods-launcher-<ver>-freebsd-x86_64.tar.gz` | A tree laid out like `/usr/local` |

```sh
pkg install libxkbcommon wayland libX11 libXcursor libXrandr libXi libxcb mesa-libs vulkan-loader fontconfig freetype2
tar -xzf novafoods-launcher-<ver>-freebsd-x86_64.tar.gz --strip-components 1 -C /usr/local
novafoods-launcher
```

**Web (WebAssembly)**

| File | What it is |
|---|---|
| `novafoods-launcher-web-<ver>.zip` | The catalog as a static site you can host anywhere; see [`packaging/web/README.md`](packaging/web/README.md) |

## Build from source

You need a recent stable Rust (1.90 or newer). On Linux, install the windowing headers first
(Debian/Ubuntu: `sudo apt install libxkbcommon-dev libwayland-dev libx11-dev libxrandr-dev libxi-dev libgl1-mesa-dev`).

```sh
git clone https://github.com/EuLucaDev/novafoods-launcher
cd novafoods-launcher
cargo run --release -p novafoods-launcher   # the desktop app
cargo xtask ci                             # fmt, clippy, tests and the wasm check
```

The web version builds with [trunk](https://trunkrs.dev/):

```sh
rustup target add wasm32-unknown-unknown
cd apps/novafoods-launcher-web && trunk serve
```

### Installing your own builds

To install and update the Nova Foods apps from checkouts on your machine instead of GitHub, clone them
next to this repo (`../novaphoto`, `../novaword`, …) and run:

```sh
cargo xtask local sync      # git pull every app, build and package each new version into ../local-mirror
cargo xtask local serve     # serve the builds and a release feed on http://127.0.0.1:8787/
NOVAFOODS_LAUNCHER_DOWNLOAD_BASE=http://127.0.0.1:8787/ cargo run --release -p novafoods-launcher --features local-mirror
```

Run `sync` again whenever you want updates, or let the server do it: `cargo xtask local serve
--sync-every 1h` pulls and builds on that interval while it serves. An app is rebuilt when its
checkout's version isn't in the mirror yet (`--rebuild` forces it), and the launcher's next check
picks it up. Add `--every-commit` to build every new commit instead, as a dev version between
releases (`0.4.0` builds as `0.4.1-dev.<commit time>`), which the launcher offers as an update. While
`NOVAFOODS_LAUNCHER_DOWNLOAD_BASE` is set (loopback addresses only), the launcher asks only the
mirror, never GitHub. Only builds with the `local-mirror` feature read it; releases never have it. `cargo xtask local help` lists the options (`--only`, `--no-pull`, `--arch`, …).

New contributors and AI agents: start with [`AGENTS.md`](AGENTS.md). Release packaging lives in
[`packaging/`](packaging/) and [`.github/workflows/release.yml`](.github/workflows/release.yml).

> [!IMPORTANT]
> **Status:** Nova Foods Launcher is new and in early alpha. Expect rough edges, and please file
> issues (include your OS, which app you were installing and the launcher's log file) or tell us on
> [Suporte](https://workflowind.agcunae.com/inbox).

## The Nova Foods Apps

Nova Foods Launcher installs the **Nova Foods Apps**: free, open-source creative tools from the
[Nova Foods](https://workflowind.agcunae.com/apps/) team, each written from scratch in Rust and each able to
stand on its own.

| | App | What it's for | Code | Learn more |
|:-:|---|---|---|---|
| <img src="https://raw.githubusercontent.com/EuLucaDev/novaphoto/main/assets/app-icon/hicolor/64x64/apps/br.com.novafoods.novaphoto.png" alt="" width="32" height="32"> | **Nova Photo** | Image editing: layers, masks, type and real PSD files | [GitHub](https://github.com/EuLucaDev/novaphoto) | [Website](https://workflowind.agcunae.com/apps/novaphoto) |
| <img src="https://raw.githubusercontent.com/EuLucaDev/novavector/main/assets/app-icon/hicolor/64x64/apps/br.com.novafoods.novavector.png" alt="" width="32" height="32"> | **Nova Vector** | Vector illustration | [GitHub](https://github.com/EuLucaDev/novavector) | [Website](https://workflowind.agcunae.com/apps/novavector) |
| <img src="https://raw.githubusercontent.com/EuLucaDev/novafilm/main/assets/app-icon/hicolor/64x64/apps/br.com.novafoods.novafilm.png" alt="" width="32" height="32"> | **Nova Film** | Video editing, color and sound | [GitHub](https://github.com/EuLucaDev/novafilm) | [Website](https://workflowind.agcunae.com/apps/novafilm) |
| <img src="https://raw.githubusercontent.com/EuLucaDev/novalight/main/assets/app-icon/hicolor/64x64/apps/br.com.novafoods.novalight.png" alt="" width="32" height="32"> | **Nova Light** | Photo library and raw development | [GitHub](https://github.com/EuLucaDev/novalight) | [Website](https://workflowind.agcunae.com/apps/novalight) |
| <img src="https://raw.githubusercontent.com/EuLucaDev/novapdf/main/assets/app-icon/hicolor/64x64/apps/br.com.novafoods.novapdf.png" alt="" width="32" height="32"> | **Nova PDF** | Reading, organizing and protecting PDFs | [GitHub](https://github.com/EuLucaDev/novapdf) | [Website](https://workflowind.agcunae.com/apps/novapdf) |
| <img src="https://raw.githubusercontent.com/EuLucaDev/novaeffect/main/assets/app-icon/hicolor/64x64/apps/br.com.novafoods.novaeffect.png" alt="" width="32" height="32"> | **Nova Effect** | Motion graphics and visual effects | [GitHub](https://github.com/EuLucaDev/novaeffect) | [Website](https://workflowind.agcunae.com/apps/novaeffect) |
| <img src="https://raw.githubusercontent.com/EuLucaDev/novadesign/main/assets/app-icon/hicolor/64x64/apps/br.com.novafoods.novadesign.png" alt="" width="32" height="32"> | **Nova Design** | Page layout and publishing | [GitHub](https://github.com/EuLucaDev/novadesign) | [Website](https://workflowind.agcunae.com/apps/novadesign) |
| <img src="https://raw.githubusercontent.com/EuLucaDev/novafig/main/assets/app-icon/hicolor/64x64/apps/br.com.novafoods.novafig.png" alt="" width="32" height="32"> | **Nova Fig** | Interface design and prototyping | [GitHub](https://github.com/EuLucaDev/novafig) | [Website](https://workflowind.agcunae.com/apps/novafig) |
| <img src="https://raw.githubusercontent.com/EuLucaDev/novaword/main/assets/app-icon/hicolor/64x64/apps/br.com.novafoods.novaword.png" alt="" width="32" height="32"> | **Nova Word** | Word processing and documents | [GitHub](https://github.com/EuLucaDev/novaword) | [Website](https://workflowind.agcunae.com/apps/novaword) |
| <img src="https://raw.githubusercontent.com/EuLucaDev/novasound/main/assets/app-icon/hicolor/64x64/apps/br.com.novafoods.novasound.png" alt="" width="32" height="32"> | **Nova Sound** | Audio recording and editing | [GitHub](https://github.com/EuLucaDev/novasound) | [Website](https://workflowind.agcunae.com/apps/novasound) |
| <img src="https://raw.githubusercontent.com/EuLucaDev/novadeck/main/assets/app-icon/hicolor/64x64/apps/br.com.novafoods.novadeck.png" alt="" width="32" height="32"> | **Nova Deck** | Presentations and slide decks | [GitHub](https://github.com/EuLucaDev/novadeck) | [Website](https://workflowind.agcunae.com/apps/novadeck) |
| <img src="https://raw.githubusercontent.com/EuLucaDev/novacad/main/assets/app-icon/hicolor/64x64/apps/br.com.novafoods.novacad.png" alt="" width="32" height="32"> | **Nova CAD** | Computer-aided design | [GitHub](https://github.com/EuLucaDev/novacad) | [Website](https://workflowind.agcunae.com/apps/novacad) |
| <img src="https://raw.githubusercontent.com/EuLucaDev/novagrid/main/assets/app-icon/hicolor/64x64/apps/br.com.novafoods.novagrid.png" alt="" width="32" height="32"> | **Nova Grid** | Spreadsheets and data | [GitHub](https://github.com/EuLucaDev/novagrid) | [Website](https://workflowind.agcunae.com/apps/novagrid) |

And [**Nova Foods**](https://workflowind.agcunae.com/apps/) itself, our AI image and video studio for artists who want real control.

<br>

<p align="center">
  <a href="https://workflowind.agcunae.com/inbox"><img alt="Talk to the Nova Foods IT team" src="https://img.shields.io/badge/Join%20us%20on%20Suporte-5865F2?style=for-the-badge&logo=discord&logoColor=white" height="40"></a>
</p>

<h3 align="center">Come make things with us</h3>

<p align="center">
  Our Suporte is where artists of every kind hang out: people who paint, shoot, draw, cut film,
  set type, and people still figuring out what they like to make. Share what you're working on,
  ask for help, tell us what's broken, or tell us what you wish these tools could do.
  Whatever your medium and however long you've been at it, you're welcome here.
</p>

<p align="center">
  <a href="https://workflowind.agcunae.com/inbox"><b>workflowind.agcunae.com/inbox</b></a> ·
  <a href="https://workflowind.agcunae.com/apps/">workflowind.agcunae.com/apps</a> ·
  <a href="https://workflowind.agcunae.com/apps">The Nova Foods Apps</a> ·
  <a href="https://github.com/EuLucaDev/novafoods-launcher">Nova Foods Launcher</a>
</p>

---

## Thanks

The community built launchers for the Nova Foods Apps before we did, and Nova Foods Launcher learned a
lot from them. Thank you to their authors:

| Project | Author | What we learned from it |
|---|---|---|
| [Nova Foods Suite Updater](https://github.com/Lyraedan/Nova Foods-Suit-Updater) | [@Lyraedan](https://github.com/Lyraedan) | Noticing apps installed or removed outside the updater with a quiet background re-scan, detecting Arch and AUR installs through pacman, and a platform picker that lists the downloads for every OS |
| [craftcenter](https://github.com/avokadosauce/craftcenter) | [@avokadosauce](https://github.com/avokadosauce) | Finding the latest release without spending API quota (the `releases/latest` redirect plus `SHA256SUMS.txt`), and verifying every file |
| [CRAFTSPACE](https://github.com/emircesur/CRAFTSPACE) | [@emircesur](https://github.com/emircesur) | Adopting apps that were already installed, falling back gracefully when GitHub rate-limits, background update notifications |
| [GetCraft](https://github.com/mbirnbach/getcraft) | [@mbirnbach](https://github.com/mbirnbach) | Picking the right download for each OS and CPU, recognising Windows Installer and macOS installs, a native Rust/egui launcher |
| [CraftHub](https://github.com/CraftHubLauncher/CraftHub) | [CraftHubLauncher](https://github.com/CraftHubLauncher) | Strict checks on where downloads come from, clear rate-limit messages, never replacing an app while it runs |
| [novafoods-suite](https://github.com/erikerikerik/novafoods-suite) | [@erikerikerik](https://github.com/erikerikerik) | ETag-cached release checks and installs that recover cleanly from interruptions |
| [Nova Foods Apps Launcher & Installer](https://github.com/ZifuM/Craft-apps-launcher-installer) | [@ZifuM](https://github.com/ZifuM) | Release-page fallbacks and starting AppImage apps with a clean environment |
| [Nova Foods Launcher](https://github.com/EricVogt93/novafoods-launcher) | [@EricVogt93](https://github.com/EricVogt93) | Desktop integration for AppImages and a careful self-update design |
| [LandingCraft](https://github.com/rallegretti/LandingCraft) | [@rallegretti](https://github.com/rallegretti) | Working with the Windows Installer directly and offering stable releases only |
| [Nova Foods Apps Manager](https://github.com/CryptoKey98/novafoods-apps-manager) | [@CryptoKey98](https://github.com/CryptoKey98) | Scheduled background update checks and per-app launch options |
| [Nova Foods Hub Launcher](https://github.com/tinyplayerss/novafoods-hub-launcher) | [@tinyplayerss](https://github.com/tinyplayerss) | A friendly, one-window hub for the whole suite |
| [CraftHub](https://github.com/MegaTheLEGEND/CraftHub) | [@MegaTheLEGEND](https://github.com/MegaTheLEGEND) | Serving the apps' web builds from a self-hosted launcher |

## License and credits

Nova Foods Launcher is dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
Copyright (c) 2026 Nova Foods Team and the Nova Foods Launcher contributors. Required notices are in [NOTICE](NOTICE).

Bundled fonts, icons, images and other assets keep their own open licenses; each one is listed
with its author, source and license in [ATTRIBUTION.md](ATTRIBUTION.md).

The app icons of the Nova Foods apps (`assets/app-icons/`) are each app's own icon, copied unmodified
from its repository and licensed like that app (MIT OR Apache-2.0).

**The Nova Foods name, logo and "A" mark are not licensed under MIT or Apache-2.0.** They (everything
in [`docs/brand/`](docs/brand/), plus the app icon in [`assets/app-icon/`](assets/app-icon/), the
logo in the launcher's header and the "A" tray icon)
are trademarks of the Nova Foods Team and are not covered by this license. They may be used only unmodified, and only as
part of this repository and Nova Foods Launcher, under [`docs/brand/LICENSE-brand.txt`](docs/brand/LICENSE-brand.txt).
Forks and modified versions must remove them.

<p align="center">
  <a href="https://workflowind.agcunae.com/apps/"><img alt="Nova Foods" src="docs/brand/novafoods-mark.svg" width="28"></a><br>
  <sub>Made by the <a href="https://workflowind.agcunae.com/apps/">Nova Foods</a> team and community.</sub>
</p>
