/// Sakiko 的输出
/// 输出类包含了一个字符串模板和一个字符串数组，字符串数组中的字符串是变量名，字符串模板中的 {} 会被替换为变量的值（类似 format! 宏）。
use ::serde::{Deserialize, Serialize};

use crate::variable::Variables;

/// 输出类
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Output(pub  String, pub Vec<String>);

/// 输出类的默认实现（实现 yaml 文件中可以不写该字段）
impl Default for Output {
    fn default() -> Self {
        Self("".to_string(), Vec::new())
    }
}

impl Output {
    /// 创建一个新的输出类
    pub fn new(name: String, vars: Vec<String>) -> Self {
        Self(name, vars)
    }

    /// 判断是否为空
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// 格式化输出
    pub fn fmt(&self, variables: &Variables) -> Result<String, &'static str> {
        for var in &self.1 {
            if variables.get(var).is_none() {
                return Err("Variable not found");
            }
        }

        let vars = self.1.iter().map(|var| variables.get(var).unwrap().to_string()).collect::<Vec<String>>();

        let mut result = String::new();
        let mut format_iter = self.0.split("{}");

        if let Some(f) = format_iter.next() {
            result.push_str(f);
        }

        for (arg, var) in format_iter.zip(vars) {
            result.push_str(&var);
            result.push_str(arg);
        }

        Ok(result)
    }

    /// 检查变量是否存在（检测用，会报告所有不存在的变量）
    pub fn check(&self, variables: &Variables) -> Result<(), String> {
        let mut result = Vec::new();
        for var in &self.1 {
            if variables.get(var).is_none() {
                result.push(format!("Variable not found: {}", var));
            }
        }
        if !result.is_empty() {
            return Err(result.join(", "));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::variable::{VariableType, Variables};

    use super::*;

    // 测试输出格式化
    #[test]
    fn test_fmt() {
        let mut variables = Variables::new();
        variables.insert("num1".to_string(), VariableType::Int(1));
        variables.insert("num2".to_string(), VariableType::Int(2));
        variables.insert("ope".to_string(), VariableType::Str("+".to_string()));

        let output = Output::new("{} {} {} = {}".to_string(), vec!["num1".to_string(), "ope".to_string(), "num1".to_string(), "num2".to_string()]);

        assert_eq!(output.fmt(&variables).unwrap(), "1 + 1 = 2");
    }
}