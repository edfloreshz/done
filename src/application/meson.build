configure_file(
    input: 'config.rs.in',
    output: 'config.rs',
    configuration: conf
)

# Copy the config.rs output to the source directory.
run_command(
  'cp',
  join_paths(meson.project_build_root(), 'src/app', 'config.rs'),
  join_paths(meson.project_source_root(), 'src/app', 'config.rs'),
  check: true
)
