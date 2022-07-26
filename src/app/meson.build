configure_file(
    input: 'constants.rs.in',
    output: 'constants.rs',
    configuration: conf
)

# Copy the constants.rs output to the source directory.
run_command(
  'cp',
  join_paths(meson.project_build_root(), 'src/app', 'constants.rs'),
  join_paths(meson.project_source_root(), 'src/app', 'constants.rs'),
  check: true
)
