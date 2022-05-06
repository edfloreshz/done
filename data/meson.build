datadir = get_option('prefix') / get_option('datadir')

application_id = 'org.devloop.Do'

install_data(
    '@0@.desktop'.format(application_id),
    install_dir: datadir / 'applications'
)

install_data(
    '@0@.metainfo.xml'.format(application_id),
    install_dir: datadir / 'metainfo'
)

install_data(
    '@0@.svg'.format(application_id),
    install_dir: datadir / 'icons' / 'hicolor' / 'scalable' / 'apps'
)