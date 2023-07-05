use std::error::Error;
use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(name = "Rust HTTP CLI")]
#[clap(version, about = "A Simple Http CLI", long_about = None)]
pub struct Args {
    #[clap(value_parser, help = "HTTP method")]
    pub method: String,

    #[clap(value_parser, help = "HTTP URL")]
    pub url: String,

    /// 添加query参数 (key=value)
    #[arg(short = 'q', value_name = "key=value", value_parser = parse_key_val::< String, String >)]
    pub query: Option<Vec<(String, String)>>,

    /// 添加form参数 (key=value)
    #[arg(short = 'f', value_name = "key=value", value_parser = parse_key_val::< String, String >)]
    pub form: Option<Vec<(String, String)>>,

    /// 添加json参数 (key=value)
    #[arg(short = 'j', value_name = "key=value", value_parser = parse_key_val::< String, String >)]
    pub json: Option<Vec<(String, String)>>,
}

/// 解析 key-value 参数
fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
    where
        T: std::str::FromStr,
        T::Err: Error + Send + Sync + 'static,
        U: std::str::FromStr,
        U::Err: Error + Send + Sync + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("无效的 key=value: 在 `{s}` 中没有发现 ="))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}
