use anyhow::anyhow;

pub trait IntoAnyhow<T> {
    fn into_anyhow(self) -> anyhow::Result<T>;
}

impl<T> IntoAnyhow<T> for Result<T, jsonrpc_v2::Error> {
    fn into_anyhow(self) -> anyhow::Result<T> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => match e {
                jsonrpc_v2::Error::Full {
                    code,
                    message,
                    data,
                } => match data {
                    Some(data) => Err(anyhow!(
                        "rpc error: code: {}, messgae: {}, data: {}",
                        code,
                        message,
                        serde_json::to_string(&data).unwrap_or_default()
                    )),
                    None => Err(anyhow!("rpc error: code: {}, messgae: {}", code, message)),
                },
                jsonrpc_v2::Error::Provided { code, message } => {
                    Err(anyhow!("rpc error: code: {}, messgae: {}", code, message))
                }
            },
        }
    }
}
