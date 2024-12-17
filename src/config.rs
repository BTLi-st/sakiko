/// Sakiko 的配置类
/// 包含配置的数据结构定义，序列化和反序列化方法

use std::ops::Not;

use ::serde::{Deserialize, Serialize};
use ::serde_yaml;
use ::std::collections::HashMap;

use crate::compare::Compares;
use crate::operation::Operations;
use crate::variable::{VariableType, Variables};
use crate::output::Output;

/// 转移类
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Transaction {
    #[serde(default = "String::new")]
    pub pattern: String, // 匹配字符串（可置空）
    #[serde(default = "Compares::new")]
    pub compares: Compares, // 比较条件（可置空）
    pub step: String, // 下一步（必须）
    #[serde(default = "Operations::new")]
    pub operation: Operations, // 操作（可置空）
}

/// 步骤类
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Step {
    #[serde(default = "Output::default")]
    pub description: Output, // 给用户的输出（可置空）
    pub transaction: Vec<Transaction>, // 转移
}

/// Sakiko 配置类
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SakikoConfig {
    bot_name: String, // 机器人名
    start_step: String, // 开始步骤
    #[serde(default = "Variables::new")]
    variables: Variables, // 变量（可置空）
    steps: HashMap<String, Step>, // 步骤
}

impl SakikoConfig {
    /// 创建一个新的配置（测试用）
    pub fn new(bot_name: &str, start_step: &str) -> SakikoConfig {
        SakikoConfig {
            bot_name: bot_name.to_string(),
            start_step: start_step.to_string(),
            variables: Variables::new(),
            steps: HashMap::new(),
        }
    }

    /// 序列化到字符串
    pub fn serialize(&self) -> String {
        serde_yaml::to_string(&self).unwrap()
    }

    /// 序列化到文件
    pub fn serialize_to_file(&self, file_path: &str) -> Result<(), serde_yaml::Error> {
        serde_yaml::to_writer(std::fs::File::create(file_path).unwrap(), &self)
    }

