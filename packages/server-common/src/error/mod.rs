use tracing::error;

#[derive(Debug)]
pub enum CustomError {
    Ai(String),
    ES(String),
    HTTP(String),
    JSON(String),
    BASE64(String),
    VecToStr(String),
    Regex(String),
    JWT(String),
    Config(String)
}
pub struct ErrorInfo {
    pub message: String,
    pub code: u64,
}
pub fn extract_info_from_custom_error(error: CustomError) -> ErrorInfo {
    match error {
        CustomError::Ai(message) => ErrorInfo{
            message,
            code:1000
        },
        CustomError::ES(message) => ErrorInfo{
            message,
            code:1001
        },
        CustomError::HTTP(message) =>  ErrorInfo{
            message,
            code:1002
        },
        CustomError::JSON(message) =>  ErrorInfo{
            message,
            code:1003
        },
        CustomError::BASE64(message) =>  ErrorInfo{
            message,
            code:1004
        },
        CustomError::VecToStr(message) =>  ErrorInfo{
            message,
            code:1005
        },
        CustomError::Regex(message) =>  ErrorInfo{
            message,
            code:1006
        },
        CustomError::JWT(message) =>  ErrorInfo{
            message,
            code:1007
        },
         CustomError::Config(message) =>  ErrorInfo{
            message,
            code:1008
        },
    }
}


pub fn log_error(error: CustomError)->CustomError{
    error!("{:?}",error);
    return error;
}