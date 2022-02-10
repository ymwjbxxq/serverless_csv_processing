use csv;
use lambda_runtime::{handler_fn, Context, Error};
use rust_csv_processig::{
    aws::client::{AWSClient, AWSConfig},
    dtos::record::Record,
    dtos::s3_event::S3Event,
};

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

    let data = download_file(aws_client, &event).await?;
    let buffer = &data[..];

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'\t')
        .double_quote(false)
        .escape(Some(b'\\'))
        .flexible(true)
        .from_reader(buffer);

    for result in rdr.deserialize() {
        let record: Record = result?;
        // do something with record
    }

    Ok(())
}

async fn download_file(aws_client: &AWSClient, event: &S3Event) -> Result<bytes::Bytes, Error> {
    let result = aws_client
        .s3_client
        .as_ref()
        .unwrap()
        .get_object()
        .bucket(event.bucket.name.to_string())
        .key(event.object.key.to_string())
        .send()
        .await?;

    let data = result.body.collect().await?.into_bytes();
    Ok(data)
}
