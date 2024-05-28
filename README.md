# Ghost Text
## Be like a ghost online

Nothing is "free" these days in online space. Do you using some "ordinary" social networks to communicate with friends/family free? No because you probably paying with your:
1. Connection (site is going popular)
2. Caution (You watching what the site decide you to watch)
3. Privacy (site is probably harvesting some informations about you)

This tool is created in intend of lowering the amount of data you share with social network platform while utilizing network resources without loosing flexibility to use the network. 
In easy words: Let's use resources and pay nothing. Because it's time to revenge that middle finger pointing to us.

Tool has simple GUI and needs a "Secret Key" to encrypt/decrypt messages using AES-256 encryption provided by Crates inside of source files.
Mainly created for Linux, but will work also in Mac. Because Nobody cares about Windows.

For now there is only a source, so you will need to build and run it by yourself.
Instructions are here:
1. Firsty download and install Rust (if you do not have it) using official [Rust Homepage](https://www.rust-lang.org/tools/install).
2. Clone code from this repository using `git clone` command.
3. use `cd` command to move inside of this repository (directory name is: "ghost-text")
4. Build project using the command bellow (This will build software as "Release version")
   ```
   cargo build --release
   ```
After that executable file will be located in `target/release` directory of the repository.
