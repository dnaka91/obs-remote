Vagrant.configure("2") do |config|
  config.vm.box = "ubuntu/focal64"

  config.vm.provider "virtualbox" do |vb|
    vb.cpus = 8
    vb.memory = 4096
  end

  config.vm.provision "shell", inline: <<-SHELL
  	set -x
    apt-get update
    apt-get dist-upgrade -y
    apt-get install -y build-essential clang libobs-dev libx11-dev pkg-config
    apt-get install -y libssl-dev # needed for cargo-edit
  SHELL

  config.vm.provision "shell", privileged: false, inline: <<-SHELL
    set -x
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env

    rustup component add llvm-tools-preview
    rustup install nightly
    cargo install cargo-binutils
    cargo install cargo-edit

    cd /vagrant && cargo check
  SHELL
end
