use anyhow::Result;
use tonic::codegen::CompressionEncoding;

use crate::{
    general::general_service_client::GeneralServiceClient,
    profiles::profiles_service_client::ProfilesServiceClient,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let mut client = GeneralServiceClient::connect("http://[::1]:50051")
        .await?
        .send_compressed(CompressionEncoding::Gzip)
        .accept_compressed(CompressionEncoding::Gzip);
    let version = client
        .version(general::VersionRequest {})
        .await?
        .into_inner();

    println!("{:#?}", version);

    let mut client = ProfilesServiceClient::connect("http://[::1]:50052")
        .await?
        .send_compressed(CompressionEncoding::Gzip)
        .accept_compressed(CompressionEncoding::Gzip);
    let version = client.list(profiles::ListRequest {}).await?.into_inner();

    println!("{:#?}", version);

    Ok(())
}

pub mod general {
    #![allow(clippy::derive_partial_eq_without_eq)]
    tonic::include_proto!("obs_remote.general");
}

pub mod profiles {
    #![allow(clippy::derive_partial_eq_without_eq)]
    tonic::include_proto!("obs_remote.profiles");
}
