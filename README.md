<div align="center">
  <br>
  <img src="https://raw.githubusercontent.com/edfloreshz/do/main/src/assets/icons/do.edfloreshz.github.svg" width="150" />
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

Do is a to-do app built for Linux with Rust and GTK, it can connect to various services and lets you manage your tasks from a single app.

Starting with [Microsoft To Do](https://todo.microsoft.com/) support and expanding to other services soon.

**WIP**

<img src="https://raw.githubusercontent.com/edfloreshz/do/main/src/assets/app.png"/>

## List of things to do

### Account
- [x] Login to your personal Microsoft account
- [ ] Logout
- [ ] Manage your account

### Lists
- [x] Show lists
- [ ] Add a new list
- [ ] Delete an existing list
- [ ] Rename an existing list

### List groups
- [ ] Show list groups
- [ ] Add a new list groups
- [ ] Delete an existing list groups
- [ ] Rename an existing list groups

### Tasks
- [x] Add a new task
- [x] Show tasks for every list
- [x] Mark a task as completed
- [ ] Delete a task
- [ ] Rename a task
- [ ] Add steps
- [ ] Add to My Day
- [ ] Add files
- [ ] Add notes

### Reminders
- [ ] Set a reminder
- [ ] Set a due date
- [ ] Set recurrence for a task

## Dependencies to build
- gtk4
- libadwaita
- pkg-config
