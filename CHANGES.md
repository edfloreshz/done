# Changes

All notable changes to this project will be documented in this file.

## [0.2.0] - 2023-09-22
- Fixed issue where async tasks would overlap the results in the UI.
- Implemented token refreshing for Microsoft To Do. 

## [0.1.9] - 2023-09-21
- Fixed a bug where the app would fail to start if migrations failed to execute.

## [0.1.8] - 2023-09-21
- Brand new UI.
- Multiple services are now supported, starting with Microsoft To Do.
- Temporarily removed smart lists.
- Updated localization.
- Updated screenshots.

## [0.1.7] - 2023-05-17
- Tasks can now set reminders by date, time and recurrence.
- Tasks pane now extends to the bottom and aligns with the task entry.
- New smart list to store completed tasks.
- Tasks now disappear when completed and are moved to Completed.

## [0.1.6] - 2023-05-17
- Tasks pane now has a title and description.
- Add to Today is now enabled.

## [0.1.5] - 2023-05-17
- Striped down design.
- More personalization options.
- Removed services, local storage is now the default.

## [0.1.4] - 2023-03-22
- Revamped design to make it look more professional.
- Tags added to tasks.
- Added French 🇫🇷 translation.


## [0.1.3] - 2023-03-16

### Added

- New look and feel.
- Task list has a new look.
- Sidebar has a welcome screen.
- Tasks can now be edited.
- Tasks can be edited before you create them.
- New preferences pane added.
- Switch the color scheme.
- Add new providers.
- New and improved icons.
- New task service providers can be added in the preferences pane.
- The app has in-app notifications.
- Added Turkish 🇹🇷 translation
- Added Spanish 🇪🇸 translation
- Added Brazilian Portuguese 🇧🇷 translation
- Moved Development banner to the bottom.
- Added support for providers using gRPC.
- Enabled logging.
- Moved app logic to a different crate.
- Implemented local service.
- Updated `Relm4` to use async components and factories.
- Added two new icons.
- Added a new screen to the sidebar to show when it's empty.
- Added two new strings to localization files.
- Missing translations in German, Italian and Portuguese.
- The app makes use of `adw::Toast` to display notifications.
- New option to include sub-tasks inside tasks

## [0.1.2] - 2022-08-22

### Added

- Implemented a new sidebar design
- Added Italian 🇮🇹 and German 🇩🇪 translation  
- Simplified list creation process.
- Added alpha state warning banner.

## [0.1.1] - 2022-07-26

### Added

- Tasks now delete correctly
- Added localization to the app
- Added translation to mexican spanish
- New list button moved to the header bar
- New app icons
- Added a main menu
- Added about dialog to the menu
- Added quit menu option
- Added a theme switcher
- Added quit menu option
- Added symbolic icons
- App icon now adapts to the selected profile
- App now adapts to different screen sizes
- Minimum size of the app reduced


## [0.1.0] - 2022-05-06

### Added

- Add lists to sidebar
- Add tasks to lists
- Mark tasks as completed
- Edit title of task
- Mark task as favorite
- Delete task
- List starred tasks
- List all tasks
