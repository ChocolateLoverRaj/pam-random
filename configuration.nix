# To test out pam-random on NixOS run
# sudo nixos-rebuild test -I nixos-config=./configuration.nix
# Then you can use pamtester:
# nix-shell -p pamtester --command "pamtester pam-random $USER authenticate"
{ pkgs, ... }:
let
  pam-random = with pkgs; stdenv.mkDerivation
    {
      name = "pam-random";
      src = ./.;
      dontUnpack = true;
      installPhase = ''
        mkdir -p $out/lib64/security
        cp $src/target/debug/libpam_random.so $out/lib64/security
      '';
    };
in
{
  imports = [
    /etc/nixos/configuration.nix
  ];

  environment.systemPackages = with pkgs; [
    pam-random
  ];

  security.pam.services."pam-random".text = ''
    auth sufficient ${pam-random}/lib64/security/libpam_random.so 1000
    account sufficient ${pam-random}/lib64/security/libpam_random.so
  '';
}
