# PAM Random
A PAM module that has a 1/2 chance of succeeding. Useful for testing PAM related stuff.

## Args
```
auth sufficient libpam_random.so [delay_ms]
```
You can optionally specify a delay in milliseconds.

## Development
### NixOS
- Install [`nix-direnv`](https://github.com/nix-community/nix-direnv)
- Run `direnv allow`. This should setup Rust and other dependencies needed
- Run `cargo b`
- Follow instructions in `configuration.nix`

### VM
To test stuff without messing up the distro I code in.
- Create a Fedora VM (can probably be any distro)
- Create a user named `test`
- Enable SSH server
- Enable root password
- Enable root SSH
- Setup password-less SSH login
- Update the `IP` variable in `test.sh`
- Run `bash ./test.sh`
- Inside the VM install `pamtester`
- Inside the VM run `pamtester pam-random test authenticate`
