[![GitHub issues](https://img.shields.io/github/release/RedisLabsModules/RedisSchema.svg)](https://github.com/RedisLabsModules/RedisSchema/releases/latest)
[![CircleCI](https://circleci.com/gh/RedisLabsModules/RedisSchema/tree/master.svg?style=svg)](https://circleci.com/gh/RedisLabsModules/RedisSchema/tree/master)

# RedisSchema
Extension modules to Redis' native data types and commands


## Getting started

```bash
cargo +nightly build --release
redis-server --loadmodule ./target/release/libredisschema.so
```

