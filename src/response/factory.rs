/// The response factory trait is a trait used by any response struct which consumes its fields and
/// returns a valid HTTP response.
pub trait ResponseFactory {
    type ResponseContent;
    type ResponseError;

    fn build(self) -> Result<Self::ResponseContent, Self::ResponseError>;
}
