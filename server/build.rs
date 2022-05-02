fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Building protobufs");
    tonic_build::compile_protos("../protobufs/orderbook.protobuf")?;
    Ok(())
}
