# advent-of-code

## Add workspace

1. Run this CLI to create a new day package
```sh
npx hygen day new
```

2. Add the new workspace member to the root Cargo.toml
```toml
...
[workspace]
members = [
  "packages/utils",
  "packages/day-1",
  "packages/day-X"
]
...
```

3. Run the code
```sh
cargo run -p day-X
```

3. Run tests
```sh
cargo test -p day-X
```

## Cheat sheet

### Run all tests
```sh
cargo test
```

### Run utils tests
```sh
cargo test -p utils
```
