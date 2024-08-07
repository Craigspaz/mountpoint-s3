use crate::object_client::ObjectClientResult;
use crate::s3_crt_client::{S3CrtClient, S3Operation, S3RequestError};
use thiserror::Error;

/// Errors returned by a [`head_bucket`](S3CrtClient::head_bucket) request.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum HeadBucketError {
    #[error("The bucket did not exist")]
    NoSuchBucket,
}

impl S3CrtClient {
    pub async fn head_bucket(&self, bucket: &str) -> ObjectClientResult<(), HeadBucketError, S3RequestError> {
        let body =
            {
                let mut message = self
                    .inner
                    .new_request_template("HEAD", bucket)
                    .map_err(S3RequestError::construction_failure)?;

                message
                    .set_request_path("/")
                    .map_err(S3RequestError::construction_failure)?;

                let span = request_span!(self.inner, "head_bucket");

                self.inner
                    .make_simple_http_request(message, S3Operation::HeadBucket, span, |request_result| {
                        match request_result.response_status {
                            404 => Some(HeadBucketError::NoSuchBucket),
                            _ => None,
                        }
                    })?
            };

        body.await.map(|_body| ())
    }
}
