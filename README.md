# RustOS

## Build a freestanding executable
- install rust
- run < rustup target add thumbv7em-none-eabihf > in terminal/command prompt
    - compile for this bare metal environment thumbv7em-none-eabihf which describes an embedded ARM system
- run < cargo build --target thumbv7em-none-eabihf > to cross compile