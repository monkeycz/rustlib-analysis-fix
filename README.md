# rustlib-analysis-fix

Try to reslove this [issue](https://github.com/rust-lang-nursery/rls/issues/227) temporary.

## Installation

* Clone or Download this project
* Build it, `rustup run nightly cargo build --release`

## Usage
`rustlib-analysis-fix <rust_src_path> <rust_analysis_path>`

### E.g (macOS)
`rustlib-analysis-fix /Users/<Username>/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/src /Users/<Username>/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/analysis`
