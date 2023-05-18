## Compile using Cargo Lambda

https://www.cargo-lambda.info/

```
cd rust_dev_preview/cross_service/photo_asset_management/
cargo lambda build --release --arm64
zip ../../target/lambda/pam/bootstrap{.zip,}
```
