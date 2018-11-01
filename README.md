# Substrate_first_steps
 Let's start with the basic steps from the [Rust
 tutorial](https://doc.rust-lang.org/book/2018-edition/index.html) to set up the
 requisites concerning Rust:
## Install
- Install rustup (choose default PATH if you have no idea what you are doing) ->
Just press ENTER.
``` curl https://sh.rustup.rs -sSf | sh ```
- Check if Rust has been installed correctly:
```
source $HOME/.cargo/env
rustc --version
```
Add the following line to your ~/.bash_profile:
By adding this your OS will know where to find the rust environment.
``` export PATH="$HOME/.cargo/bin:$PATH" ```
