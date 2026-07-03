# amber

Small tools - Beautiful terminals

## Modules 

- `log`: Log messages to stdout in a structured format.

## install

```toml
[dependencies]
amber-lib = { path = "./amber-lib" }
```

Or use as a CLI:

```nix
{
  inputs.amber.url = "github:dominicegginton/amber";
  
  outputs = { amber, ... }: {
    devShells.default = {
      packages = [ amber.packages.${system}.amber-bin ];
    };
  };
}
```

## usage

```rust
use amber_lib::{Level, StdoutLogger, Logger};

let logger = StdoutLogger;
logger.log(Level::Info, "Hello", &[("key", "value")])?;
```

```bash
amber log "message" --level info --field key value
```
