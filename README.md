# simplerr
Simple errors in Rust

**This is a very early version and anything, including matching syntax can change with little or no warning** 

## Usage 

```toml
[dependancies]
simpl = {git = "https://github.com/durch/simplerr"}
```

```rust
use std::fs;
use simpl::err;

err!(ExampleError,
    {
        Io@std::io::Error;
    });

fn main() -> Result<()> {
    fs::create_dir("test")?;
    fs::remove_dir_all("test")?;
    Ok(())
}
```
