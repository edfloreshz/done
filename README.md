<div align="center">
  <br>
  <img src="https://raw.githubusercontent.com/edfloreshz/do/main/src/resources/icons/do.edfloreshz.github.svg" width="150" />
  <h1>Do</h1>
  <a href="https://github.com/edfloreshz/do/actions/workflows/rust.yml">
    <img src="https://img.shields.io/github/workflow/status/edfloreshz/sensei/Rust?logo=GitHub" alt="build"/>
  </a>
  <a href="https://crates.io/crates/do">
    <img src="https://img.shields.io/crates/v/do?label=Do" alt="crate"/>
  </a>
   <a href="https://crates.io/crates/do">
    <img src="https://img.shields.io/crates/d/do" alt="downloads"/>
  </a>
</div>

Do is a rewrite of [Gnome To Do]() in Rust using [gtk-rs]() and [Relm4](), we aim to improve on the existing set of features provided by To Do to provide the ultimate to-do experience.

<div align="center">
  <img src="https://user-images.githubusercontent.com/22224438/162361232-d3d5d5b9-11ca-45e3-9c85-3bee901828d0.png"/>
</div>


## List of things to do

### Accounts
- [ ] Allow multiple providers (Google, Microsoft To Do, Microsoft Exchange, Todoist, Nextcloud)

### Lists
- [x] Show lists
- [x] Add a new list
- [ ] Delete an existing list
- [ ] Rename an existing list

### Tasks
- [x] Add a new task
- [x] Show tasks for every list
- [ ] Mark a task as completed
- [ ] Delete a task
- [ ] Rename a task
- [ ] Add steps
- [ ] Add to My Day
- [ ] Mark as Favorite
- [ ] Add notes

### Reminders
- [ ] Set a reminder
- [ ] Set a due date
- [ ] Set recurrence for a task

## Dependencies to build
- gtk4
- libadwaita
- pkg-config
