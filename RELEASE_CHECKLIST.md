# Release Checklist

Make sure each of these things are done in sequence before a release.

- [ ] 1. `git checkout master` and `git pull` - make sure you are on master and have the latest changes
- [ ] 2. Check `Cargo.toml` to make sure you are publishing the intended version
- [ ] 3. Update `Unreleased` section in `CHANGELOG.md` to list version being published and update links
- [ ] 4. Update `docs/_data/lib.json` with the version to be published
- [ ] 5. `git push origin master` -  make sure everything is pushed
- [ ] 6. `git status` - nothing to commit, working directory clean
- [ ] 7. `cargo package` - check which files are being included in the build
- [ ] 8. `cargo test --features "test unstable"` - all tests should pass
- [ ] 9. `cargo run --example runtest` - runtest should exit successfully
- [ ] 10. `cargo build --examples` - all examples should build
- [ ] 11. `cargo doc --open` - spot check the documentation for any issues
- [ ] 12. `cargo publish --dry-run` - everything should pass
- [ ] 13. `cargo publish` - this will upload the package and is NOT reversible
- [ ] 14. `git tag v1.2.3` - create a tag with the correct version
- [ ] 15. `git push origin --tags` - push the new tags to the repository
- [ ] 17. Update the version in `Cargo.toml` in master to the next version to be released
- [ ] 18. Add a new `Unreleased` section to `CHANGELOG.md` with relevant section headings from https://keepachangelog.com
- [ ] 19. `git push origin master` - push anything new
