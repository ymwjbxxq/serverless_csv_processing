#[derive(Debug)]
pub struct AWSConfig {
    pub config: aws_types::config::Config,
}

impl AWSConfig {
    pub fn set_config(config: aws_types::config::Config) -> Self {
        Self { config }
    }

    fn get_config(&self) -> aws_types::config::Config {
        self.config.clone()
    }

    pub fn on_s3_presigned_url(&self) -> AWSClient {
        let aws_client = AWSClient {
            config: self.get_config(),
            s3_client: Some(self.s3_client()),
        };

        aws_client
    }

    fn s3_client(&self) -> aws_sdk_s3::Client {
        aws_sdk_s3::Client::new(&self.config)
    }
}

#[derive(Clone, Debug)]
pub struct AWSClient {
    pub config: aws_types::config::Config,
    pub s3_client: Option<aws_sdk_s3::Client>,
}
