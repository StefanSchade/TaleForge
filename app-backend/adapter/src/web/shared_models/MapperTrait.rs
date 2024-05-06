use crosscutting::error_management::error::Error;

pub trait MapperTrait<ApiModel, PortModel>: Send + Sync {
    fn from_api(api_model: ApiModel) -> Result<PortModel, Error>;
    fn to_api(domain_model: PortModel) -> Result<ApiModel, Error>;
}