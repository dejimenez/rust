fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("build .proto");
    tonic_build::compile_protos("proto/hello.proto")?;
    Ok(())
}