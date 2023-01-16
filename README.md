# cargo-lib

Strongly-typed Cargo instructions for your build script.

## Usage
Add cargo-lib as a build dependency to be able to use it in your build.rs file:
```toml
[build-dependencies]
cargo-lib = "0.1"
```

Then in your build script:
```rust
use cargo_lib as cl;

fn main() {
    cl::warning("Running the build script");
    cl::rerun_if_env_changed("CC");
    // you can also use `None` for the second argument
    cl::rustc_link_search("/some/path", cl::SearchLibKind::Framework);
    cl::rustc_link_lib("MyFramework", cl::LibKind::Framework);
    cl::rustc_link_lib("c++", None);
}
```