    /// 从字符串反序列化
    pub fn deserialize(yaml: &str) -> Result<SakikoConfig, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }

    /// 从文件反序列化
    pub fn deserialize_from_file(file_path: &str) -> Result<SakikoConfig, serde_yaml::Error> {
        serde_yaml::from_reader(std::fs::File::open(file_path).unwrap())
    }

    /// 添加变量（测试用）
    pub fn add_variable(&mut self, variable_name: &str, variable_type: VariableType) {
        self.variables
            .insert(variable_name.to_string(), variable_type);
    }

    /// 添加步骤（测试用）
    pub fn add_step(&mut self, step_name: &str, description: &Output) {
        self.steps.insert(
            step_name.to_string(),
            Step {
                description: description.clone(),
                transaction: Vec::new(),
            },
        );
    }

    /// 添加转移（测试用）
    pub fn add_transaction(
        &mut self,
        step_name: &str,
        pattern: &str,
        compares: &Compares,
        next_step: &str,
        operation: &Operations,
    ) {
        self.steps
            .get_mut(step_name)
            .unwrap()
            .transaction
            .push(Transaction {
                pattern: pattern.to_string(),
                compares: compares.clone(),
                step: next_step.to_string(),
                operation: operation.clone(),
            });
    }

    /// 获取开始步骤
    pub fn get_start_step(&self) -> &str {
        &self.start_step
    }

    /// 获取机器人名
    pub fn get_bot_name(&self) -> &str {
        &self.bot_name
    }

    /// 获取变量
    pub fn get_variables(&self) -> &Variables {
        &self.variables
    }

    /// 获取步骤
    pub fn get_step(&self, step_name: &str) -> Option<&Step> {
        self.steps.get(step_name)
    }

    /// 是否包含步骤 end
    pub fn has_end(&self) -> bool {
        self.steps.contains_key("end")
    }

    /// 每个转移中的步骤是否存在
    pub fn check(&self) -> Result<(), String> {
        let mut errors = Vec::new();

        if self.steps.contains_key(&self.start_step).not() {
            errors.push(format!("Start step {} not found", self.start_step));
        }

        for (step_name, step) in &self.steps {
            for transaction in &step.transaction {
                if self.steps.contains_key(&transaction.step).not() && transaction.step != "end" {
                    errors.push(format!(
                        "Step {} in transaction of step {} not found",
                        transaction.step, step_name
                    ));
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join(", "))
        }
    }

    /// 检测比较
    pub fn check_compares(&self) -> Result<(), String> {
        let mut errors = Vec::new();

        for (step_name, step) in &self.steps {
            for transaction in &step.transaction {
                if let Err(err) = transaction.compares.check(&self.variables) {
                    errors.push(format!(
                        "Compares in transaction of step {} failed: {}",
                        step_name, err
                    ));
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join("\n"))
        }
    }

    /// 检测输出
    pub fn check_description(&self) -> Result<(), String> {
        let mut errors = Vec::new();

        for (step_name, step) in &self.steps {
            if let Err(err) = step.description.check(&self.variables) {
                errors.push(format!("Description of step {} failed: {}", step_name, err));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join("\n"))
        }
    }

    /// 检测操作
    pub fn check_operations(&self) -> Result<(), String> {
        let mut errors = Vec::new();

        for (step_name, step) in &self.steps {
            for transaction in &step.transaction {
                if let Err(err) = transaction.operation.check(&self.variables) {
                    errors.push(format!(
                        "Operation in transaction of step {} failed: {}",
                        step_name, err
                    ));
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join("\n"))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::compare::{Compare, CompareItem, CompareType, Compares};
    use crate::operation::{Operation, Operations};

    use super::*;

    // 测试用配置
    fn create_test_config() -> SakikoConfig {
        let mut config = SakikoConfig::new("Sakiko", "start");
        config.add_variable("int1", VariableType::Int(1));
        config.add_variable("int2", VariableType::Int(2));
        config.add_variable("str", VariableType::Str("test".to_string()));
        config.add_variable("float", VariableType::Float(1.0));
        config.add_variable("int_vec", VariableType::IntVec(vec![1, 2, 3]));
        config.add_variable(
            "str_vec",
            VariableType::StrVec(vec!["a".to_string(), "b".to_string()]),
        );
        config.add_variable("float_vec", VariableType::FloatVec(vec![1.0, 2.0, 3.0]));
        let mut str_dic = HashMap::new();
        str_dic.insert("key".to_string(), "value".to_string());
        str_dic.insert("key2".to_string(), "value2".to_string());
        config.add_variable("str_dic", VariableType::StrDic(str_dic));
        let mut output = Output("Start of the conversation".to_string(), Vec::new());
        config.add_step("start", &output);
        output = Output("End of the conversation {}".to_string(), vec!["int1".to_string()]);
        config.add_step("end", &output);
        let mut compares = Compares::new();
        compares.add(CompareItem {
            compare: Compare::And,
            compare_type: CompareType::Ne("int1".to_string(), "int3".to_string()),
        });
        config.add_transaction(
            "start",
            "goodbye",
            &compares,
            "end",
            &Operations {
                0: vec![
                    Operation::Let("int".to_string(), VariableType::Int(2)),
                    Operation::Add("int1".to_string(), "int1".to_string(), "int2".to_string()),
                ],
            },
        );
        config
    }

    // 测试相等，确保序列化和反序列化的正确性
    #[test]
    fn test_eq() {
        let config = create_test_config();

        let config2 = create_test_config();

        let config3 = SakikoConfig::new("3", "start");

        assert_eq!(config, config2);
        assert_ne!(config, config3);
    }

    // 测试序列化和反序列化
    #[test]
    fn test_serialize() {
        let config = create_test_config();

        let yaml = serde_yaml::to_string(&config).unwrap();
        println!("{}", yaml);
        let deserialized: SakikoConfig = serde_yaml::from_str(&yaml).unwrap();
        println!("{:?}", deserialized);
        assert_eq!(config, deserialized);
    }

    // 测试文件读写
    #[test]
    fn file_io() {
        let config = create_test_config();
        let file_path = "test.yaml";
        config.serialize_to_file(file_path).unwrap();
        let deserialized = SakikoConfig::deserialize_from_file(file_path).unwrap();
        assert_eq!(config, deserialized);
    }
}
