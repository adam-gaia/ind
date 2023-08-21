(import
  (
    let lock = builtins.fromJSON (builtins.readFile ./flake.lock); in
    fetchGit {
      url = "https://github.com/edolstra/flake-compat.git";
      #url = "https://github.com/edolstra/flake-compat/archive/${lock.nodes.flake-compat.locked.rev}.tar.gz";
      rev = lock.nodes.flake-compat.locked.rev;
      # sha256 = lock.nodes.flake-compat.locked.narHash;
    }
  )
  { src = ./.; }
).defaultNix

