
fn main() -> std::io::Result<()> {
    prost_build::compile_protos(&["src/fyers.v1.proto"], &["src/"])?;
    Ok(())
}
