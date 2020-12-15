#image-base64-rs

<p align="left">
    <a href="https://crates.io/crates/image-base64-wasm">
        <img src="https://img.shields.io/crates/v/image-base64-wasm.svg"
             alt="crates">
    </a>    
</p>

Convert image to base64, and vise versa

## Compatibility

The library is compatible with **wasm**

To build into wasm, uncomment the target line in `.cargo/config`

```
[build]
# target = "wasm32-unknown-unknown"
```

## Code Example

```
extern crate image_base64_wasm;

fn main() {
  let base64 = "base64 String";
  let image = image_base64_wasm::from_base64(base64);
  
  let image_path = "local image file path"
  let base64 = image_base64_wasm::to_base64(image_path); 
}
```

## Installation

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
image-base64-wasm = "0.1"
```

## License

MIT
