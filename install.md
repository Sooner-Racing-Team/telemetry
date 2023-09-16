# Installation

It can be difficult to install dependencies by yourself. We plan to make setup scripts at a later time, but for now, please follow this installation guide!

## Dependencies

Our projects require some certain command-line utilities to flash an Arduino. You'll need Cargo on all platforms, so follow their [installation instructions](https://rustup.rs/).

### Platform-Specific

For individual platforms, utilize the following dependencies:

#### Windows

- Make sure you have [Scoop](https://scoop.sh/).
- `scoop install avr-gcc avrdude`

#### macOS

- TODO

#### Linux

- **Debian**/**Ubuntu**/etc: `sudo apt install avr-libc gcc-avr pkg-config avrdude libudev-dev build-essential`
- **Fedora**/etc: `sudo dnf install avr-libc avr-gcc pkg-config systemd-devel make automake gcc gcc-c++ kernel-devel avrdude`
- Please feel free to PR your favorite distro here.

### Tooling

Now, use Cargo to install ravedude, a tool to help with Arduino boards: `cargo +stable install ravedude`

If everything goes well, you should be done! If you're having trouble, try restarting your computer. If that doesn't work, let us know in `#telemetry`!
