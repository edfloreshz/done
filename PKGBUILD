# Maintainer: Eduardo Flores <edfloreshz@gmail.com>

pkgname=done-git
pkgrel=1
pkgver=0.1.0
pkgdesc="Done is a simple to do app written in GTK and Rust."
arch=('x86_64')
url="https://github.com/edfloreshz/done"
license=('GPL2')
depends=('gtk4' 'libadwaita' 'pkg-config')
makedepends=('cargo' 'git')
optdepends=()
provides=('done')
conflicts=('done')
source=("done-git::git+https://github.com/edfloreshz/done#branch=main")
md5sums=('SKIP')

prepare() {
	cd "$pkgname"
	echo "$(git rev-list --count HEAD).$(git rev-parse --short HEAD)"
}

build() {
	cd "$pkgname"
	cargo build --release
}

package() {
	cd "$pkgname"
	install -Dm644 data/dev.edfloreshz.Done.desktop "$pkgdir/usr/share/applications/dev.edfloreshz.Done.desktop"
	install -Dm644 data/dev.edfloreshz.Done.svg "$pkgdir/usr/share/icons/hicolor/scalable/apps/dev.edfloreshz.Done.svg"
	install -Dm664 data/dev.edfloreshz.Done.metainfo.xml "$pkgdir/usr/share/metainfo/dev.edfloreshz.Done.metainfo.xml"
	install -Dm644 README.md "$pkgdir/usr/share/doc/done/README.md"
	install -Dm755 target/release/done "$pkgdir/usr/bin/done"
}
