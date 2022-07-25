pkgdatadir = get_option('prefix') / get_option('datadir') / meson.project_name()

conf = configuration_data()
conf.set_quoted('VERSION', meson.project_version())
conf.set_quoted('GETTEXT_PACKAGE', 'done')
conf.set_quoted('LOCALEDIR', join_paths(get_option('prefix'), get_option('localedir')))
conf.set_quoted('PKGDATADIR', pkgdatadir)
conf.set_quoted('APPLICATION_ID', application_id)
conf.set_quoted('PROFILE', get_option('profile'))

rust_sources = files(
  'app/application.rs',
  'app/config.rs',
  'app/constants.rs',
  'app/mod.rs',
  'app/window.rs',
  'core/local/lists.rs',
  'core/local/mod.rs',
  'core/local/tasks.rs',
  'core/mod.rs',
  'core/provider.rs',
  'main.rs',
  'models/list.rs',
  'models/mod.rs',
  'models/task.rs',
  'plugins/eds/mod.rs',
  'plugins/eds/parser_eds.rs',
  'plugins/eds/plugin_eds.rs',
  'plugins/eds/provider_eds.rs',
  'plugins/eds/task_eds.rs',
  'plugins/local/mod.rs',
  'plugins/local/parser_local.rs',
  'plugins/local/plugin_local.rs',
  'plugins/local/provider_local.rs',
  'plugins/local/task_local.rs',
  'plugins/mod.rs',
  'schema.rs',
  'storage/mod.rs',
  'task.rs',
  'widgets/app.rs',
  'widgets/contents/content.rs',
  'widgets/contents/details.rs',
  'widgets/contents/mod.rs',
  'widgets/contents/task_list.rs',
  'widgets/mod.rs',
  'widgets/panel/list.rs',
  'widgets/panel/mod.rs',
  'widgets/panel/sidebar.rs',
  'widgets/panel/theme_selector.rs',
  'widgets/popovers/edit_list.rs',
  'widgets/popovers/edit_task.rs',
  'widgets/popovers/main_menu.rs',
  'widgets/popovers/mod.rs',
  'widgets/popovers/new_list.rs'
)

sources = [cargo_sources, rust_sources]

cargo_script = find_program(join_paths(meson.project_source_root(), 'build-aux/cargo.sh'))
cargo_release = custom_target(
  'cargo-build',
  build_by_default: true,
  input: sources,
  output: meson.project_name(),
  console: true,
  install: true,
  install_dir: get_option('bindir'),
  command: [
    cargo_script,
    meson.project_build_root(),
    meson.project_source_root(),
    '@OUTPUT@',
    get_option('buildtype'),
    meson.project_name(),
  ]
)

subdir('app')
