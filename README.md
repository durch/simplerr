# simplerr
Simple errors in Rust

## Usage 

```toml
[dependancies]
simpl = {git = "https://github.com/durch/simplerr"}
```

```rust
use std::fs;

use simpl::err;

err!(ExampleError);
from!(std::io::Error);

fn main() -> Result<()> {
  fs::create_dir("test")?;
  Ok(())
}

```
