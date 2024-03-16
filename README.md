# PAM Random
A PAM module that has a 1/2 chance of succeeding. Useful for testing PAM related stuff.

## Development
I created a VM to test stuff without messing up the distro I code in.
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
