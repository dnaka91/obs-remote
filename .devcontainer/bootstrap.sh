#!/usr/bin/env bash
set -x

HOME=/home/vscode

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

cat > $HOME/.cargo/config.toml <<-EOF
    [target.x86_64-unknown-linux-gnu]
    rustflags = ["-C", "link-arg=-fuse-ld=lld"]
EOF

echo "export PATH=$HOME/.cargo/bin:$PATH" >> $HOME/.bashrc
mkdir -p $HOME/.config/fish
echo "set -p fish_user_paths $HOME/.cargo/bin" >> $HOME/.config/fish/config.fish
