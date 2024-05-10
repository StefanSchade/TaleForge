use crosscutting::error_management::error::Error;

pub trait ResponseMapperTrait<ApiResponseCodeLevel, ApiResponseBodyLevel, PortModel>: Send + Sync {
    fn to_api_body(domain_model: PortModel) -> Result<ApiResponseBodyLevel, Error>;
    fn to_api_200(domain_model: PortModel) -> ApiResponseCodeLevel;
    fn to_api_error_codes(domain_model: PortModel) -> ApiResponseCodeLevel;
}