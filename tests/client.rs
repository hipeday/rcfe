mod etcdserverpb {
    tonic::include_proto!("etcdserverpb");
}

mod mvccpb {
    tonic::include_proto!("mvccpb");
}

mod authpb {
    tonic::include_proto!("authpb");
}

mod v3electionpb {
    tonic::include_proto!("v3electionpb");
}

mod v3lockpb {
    tonic::include_proto!("v3lockpb");
}

use rcfe::ClientOptions;

#[tokio::test]
async fn test_client() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = rcfe::Client::new(ClientOptions {
        endpoints: vec![String::from("http://etcd0:2379")],
    })?;

    let kv_client = client.kv_client();
    let response = kv_client.range("greeting").await?;

    println!("{:?}", response);
    Ok(())
}