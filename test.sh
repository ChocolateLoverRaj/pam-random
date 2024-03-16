IP=192.168.124.164

cargo build
scp ./pam-random root@$IP:/etc/pam.d
scp ./target/debug/libpam_random.so root@$IP:/lib64/security

# To test
# ssh, then run
# pamtester pam-random <user> authenticate
