# Ghost Text
## Be like a ghost online!

Nothing is "free" these days in online space. Do you using some of the famous social networks to communicate with friends/family? Well, congratulation, you pay with your personal data and privacy!

Did you have a moment when you would like to use a privacy-frendly social network, but realized nobody of your friends is there? Inconvenient indead.
This encryption software is created in intend of blocking the amount of data you share with social network platform while perserve flexibility of usege of the social network.

In other words: Let's use resources and benefits of these social networks and pay nothing, not even a byte of your privacy. Because i think it is a good time to stop getting piss on our heads and be told it's raining.

Tool has simple GUI wrote in Slint and needs a "Secret Key" to encrypt/decrypt messages using AES-256 encryption provided by encryption crates inside of source files.
Mainly created for Linux, but will work also in Mac. Because Nobody cool boys do not use Windows and also libraries used in this code are not compatible with Windows directory trees.


Build from source instructions:
1. Firsty download and install Rust (if you do not have it already) using official [Rust Homepage](https://www.rust-lang.org/tools/install).
2. Clone code from this repository using command `git clone` + repository URL.
3. Open terminal in the cloned directory (name: ghost-text)
4. Build project using the command bellow (This will build software as "Release" version)
   ```
   cargo build --release
   ```

After that executable file will be located in `target/release` directory of the repository.

*Or just run debug version using
   ```
   cargo run
   ```
for convenience. This wont build anything, but just execute the code.


### Pre-built version can be executed as bellow
1. Firsty download released binary file from this repository
2. Open terminal in the directory where you downloaded the file
3. Execute commands bellow:
   ```
   chmod u+x GhostText
   ./GhostText 
   ```



Sample Video here → https://github.com/user-attachments/assets/ec8b203c-808b-4144-b711-b65a04759a14

