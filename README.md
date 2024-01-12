warning: this is not official support.

Back up your CONFIG.BIN before trying this.


# What does it do

You can use it to change the interval settings of your BLE micro pro

# How

[install rust](https://www.rust-lang.org/tools/install)

clone this repo and `cd bmp-config`, put your CONFIG.BIN here.

run the script by running `cargo run -- P C`, the program will generate a file called `CONFIG.BIN.new` with the new settings.
Change P and M to the periperal and central interval. For example, `cargo run -- 20 20`

the slave will ignore the central interval even if you provide it.
