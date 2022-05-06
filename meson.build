project('todo', 'rust',
	version:'0.1.3',
	license: ['GPL-2.0']
)

dependency('glib-2.0')
dependency('gio-2.0')
dependency('gtk4', version: '>= 4.6.0')
dependency('libadwaita-1')

i18n = import('i18n')

cargo = find_program('cargo', required: true)

cargo_sources = files(
  'Cargo.toml',
  'Cargo.lock',
)

subdir('data')
subdir('src')