use lambda_runtime::{handler_fn, Context, Error};
use rust_csv_processig::{
    aws::client::{AWSClient, AWSConfig},
    dtos::record::Record,
    dtos::s3_event::S3Event,
};
use tokio_stream::StreamExt;
use tokio_util::io::StreamReader;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initialize AWS client
    let config = aws_config::load_from_env().await;
    let config = AWSConfig::set_config(config);
    let aws_client = config.on_s3_presigned_url();

    lambda_runtime::run(handler_fn(|event: S3Event, ctx: Context| {
        execute(&aws_client, event, ctx)
    }))
    .await?;

    Ok(())
}

pub async fn execute(aws_client: &AWSClient, event: S3Event, _ctx: Context) -> Result<(), Error> {
    println!("{:?}", event);

    let stream = aws_client
        .s3_client
        .as_ref()
        .unwrap()
        .get_object()
        .bucket(event.bucket.name.to_string())
        .key(event.object.key.to_string())
        .send()
        .await?
        .body
        .map(|result| result.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)));

    // Convert the stream into an AsyncRead
    let stream_reader = StreamReader::new(stream);

    // Create a CSV reader
    let mut csv_reader = csv_async::AsyncReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'\t')
        .double_quote(false)
        .escape(Some(b'\\'))
        .flexible(true)
        .create_deserializer(stream_reader);

    let mut count = 0;
    // Iterate over the CSV rows
    let mut records = csv_reader.deserialize::<Record>();
    while let Some(record) = records.next().await {
        let _record: Record = record?;
        count += 1;
    }

    println!("COMPLETED {:?}", count);

    Ok(())
}
