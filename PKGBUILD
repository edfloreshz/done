# Maintainer: Eduardo Flores <edfloreshz@gmail.com>

pkgname=doable-git
pkgrel=1
pkgver=0.1.0
pkgdesc="Doable is a simple to do app written in GTK and Rust."
arch=('x86_64')
url="https://github.com/edfloreshz/doable"
license=('GPL2')
depends=('gtk4' 'libadwaita' 'pkg-config')
makedepends=('cargo' 'git')
optdepends=()
provides=('doable')
conflicts=('doable')
source=("doable-git::git+https://github.com/edfloreshz/doable#branch=main")
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
	install -Dm644 data/dev.edfloreshz.Doable.desktop "$pkgdir/usr/share/applications/dev.edfloreshz.Doable.desktop"
	install -Dm644 data/dev.edfloreshz.Doable.svg "$pkgdir/usr/share/icons/hicolor/scalable/apps/dev.edfloreshz.Doable.svg"
	install -Dm664 data/dev.edfloreshz.Doable.metainfo.xml "$pkgdir/usr/share/metainfo/dev.edfloreshz.Doable.metainfo.xml"
	install -Dm644 README.md "$pkgdir/usr/share/doc/doable/README.md"
	install -Dm755 target/release/doable "$pkgdir/usr/bin/doable"
}
