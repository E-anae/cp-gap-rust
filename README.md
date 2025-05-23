# cp-gap-rust

Bonus step for GAP project

## Installation and setup

### Linux / WSL / macOs

install cargo and rustup

`curl https://sh.rustup.rs -sSf | sh`

install target for project

`rustup target add thumbv7em-none-eabihf`

install probe-rs

`curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh`

> [!NOTE]
> you may need to run `probe-rs complete install` to complete the installation

By default, the debug probes are only accessible by users with root privileges on Linux based systems. It is recommend to use appropriate udev rules to allow users without root privileges access to the debug probes as well.

Download the [rules file](https://probe.rs/files/69-probe-rs.rules) and place it in /etc/udev/rules.d.
Run `sudo udevadm control --reload` to ensure the new rules are used.
Run `sudo udevadm trigger` to ensure the new rules are applied to already added devices.

If you’re still unable to access the debug probes after following these steps, try adding your user to the plugdev group.

> [!NOTE] > _"If you are using WSL, you may need to enable the udev service. To check if the service is running, run service udev status. If the service is not started, edit /etc/wsl.conf (with sudo) and make sure the following is included:"_

```
[boot]
command="service udev start"
```

### Windows

install cargo and rustup

[Download rustup-init.exe (32-bit)](https://static.rust-lang.org/rustup/dist/i686-pc-windows-msvc/rustup-init.exe)

[Download rustup-init.exe (64-bit)](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe)

install target for project

`rustup target add thumbv7em-none-eabihf`

install probe-rs

`irm https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.ps1 | iex`
