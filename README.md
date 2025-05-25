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

## How to use ?

> [!NOTE]
> No need for a Makefile here cargo is all you need!

To build the project simply run `cargo build --release`.
You can then find the binary at `target/thumbv7em-none-eabihf/release/cp-gap-rust`.

If you want to flash the code, onto the chip connect the chipt to your computer with the given micro-usb cable.

It should now appear on the list of device connected to your computer.

> [!NOTE]
> If you are using WSL take a look at [this](https://learn.microsoft.com/windows/wsl/connect-usb). You are going to need to forward your COM ports to your WSL.
> Once installed, restart your computer, open a Windows' powershell terminal in administrator mode.
> Find the right busid using `usbipd list`, it should look like "<busid> ... ST-Link Debug, Dispositif de stockage de masse USB, STMic..."
> You will now have to bind and attach your device to your WSL, using first, `usbipd bind --busid <busid>`, and then `usbipd attach --wsl --busid <busid>`

you can run `cargo flash --release --chip STM32F429ZI`.

You can also run `cargo embed --release` which will flash and open a debug rtt terminal.

Run `cargo clean` to clean the project from generated files.

## Implementation

### Step 0 (Setup)

Chosen MCU: STM32F429ZIT6
The setup of this project was done using `cargo new cp-gap-rust`, then all the includes needed were added to the Cargo.toml.

```
    embedded-hal = "0.2.0"
    cortex-m = { version = "0.7", features = ["critical-section-single-core"] }
    cortex-m-rt = "0.6"
    stm32f4xx-hal = { version = "0.10", features = ["rt", "stm32f429"] }
    rtt-target = { version="0.3.1", features = ["cortex-m"] }
    panic-halt = "0.2.0"
    cty = "0.2.2"
    tinyrlibc = {version = "0.5.0", features = ["alloc"]}
    embedded-alloc = "0.6.0"
    critical-section = "1.1"
```

For this kind of project we also needed to setup build configuration in the form of `build.rs` and `.cargo/config.toml` which allows us to define a target for build, linkage and linker script for our binary.

For embedded projects made to run on external devices we also needed to provide a Embed.toml to explicit the device it will run on.

### Step 1 (Libgapcom)

Linkage to libgapcom.a in build.rs, build.rs is executed only at compilation does not change execution.
To use symbols present in the lib we need to create an interface in rust that allows us to call C functions from the lib. This part is implemented in `src/binding.rs` and was in part generated using Bindgen, a crate that automatically generates this kind of interface given the header files.

The idea is that we just need to declare equivalent functions in Rust with equivalent types without the body and calling them will call the lib function as they have the same symbol. Of course they must all be unsafe as C functions are unsafe.

The chosen method to read and send message from capcom is UART (UART7 in this case). The rx interupt implementation can be found at `src/interrupts.rs`, where we call capcom accept to process the sent command. And the sender impl can be found in `src/gapcom_sender.rs`, it uses UART7's tx to send the response.

### Step 2 (Logger)

Logger implementation can be found at `src/logger.rs`, it uses USART1's tx to log messages. After instanciation, a logger instance can be retrieved using the logger_instance function which will retrieve the logger and return a static ref to the value contained inside the mutex which will allow us to access the logger from anywhere in the project.

The logger and logs have 4 levels, debug, info, warning, error. And logs will only be logged if the level of the log is higher or equal to the current logger level.

Messages will be logged using this pattern: "[LEVEL] MESSAGE"

### Step
