pub mod client;
pub mod error;
pub mod factory;
pub mod kv;
pub mod options;

pub mod etcdserverpb {
    tonic::include_proto!("etcdserverpb");
}

pub mod mvccpb {
    tonic::include_proto!("mvccpb");
}

pub mod authpb {
    tonic::include_proto!("authpb");
}

pub mod v3electionpb {
    tonic::include_proto!("v3electionpb");
}

pub mod v3lockpb {
    tonic::include_proto!("v3lockpb");
}