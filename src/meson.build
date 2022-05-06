pkgdatadir = get_option('prefix') / get_option('datadir') / meson.project_name()

conf = configuration_data()
conf.set_quoted('VERSION', meson.project_version())
conf.set_quoted('GETTEXT_PACKAGE', 'doable')
conf.set_quoted('LOCALEDIR', join_paths(get_option('prefix'), get_option('localedir')))
conf.set_quoted('PKGDATADIR', pkgdatadir)
conf.set_quoted('APPLICATION_ID', application_id)
conf.set_quoted('PROFILE', get_option('profile'))

configure_file(
    input: 'const.rs.in',
    output: 'const.rs',
    configuration: conf
)

# Copy the const.rs output to the source directory.
run_command(
  'cp',
  join_paths(meson.project_build_root(), 'src', 'const.rs'),
  join_paths(meson.project_source_root(), 'src', 'const.rs'),
  check: true
)

rust_sources = files(
  'const.rs',
  'config.rs',
  'main.rs',
  'schema.rs',
  'core/local/lists.rs',
  'core/local/tasks.rs',
  'core/local/mod.rs',
  'core/mod.rs',
  'models/list.rs',
  'models/task.rs',
  'models/mod.rs',
  'storage/database.rs',
  'storage/mod.rs',
  'widgets/app.rs',
  'widgets/content.rs',
  'widgets/details.rs',
  'widgets/list.rs',
  'widgets/mod.rs',
  'widgets/sidebar.rs',
  'widgets/task.rs',
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
