use std::sync::Arc;

/// 该文件为库的入口文件，定义了库的公共接口
pub mod config;
mod variable;
mod compare;
pub mod sakiko;
mod operation;
mod output;
pub mod check;

pub use config::SakikoConfig;
pub use sakiko::Session;

/// 加载配置文件
/// 传入配置文件路径，返回智能指针封装的配置
pub fn load_config(path: &str) -> Result<Arc<config::SakikoConfig>, serde_yaml::Error> {
    let config = config::SakikoConfig::deserialize_from_file(path)?;
    Ok(Arc::new(config))
}