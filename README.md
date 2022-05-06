<div align="center">
  <br>
  <img src="https://raw.githubusercontent.com/edfloreshz/do/main/data/icons/hicolor/scalable/dev.edfloreshz.Doable.svg" width="150" />
  <h1>Doable</h1>
  <a href="https://github.com/edfloreshz/doable/actions/workflows/rust.yml">
    <img src="https://img.shields.io/github/workflow/status/edfloreshz/sensei/Rust?logo=GitHub" alt="build"/>
  </a>
  <a href="https://crates.io/crates/doable">
    <img src="https://img.shields.io/crates/v/doable?label=Doable" alt="crate"/>
  </a>
   <a href="https://crates.io/crates/doable">
    <img src="https://img.shields.io/crates/d/doable" alt="downloads"/>
  </a>
</div>
<br/>

Doable is a simple to do app written in Rust using [gtk-rs](https://gtk-rs.org/) and [Relm4](https://relm4.org/), we aim to improve on the existing set of features
provided by GNOME To Do to provide the ultimate to do experience.

<div align="center">
  <img src="https://user-images.githubusercontent.com/22224438/166165400-5a523df1-b818-4172-9e05-b62662960c31.png"/>
</div>


## Install
| Platform   | Command                                 |
|------------|-----------------------------------------|
| Arch Linux | `paru -S doable`                        |
| Cargo      | `cargo instal doable`                   |
| Flatpak    | `flatpak install dev.edfloreshz.Doable` |

## To do

### Accounts

- [ ] Allow multiple providers (Google, Microsoft To Do, Microsoft Exchange, Todoist, Nextcloud)

### Lists

- [x] Show lists
- [x] Add a new list
- [ ] Delete an existing list
- [ ] Rename an existing list
- [x] Update task counters

### Smart Lists
- [ ] Inbox
- [ ] Today
- [ ] Next 7 Days
- [x] All
- [x] Starred
- [ ] Archived

### Tasks
- [x] Add a new task
- [x] Show tasks for every list
- [x] Mark a task as completed
- [ ] Delete a task
- [ ] Rename a task
- [ ] Add steps
- [ ] Add to My Day
- [x] Mark as Favorite
- [ ] Add notes

### Reminders
- [ ] Set a reminder
- [ ] Set a due date
- [ ] Set recurrence for a task

## Dependencies to build
- gtk4
- libadwaita
- pkg-config
