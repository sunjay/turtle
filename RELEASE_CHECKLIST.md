# Release Checklist

Make sure each of these things are done in sequence before a release.

- [ ] 1. `git checkout master` - make sure you are on master
- [ ] 2. `git push origin master` -  make sure everything is pushed
- [ ] 3. `cargo package` - check which files are being included in the build
- [ ] 4. `cargo test --features test` - all tests should pass
- [ ] 5. `cargo run --example runtest` - runtest should exit successfully
- [ ] 6. `cargo build --examples` - all examples should build
- [ ] 7. `cargo doc --open` - spot check the documentation for any issues
- [ ] 8. `cargo publish --dry-run` - everything should pass
- [ ] 9. `cargo publish` - this will upload the package and is NOT reversible
- [ ] 10. `git tag v1.2.3` - create a tag with the correct version
- [ ] 11. `git push origin --tags` - push the new tags to the repository
- [ ] 12. Update `_data/lib.json` on `gh-pages` branch with the latest version
- [ ] 13. Update the version in `Cargo.toml` in master to the next version to be released
- [ ] 14. `git push origin master` - push anything new
