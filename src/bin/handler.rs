use aws_lambda_events::s3::S3Event;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use rust_csv_processig::dtos::record::Record;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .without_time()
        .with_max_level(tracing_subscriber::filter::LevelFilter::INFO)
        .init();

    let config = aws_config::load_from_env().await;
    let s3_client = aws_sdk_s3::Client::new(&config);

    run(service_fn(|event: LambdaEvent<S3Event>| {
        handler(&s3_client, event)
    }))
    .await
}

pub async fn handler(
    s3_client: &aws_sdk_s3::Client,
    event: LambdaEvent<S3Event>,
) -> Result<(), Error> {
    println!("{event:?}");
    let s3_event = event.payload.records.first().map(|event| event.clone().s3);
    if let Some(s3_event) = s3_event {
        let stream = s3_client
            .get_object()
            .bucket(s3_event.bucket.name.unwrap())
            .key(s3_event.object.key.unwrap())
            .send()
            .await?
            .body
            .into_async_read();

        let mut csv_reader = csv_async::AsyncReaderBuilder::new()
            .has_headers(true)
            .delimiter(b'\t')
            .double_quote(false)
            .escape(Some(b'\\'))
            .flexible(true)
            .create_deserializer(stream);

        let mut count = 0;
        // Iterate over the CSV rows
        let mut records = csv_reader.deserialize::<Record>();
        while let Some(record) = records.next().await {
            let _record: Record = record?;
            count += 1;
        }

        println!("COMPLETED {count}");
    } else {
        println!("No S3 event found");
    }

    Ok(())
}
