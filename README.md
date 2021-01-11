# image-base64-wasm

<p align="left">
    <a href="https://crates.io/crates/image-base64-wasm">
        <img src="https://img.shields.io/crates/v/image-base64-wasm.svg"
             alt="crates">
    </a>    
</p>

Convert jpeg, png, gif, ico to base64, and vise versa

## Compatibility

The library is compatible with **wasm**

To build into wasm, uncomment the target line in `.cargo/config`

```
[build]
# target = "wasm32-unknown-unknown"
```

## Code Example

- Read from a file

```rust
extern crate image_base64_wasm;

fn main() {
  let base64 = "base64 String";
  let image = image_base64_wasm::from_base64(base64);
  
  let image_path = "local image file path"
  let base64 = image_base64_wasm::to_base64(image_path); 
}
```

- From `Vec<u8>` (useful for url response body)

```rust
extern crate image_base64_wasm;

fn main() {
  let base64 = "base64 String";
  let image = image_base64_wasm::from_base64(base64);
  
  let img_data: Vec<u8> = ...; // TODO replace this part
  let base64 = image_base64_wasm::vec_to_base64(img_data); 
}
```

## Installation

Add the dependency to your `Cargo.toml` under `[dependencies]`:

```toml
image-base64-wasm = "0.5.0"
```

## License

MIT
