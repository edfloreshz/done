# Maintainer: Eduardo Flores <edfloreshz@gmail.com>

pkgname=do-git
_pkgname=do
pkgrel=1
pkgver=0.1.1
pkgdesc="Microsoft To Do client for Linux built with Rust and GTK."
arch=('x86_64')
url="https://github.com/edfloreshz/do"
license=('GPL2')
depends=()
makedepends=('cargo' 'git')
optdepends=()
provides=('do')
conflicts=('do')
source=("do::git+https://github.com/edfloreshz/do")
md5sums=('SKIP')

prepare() {
	cd "$_pkgname"
	echo "$(git rev-list --count HEAD).$(git rev-parse --short HEAD)"
}

build() {
	cd "$_pkgname"
	cargo build --release
}

package() {
	cd "$_pkgname"
	install -Dm644 assets/res/do.desktop "$pkgdir"/usr/share/applications/do.desktop
    install -Dm644 assets/icons/do.edfloreshz.github.svg "$pkgdir"/usr/share/icons/hicolor/scalable/apps/do.svg
	install -Dm644 README.md "$pkgdir"/usr/share/doc/do/README.md
	install -Dm755 target/release/do "$pkgdir"/usr/bin/do
}
