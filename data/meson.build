application_id_static = 'dev.edfloreshz.Done'

configure_file(
    input: 'Cargo.toml.in',
    output: 'Cargo.toml',
    configuration: conf
)

# Copy the Cargo.toml output to the source directory.
run_command(
  'cp',
  join_paths(meson.project_build_root(), 'data', 'Cargo.toml'),
  join_paths(meson.project_source_root(), 'Cargo.toml'),
  check: true
)

desktop_data = configuration_data()
desktop_data.set('APPLICATION_ID', application_id)
desktop_file = i18n.merge_file(
  input: configure_file(
  	input: 'dev.edfloreshz.Done.desktop.in.in',
  	output: 'dev.edfloreshz.Done.desktop.in',
  	configuration: desktop_data,
  ),
  output: '@0@.desktop'.format(application_id),
  type: 'desktop',
  po_dir: '../po',
  install: true,
  install_dir: get_option('datadir') / 'applications'
)

desktop_utils = find_program('desktop-file-validate', required: false)
if desktop_utils.found()
  test('Validate desktop file',
    desktop_utils,
    args: [desktop_file],
    suite: ['lint'],
  )
endif

appstream_data = configuration_data()
appstream_data.set('APPLICATION_ID', application_id)
appstream_file = i18n.merge_file(
  input: configure_file(
  	input: 'dev.edfloreshz.Done.appdata.xml.in.in',
  	output: 'dev.edfloreshz.Done.appdata.xml.in',
  	configuration: appstream_data,
  ),
  output: '@0@.appdata.xml'.format(application_id),
  po_dir: '../po',
  install: true,
  install_dir: get_option('datadir') / 'appdata'
)

appstream_util = find_program('appstream-util', required: false)
if appstream_util.found()
  test('Validate appstream file', appstream_util,
    args: ['validate', appstream_file]
  )
endif

gschema_data = configuration_data()
gschema_data.set('APPLICATION_ID', application_id)
gschema_file = i18n.merge_file(
  input: configure_file(
  	input: 'dev.edfloreshz.Done.gschema.xml.in.in',
  	output: 'dev.edfloreshz.Done.gschema.xml.in',
  	configuration: gschema_data,
  ),
  output: '@0@.gschema.xml'.format(application_id),
  po_dir: '../po',
  install: true,
  install_dir: get_option('datadir') / 'glib-2.0' / 'schemas'
)

compile_schemas = find_program('glib-compile-schemas', required: false)
if compile_schemas.found()
  test('Validate schema file', compile_schemas,
    args: ['--strict', '--dry-run', meson.current_source_dir()]
  )
endif

metainfo_data = configuration_data()
metainfo_data.set('APPLICATION_ID', application_id)
metainfo_file = i18n.merge_file(
  input: configure_file(
  	input: 'dev.edfloreshz.Done.metainfo.xml.in.in',
  	output: 'dev.edfloreshz.Done.metainfo.xml.in',
  	configuration: metainfo_data,
  ),
  output: '@0@.metainfo.xml'.format(application_id),
  po_dir: '../po',
  install: true,
  install_dir: get_option('datadir') / 'metainfo'
)

subdir('icons')
