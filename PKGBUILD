pkgname=grezi
pkgver=3.0.0
pkgrel=1
pkgdesc='Presentation as code'
arch=('any')
license=('gpl')
depends=(
  helix
  gtk4
)


package() {
  install -Dm755 "$srcdir/../target/release/grezi" "$pkgdir/usr/bin/grezi"
  install -Dm644 "$srcdir/../grz.xml" "$pkgdir/usr/share/mime/packages/grz.xml"
  install -Dm644 "$srcdir/../grezi.thumbnailer" "$pkgdir/usr/share/thumbnailers/grezi.thumbnailer"
}
