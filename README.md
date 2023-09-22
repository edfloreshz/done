<div align="center">
  <br>
  <img src="https://raw.githubusercontent.com/edfloreshz/done/main/data/icons/dev.edfloreshz.Done.svg" width="150" />
  <h1>Done</h1>
  
  <h3>To-do lists reimagined</h3>
  <h4>A simple task management solution for seamless organization and efficiency.</h4>
  <a href="https://github.com/edfloreshz/done/actions/workflows/ci.yml">
    <img src="https://img.shields.io/github/actions/workflow/status/edfloreshz/done/ci.yml?style=for-the-badge" alt="build"/>
  </a>
  <a href="https://crates.io/crates/done">
    <img src="https://img.shields.io/crates/v/done?label=Done&style=for-the-badge" alt="crate"/>
  </a>
   <a href="https://crates.io/crates/done">
    <img src="https://img.shields.io/crates/d/done?style=for-the-badge" alt="downloads"/>
  </a>
  <br/>
  <a href="https://github.com/sponsors/edfloreshz">
    <img src="https://img.shields.io/badge/sponsor-30363D?style=for-the-badge&logo=GitHub-Sponsors&logoColor=#white"/>
  </a>
  <a href="https://matrix.to/#/%23done-chat%3Amatrix.org">
    <img src="https://img.shields.io/badge/matrix-000000?style=for-the-badge&logo=Matrix&logoColor=white"/>
  </a>
  <a href="https://github.com/edfloreshz/done">
    <img src="https://img.shields.io/badge/GitHub-100000?style=for-the-badge&logo=github&logoColor=white"/>
  </a>
  <a href="https://t.me/done_devs">
    <img src="https://img.shields.io/badge/Telegram-2CA5E0?style=for-the-badge&logo=telegram&logoColor=white"/>
  </a>
</div>

  ![Screenshot](./data/resources/screenshots/tasks.png#gh-light-mode-only)
  ![Screenshot](./data/resources/screenshots/dark.png#gh-dark-mode-only)

Our user-friendly app allows you to effortlessly consolidate your existing tasks into a single application for optimal productivity and organization.



## Installation
| Platform   | Command                                 |
|------------|-----------------------------------------|
| Flathub    | <a href="https://flathub.org/apps/details/dev.edfloreshz.Done"><img src="https://flathub.org/assets/badges/flathub-badge-en.png" width="150"/></a> |

# Build

## Dependencies to build

Cargo:
- gtk4
- libadwaita

Meson:
- cmake
- gettext
- pkg-config

Ubuntu 22.10:
```bash
sudo apt install libadwaita-1-dev libgtk-4-dev libsqlite3-dev libsecret-1-dev meson
```
Arch Linux:
```bash
sudo pacman -S libadwaita gtk4 sqlite libsecret meson
```
Fedora:
```
sudo dnf -y install libadwaita-devel gtk4-devel sqlite-devel libsecret-devel meson
```

## Debug
To enable logging set `RUST_LOG` to `info`.
```bash
RUST_LOG=info
```

To test metainfo:
```bash
gnome-software --show-metainfo=data/dev.edfloreshz.Done.metainfo.xml.in.in,icon=data/icons/dev.edfloreshz.Done.Devel.svg
```

Use absolute paths for the icon.

## Deploy
To deploy the app, head to [RELEASE.md](RELEASE.md)

Copyright and licensing
-----------------------

Copyright 2023 © Eduardo Flores

Done is released under the terms of the [Mozilla Public License v2](https://github.com/edfloreshz/done/blob/main/LICENSE)
