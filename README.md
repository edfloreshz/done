<div align="center">
  <br>
  <img src="https://raw.githubusercontent.com/edfloreshz/done/main/data/icons/dev.edfloreshz.Done.svg" width="150" />
  <h1>Done</h1>
  <h3>To-do lists reimagined</h3>
  <h4>The ultimate task management solution for seamless organization and efficiency.</h4>
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
  <a href="https://matrix.to/#/#done-devs:matrix.org">
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

Our user-friendly app allows you to effortlessly consolidate your existing task providers into a single application for optimal productivity and organization.

#### **This is still in very early development. Be aware it is a work in progress and far from complete yet.**


## Installation
| Platform   | Command                                 |
|------------|-----------------------------------------|
| Arch Linux | `paru -S done-git`                    |
| Flathub    | <a href="https://flathub.org/apps/details/dev.edfloreshz.Done"><img src="https://flathub.org/assets/badges/flathub-badge-en.png" width="150"/></a> |

## Plugins
In order to realize its full potential, the Done app has been designed with a strong focus on versatility, flexibility and extensibility.

This is why we have implemented a plugin system, allowing for the addition of new task services, making it the go-to choice for 
anyone looking for a comprehensive and complete to-do list solution.

To get started creating plug-ins, head to [`PLUGINS.md`](PLUGINS.md).

## To do

### Accounts

- [ ] Allow multiple providers (Google, Microsoft To Do, Microsoft Exchange, Todoist, Nextcloud)

### Lists

- [x] Show lists
- [x] Add a new list
- [x] Delete an existing list
- [x] Rename an existing list
- [x] Update task counters

### Smart Lists
- [x] Today
- [x] Next 7 Days
- [x] All
- [x] Starred

### Tasks
- [x] Add a new task
- [x] Show tasks for every list
- [x] Mark a task as completed
- [x] Delete a task
- [x] Rename a task
- [ ] Add steps
- [ ] Add tags
- [ ] Add to My Day
- [x] Mark as Favorite
- [x] Add notes

### Reminders
- [x] Set a reminder date
- [ ] Set a reminder time
- [x] Set a due date
- [ ] Set recurrence for a task

### Notifications
- [ ] Send notifications

### Backups
- [ ] Export tasks

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
sudo apt install libadwaita-1-dev libgtk-4-dev protobuf-compiler
```
Arch Linux:
```bash
sudo pacman -S libadwaita gtk4 protobuf
```
Fedora:
```
sudo dnf -y install libadwaita-devel gtk4-devel protobuf protobuf-compiler protobuf-devel
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

Copyright 2022 © Eduardo Flores

Done is released under the terms of the [Mozilla Public License v2](https://github.com/edfloreshz/done/blob/main/LICENSE)
