// importing generated gRPC code
use poh_grpc::*;
// importing types for messages
use poh::*;
mod poh;
mod poh_grpc;

use clap::Parser;
use grpc::ClientStubExt;

#[derive(Parser)]
struct Cli {
    /// host address
    host: String,
    /// host port
    port: u16,
    /// public_key to check
    public_key: String,
    /// signature to check
    signature: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let host = args.host;
    let signature = args.signature;
    let public_key = std::fs::read_to_string(args.public_key).expect("Unable to read file");
    let port = args.port;

    let mut req = VerifyAttachedSignatureRequest::new();
    let signature = std::fs::read_to_string(signature.clone()).expect("Unable to read file");

    let client = PoHClient::new_plain(&host, port, Default::default()).unwrap();
    req.set_file_attached_signature(signature);
    req.set_public_key(public_key);

    // send the request
    let resp = client
        .verify_attached_signature(grpc::RequestOptions::new(), req)
        .join_metadata_result()
        .await?;
    println!("Valid: {}, info: {}", resp.1.valid, resp.1.info);
    if !resp.1.valid {
        Err("Invalid signature")?
    }
    Ok(())
}
