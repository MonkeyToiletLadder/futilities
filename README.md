# futilities

## A file utility library

This crate provides basic file utilities. Currently only detects image files (PNG, JPEG, SVG, XCF ...) by reading file headers or file information. 

## Cargo.toml

```toml
[dependencies]
futilities = "0.1.1"
```

## Usage

```rust
use std::path::Path;
use futilities::*;

fn main() {
	// Use on Path PathBuf String and str
	println!("{:?}", Path::new("image.png").is_jpg());
	println!("{:?}", String::new("image.bmp").is_bmp());
	println!("{:?}", "image.svg".is_svg());
}
```

## Not detecting files correctly?

If you suspect that this library doesnt detect a certain file type correctly email me at dylan_w2014@outlook.com with the file type your trying to detect and the file.  Or optionally if you think you know why it wouldnt be detected write why.  Thank you.
