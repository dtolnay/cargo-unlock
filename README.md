# cargo unlock

[<img alt="github" src="https://img.shields.io/badge/github-dtolnay/cargo--unlock-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/dtolnay/cargo-unlock)
[<img alt="crates.io" src="https://img.shields.io/crates/v/cargo-unlock.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/cargo-unlock)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/dtolnay/cargo-unlock/CI/master?style=for-the-badge" height="20">](https://github.com/dtolnay/cargo-unlock/actions?query=branch%3Amaster)

New Cargo versions sometimes write out a lockfile that triggers parse failures
in old Cargo versions.

```console
$  cargo +1.41.0 check
...
    Finished dev [unoptimized + debuginfo] target(s) in 0.91s

$  cargo +1.37.0 check
error: failed to parse lock file at: /git/testing/Cargo.lock

Caused by:
  invalid serialized PackageId for key `package.dependencies`
```

The `cargo unlock` subcommand simply deletes the offending lockfile so that the
old Cargo can proceed. Install by running `cargo install cargo-unlock`.

<br>

### vs `rm Cargo.lock`

`cargo unlock` has three advantages:

- It still works if run from a workspace member, where the lockfile would be
  located at the workspace root rather than the current directory.

- It works if run from a subdirectory of a crate rather than the crate root.

- If like me you use
  <code>export <a href="https://www.gnu.org/software/bash/manual/bash.html#index-FIGNORE">FIGNORE</a>=Cargo.lock</code>
  in your shell so that all autocompletes prefer Cargo.toml over Cargo.lock,
  this is easier to type.

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
