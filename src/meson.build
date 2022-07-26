gnome.compile_resources('done',
  'done.gresource.xml',
  gresource_bundle: true,
  install: true,
  install_dir: pkgdatadir,
)

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
  'models/queryable/list.rs',
  'models/queryable/task.rs',
  'models/queryable/mod.rs',
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
  'widgets/component/content.rs',
  'widgets/component/mod.rs',
  'widgets/component/sidebar.rs',
  'widgets/factory/list.rs',
  'widgets/factory/mod.rs',
  'widgets/factory/task.rs',
  'widgets/mod.rs',
  'widgets/popover/main_menu.rs',
  'widgets/popover/mod.rs',
  'widgets/popover/new_list.rs'
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