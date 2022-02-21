use anyhow::Result;

use crate::{
    general::general_client::GeneralClient,
    v5::profiles::profiles_client::ProfilesClient as ProfilesClientV5,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let mut client = GeneralClient::connect("http://[::1]:50051")
        .await?
        .send_gzip()
        .accept_gzip();
    let version = client.get_version(()).await?.into_inner();

    println!("{:#?}", version);

    let mut client = ProfilesClientV5::connect("http://[::1]:50052")
        .await?
        .send_gzip()
        .accept_gzip();
    let version = client.list(()).await?.into_inner();

    println!("{:#?}", version);

    Ok(())
}

pub mod general {
    tonic::include_proto!("obs_remote.general");
}

pub mod v5 {
    pub mod profiles {
        tonic::include_proto!("obs_remote.v5.profiles");
    }
}
