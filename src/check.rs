/// 实现配置的检测

use crate::config::SakikoConfig;

/// 检测 config 是否合法
/// 传入配置，返回错误信息
pub fn check_config(config: &SakikoConfig) -> Result<(), String> {
    let mut result = Vec::new();
    // 检查所有步骤是否存在
    if let Err(e) = config.check() {
        result.push("Step check failed".to_string());
        result.push(e);
    }
    // 检查步骤是否包含 end
    if config.has_end() {
        result.push("Step 'end' is reserved".to_string());
    }
    // 检查变量是否包含 input
    if config.get_variables().has_input() {
        result.push("Variable 'input' is reserved".to_string());
    }
    // 检测比较是否正常
    if let Err(e) = config.check_compares() {
        result.push("Compares check failed".to_string());
        result.push(e);
    }
    // 检测输出是否正常
    if let Err(e) = config.check_description() {
        result.push("Description check failed".to_string());
        result.push(e);
    }
    // 检测操作是否正常
    if let Err(e) = config.check_operations() {
        result.push("Operation check failed".to_string());
        result.push(e);
    }
    // 检查结果
    if !result.is_empty() {
        return Err(result.join("\n"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_config() {
        let config = SakikoConfig::deserialize_from_file("demo/error.yaml").unwrap();
        assert!(check_config(&config).is_err());
        print!("{}\n", check_config(&config).unwrap_err());
    }
}
