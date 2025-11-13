pkgname=hyprscrolling-orchestrator
pkgver=0.1.0
pkgrel=1
pkgdesc="hyprscrolling orchestrator will manage windows with absolute keybindings"
arch=('x86_64' 'aarch64')
url="https://github.com/fibsussy/hyprscrolling-orchestrator"
license=('MIT')
depends=()
makedepends=('curl')
source=("https://github.com/fibsussy/hyprscrolling-orchestrator/releases/download/v${pkgver}/hyprscrolling-orchestrator-linux-${CARCH}.tar.gz"
        "https://raw.githubusercontent.com/fibsussy/hyprscrolling-orchestrator/main/LICENSE")
sha256sums=('SKIP'
            'SKIP')
options=('!debug')

package() {
    install -Dm755 "hyprscrolling-orchestrator" "$pkgdir/usr/bin/hyprscrolling-orchestrator"
    install -Dm644 "LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
