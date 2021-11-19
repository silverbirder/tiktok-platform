// use googapis::{
//     google::storagetransfer::v1::{
//         storage_transfer_service_client::StorageTransferServiceClient,
//         transfer_spec::DataSink::GcsDataSink, transfer_spec::DataSource::HttpDataSource,
//         CreateTransferJobRequest, GcsData, HttpData, TransferJob, TransferSpec,
//     },
//     CERTIFICATES,
// };
// use gouth::Token;
// use tonic::{
//     metadata::MetadataValue,
//     transport::{Certificate, Channel, ClientTlsConfig},
//     Request,
// };

// use crate::parser::Input;

// use std::env;

// #[tokio::main]
// pub async fn transfer(data: Input) -> Result<(), Box<dyn std::error::Error>> {
//     let tls_config = ClientTlsConfig::new()
//         .ca_certificate(Certificate::from_pem(CERTIFICATES))
//         .domain_name("storagetransfer.googleapis.com");

//     let channel = Channel::from_static("https://storagetransfer.googleapis.com")
//         .tls_config(tls_config)?
//         .connect()
//         .await?;

//     let token = Token::new()?;
//     let mut service =
//         StorageTransferServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
//             let token = &*token.header_value().unwrap();
//             let meta = MetadataValue::from_str(token).unwrap();
//             req.metadata_mut().insert("authorization", meta);
//             Ok(req)
//         });

//     let project = env::var("GCP_PROJECT").expect("GCP_PROJECT must be set");
//     let transfer_job = Some(TransferJob {
//         name: String::from("transferJobs/transfer"),
//         description: String::from("description"),
//         project_id: project,
//         transfer_spec: Some(TransferSpec {
//             object_conditions: None,
//             transfer_options: None,
//             data_sink: Some(GcsDataSink(GcsData {
//                 bucket_name: String::from("xxx-tmp"),
//                 path: String::from(""),
//             })),
//             data_source: Some(HttpDataSource(HttpData {
//                 // tsv file. @see https://cloud.google.com/storage-transfer/docs/create-url-list
//                 list_url: String::from(".tsv"),
//             })),
//         }),
//         notification_config: None,
//         schedule: None,
//         status: 1,
//         creation_time: None,
//         last_modification_time: None,
//         deletion_time: None,
//         latest_operation_name: String::from(""),
//     });

//     let response = service
//         .create_transfer_job(Request::new(CreateTransferJobRequest {
//             transfer_job: transfer_job,
//         }))
//         .await?;
//     println!("RESPONSE={:?}", response);

//     Ok(())
// }
