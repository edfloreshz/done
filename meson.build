project('doable', 'rust',
          version: '0.1.0',
    meson_version: '>= 0.59.0',
  default_options: [ 'warning_level=2'],
)

i18n = import('i18n')

gnome = import('gnome')

dependency('glib-2.0')
dependency('gio-2.0')
dependency('gtk4', version: '>= 4.6.0')
dependency('libadwaita-1')


cargo_sources = files(
  'Cargo.toml',
  'Cargo.lock',
)

subdir('data')
subdir('src')
subdir('po')

gnome.post_install(
  glib_compile_schemas: true,
  gtk_update_icon_cache: true,
  update_desktop_database: true,
)
