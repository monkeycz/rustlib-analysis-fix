# rustlib-analysis-fix

Try to resolve this [issue](https://github.com/rust-lang-nursery/rls/issues/227) temporary.

## Installation

* Clone or Download this project
* Build it, `rustup run nightly cargo build --release`

## Usage
* Backup your rustlib analysis folder first
* Run it, `rustlib-analysis-fix <rust_src_path> <rust_analysis_path>`

### e.g (macOS, nightly)
`rustlib-analysis-fix /Users/<Username>/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/src /Users/<Username>/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/analysis`
