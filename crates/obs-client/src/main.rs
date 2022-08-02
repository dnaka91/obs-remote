use anyhow::Result;
use tonic::codegen::CompressionEncoding;

use crate::{general::general_client::GeneralClient, profiles::profiles_client::ProfilesClient};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let mut client = GeneralClient::connect("http://[::1]:50051")
        .await?
        .send_compressed(CompressionEncoding::Gzip)
        .accept_compressed(CompressionEncoding::Gzip);
    let version = client.version(()).await?.into_inner();

    println!("{:#?}", version);

    let mut client = ProfilesClient::connect("http://[::1]:50052")
        .await?
        .send_compressed(CompressionEncoding::Gzip)
        .accept_compressed(CompressionEncoding::Gzip);
    let version = client.list(()).await?.into_inner();

    println!("{:#?}", version);

    Ok(())
}

pub mod general {
    tonic::include_proto!("obs_remote.general");
}

pub mod profiles {
    tonic::include_proto!("obs_remote.profiles");
}
