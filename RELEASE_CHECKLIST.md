# Release Checklist

Make sure each of these things are done in sequence before a release.

- [ ] 1. `git checkout master` - make sure you are on master
- [ ] 2. Check `Cargo.toml` to make sure you are publishing the intended version
- [ ] 3. Update `docs/_data/lib.json` with the version to be published
- [ ] 3. `git push origin master` -  make sure everything is pushed
- [ ] 4. `git status` - nothing to commit, working directory clean
- [ ] 5. `cargo package` - check which files are being included in the build
- [ ] 6. `cargo test --features test` - all tests should pass
- [ ] 7. `cargo run --example runtest` - runtest should exit successfully
- [ ] 8. `cargo build --examples` - all examples should build
- [ ] 9. `cargo doc --open` - spot check the documentation for any issues
- [ ] 10. `cargo publish --dry-run` - everything should pass
- [ ] 11. `cargo publish` - this will upload the package and is NOT reversible
- [ ] 12. `git tag v1.2.3` - create a tag with the correct version
- [ ] 13. `git push origin --tags` - push the new tags to the repository
- [ ] 15. Update the version in `Cargo.toml` in master to the next version to be released
- [ ] 16. `git push origin master` - push anything new
