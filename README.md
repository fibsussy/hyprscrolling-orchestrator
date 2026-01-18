# hyprscrolling-orchestrator 🚀

simple tool to allow you to focus and move hyprscroller with absolute binds
so you dont have to move relatively all the time


## 🔧 Installation

## 🔧 Installation

### One-Line Install (Arch Linux)

**Precompiled binary (default, fast):**
```bash
curl -fsSL https://raw.githubusercontent.com/fibsussy/hyprscrolling-orchestrator/main/install.sh | bash
```

**Or build from source:**
```bash
curl -fsSL https://raw.githubusercontent.com/fibsussy/hyprscrolling-orchestrator/main/install.sh | bash -s local
```

**Note:** For security, inspect the install script before running it. View it [here](https://github.com/fibsussy/hyprscrolling-orchestrator/blob/main/install.sh).

### Manual Installation

#### Prerequisites

Add yourself to the `input` group:
```bash
sudo usermod -a -G input $USER
# Log out and log back in for changes to take effect
```

#### From Source

```bash
# Clone and build
git clone https://github.com/fibsussy/hyprscrolling-orchestrator.git
cd hyprscrolling-orchestrator
cargo build --release

# Install
sudo cp target/release/hyprscrolling-orchestrator /usr/bin/
```

### Post-Installation Setup

1. **Copy the example config:**
```bash
mkdir -p ~/.config/hyprscrolling-orchestrator
cp /usr/share/doc/hyprscrolling-orchestrator/config.example.ron ~/.config/hyprscrolling-orchestrator/config.ron
```

2. **Edit your config:**
```bash
$EDITOR ~/.config/hyprscrolling-orchestrator/config.ron
```

3. **Select which keyboards to enable:**
```bash
hyprscrolling-orchestrator toggle
```

**Or build from source:**
```bash
curl -fsSL https://raw.githubusercontent.com/fibsussy/hyprscrolling-orchestrator/main/install.sh | bash -s local
```

**Note:** For security, inspect the install script before running it. View it [here](https://github.com/fibsussy/hyprscrolling-orchestrator/blob/main/install.sh).

### Manual Installation (from source)

```bash
# Clone the repository
git clone https://github.com/fibsussy/hyprscrolling-orchestrator.git
cd hyprscrolling-orchestrator

# Build and install
cargo build --release
sudo cp target/release/hyprscrolling-orchestrator /usr/local/bin/
```
