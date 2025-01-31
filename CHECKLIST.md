# Before releasing

 - check compilation success
 - update Cargo.lock: `cargo update`
 - cargo fmt --all / cargo check / cargo clippy
 - update the changelog
 - merge
 - remove the 'beta' from the version field in master branch Cargo.toml
 - create a new SIGNED tag vX.Y.Z on master: `git tag -s v8.9.5`
 - verify the signed tag: `git tag -v v8.9.5`
 - git push origin vX.Y.Z
 - bump Cargo.toml to next version on master, suffixed by 'beta'

# After release

 - Check CI status
 - Check Releases status
 - Edit release name
