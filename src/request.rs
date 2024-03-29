use anyhow::anyhow;
use anyhow::Result;

pub enum Method {
    Get,
    Post,
    Put,
    Delete,
}

impl Method {
    pub fn parse(method: &str) -> Result<Self> {
        return match method {
            "GET" => Ok(Self::Get),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            "DELETE" => Ok(Self::Delete),
            _ => Err(anyhow!("failed to parse method")),
        };
    }
}

pub struct Path(String);

pub struct Request {
    method: Method,
    path: String,
}

impl Request {
    pub fn parse(request: &str) -> Result<Self> {
        let mut rest = request.split(" ");
        let method = rest.next().ok_or(anyhow!("no method"))?;
        let method = Method::parse(method)?;
        todo!()
    }
}

trait FromRequest {
    fn from_request(request: &Request) -> Self;
}
