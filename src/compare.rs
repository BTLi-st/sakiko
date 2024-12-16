// Sakiko 的比较
// 比较类包含了一个比较类型和一个比较值，比较类型包含了等于、不等于、大于、大于等于、小于、小于等于，比较值包含了两个变量名。
// 支持的类型有整数、浮点数和字符串，比较时会检查类型是否匹配。
// 计算时支持与和或
use std::ops::Not;

use ::serde::{Deserialize, Serialize};

pub use crate::variable::VariableType;
pub use crate::variable::Variables;

// 比较类型
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum CompareType {
    Eq(String, String), // Equal
    Ne(String, String), // Not Equal
    Gt(String, String), // Greater Than
    Ge(String, String), // Greater or Equal
    Lt(String, String), // Less Than
    Le(String, String), // Less or Equal
}

// 比较
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Copy)]
pub enum Compare {
    And,
    Or,
}

// 比较项
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct CompareItem {
    pub compare: Compare,
    pub compare_type: CompareType,
}

// 比较类
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Compares (pub Vec<CompareItem>);

impl CompareType {
    // 检查比较类型
    pub fn compare_check(&self, variable: &Variables) -> Result<(), String> {
        match self {
            CompareType::Eq(a, b)
            | CompareType::Ne(a, b)
            | CompareType::Gt(a, b)
            | CompareType::Ge(a, b)
            | CompareType::Lt(a, b)
            | CompareType::Le(a, b) => {
                let a = variable.get(a).ok_or_else(|| format!("Variable {} not found", a))?;
                let b = variable.get(b).ok_or_else(|| format!("Variable {} not found", b))?;
                if a.is_same_type(b) && a.is_vector().not() && a.is_dict().not() { // a, b are not vector and same type
                    Ok(())
                } else {
                    Err("Type mismatch".to_string())
                }
            }
        }
    }

    // 计算比较
    pub fn compare(&self, variable: &Variables) -> Result<bool, String> {
        let (a, b) = match self {
            CompareType::Eq(a, b)
            | CompareType::Ne(a, b)
            | CompareType::Gt(a, b)
            | CompareType::Ge(a, b)
            | CompareType::Lt(a, b)
            | CompareType::Le(a, b) => {
                let a = variable.get(a).ok_or_else(|| format!("Variable {} not found", a))?;
                let b = variable.get(b).ok_or_else(|| format!("Variable {} not found", b))?;
                (a, b)
            }
        };
    
        match (self, a, b) {
            (CompareType::Eq(_, _), VariableType::Int(a), VariableType::Int(b)) => Ok(a == b),
            (CompareType::Ne(_, _), VariableType::Int(a), VariableType::Int(b)) => Ok(a != b),
            (CompareType::Gt(_, _), VariableType::Int(a), VariableType::Int(b)) => Ok(a > b),
            (CompareType::Ge(_, _), VariableType::Int(a), VariableType::Int(b)) => Ok(a >= b),
            (CompareType::Lt(_, _), VariableType::Int(a), VariableType::Int(b)) => Ok(a < b),
            (CompareType::Le(_, _), VariableType::Int(a), VariableType::Int(b)) => Ok(a <= b),
    
            (CompareType::Eq(_, _), VariableType::Float(a), VariableType::Float(b)) => Ok(a == b),
            (CompareType::Ne(_, _), VariableType::Float(a), VariableType::Float(b)) => Ok(a != b),
            (CompareType::Gt(_, _), VariableType::Float(a), VariableType::Float(b)) => Ok(a > b),
            (CompareType::Ge(_, _), VariableType::Float(a), VariableType::Float(b)) => Ok(a >= b),
            (CompareType::Lt(_, _), VariableType::Float(a), VariableType::Float(b)) => Ok(a < b),
            (CompareType::Le(_, _), VariableType::Float(a), VariableType::Float(b)) => Ok(a <= b),
    
            (CompareType::Eq(_, _), VariableType::Str(a), VariableType::Str(b)) => Ok(a == b),
            (CompareType::Ne(_, _), VariableType::Str(a), VariableType::Str(b)) => Ok(a != b),
            (CompareType::Gt(_, _), VariableType::Str(a), VariableType::Str(b)) => Ok(a > b),
            (CompareType::Ge(_, _), VariableType::Str(a), VariableType::Str(b)) => Ok(a >= b),
            (CompareType::Lt(_, _), VariableType::Str(a), VariableType::Str(b)) => Ok(a < b),
            (CompareType::Le(_, _), VariableType::Str(a), VariableType::Str(b)) => Ok(a <= b),
    
            _ => Err("Type mismatch".to_string()),
        }
    }
}

impl Compares {
    pub fn new() -> Self {
        Compares(Vec::new())
    }

    pub fn add(&mut self, compare: CompareItem) {
        self.0.push(compare);
    }

    // 检查比较（检测用，会报告所有错误）
    pub fn check(&self, variable: &Variables) -> Result<(), String> {
        let mut errors = Vec::new();

        for compare_item in &self.0 {
            match compare_item.compare {
                Compare::And | Compare::Or => {
                    if let Err(err) = compare_item.compare_type.compare_check(variable) {
                        errors.push(err);
                    }
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join(", "))
        }
    }

    // 计算比较
    pub fn calc(&self, variable: &Variables) -> Result<bool, String> {
        self.0
            .iter()
            .try_fold(None, |acc, compare_item| {
                let current_result = match compare_item.compare {
                    Compare::And | Compare::Or => {
                        compare_item.compare_type.compare(variable)?
                    }
                };
                Ok(Some(match (acc, compare_item.compare) {
                    (None, _) => current_result,
                    (Some(result), Compare::And) => result && current_result,
                    (Some(result), Compare::Or) => result || current_result,
                }))
            })
            .map(|result| result.unwrap_or(true))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 创建测试变量
    fn create_test_variables() -> Variables {
        let mut variables = Variables::new();
        variables.insert("int1".to_string(), VariableType::Int(1));
        variables.insert("int2".to_string(), VariableType::Int(1));
        variables.insert("int3".to_string(), VariableType::Int(2));
        variables.insert("str1".to_string(), VariableType::Str("test".to_string()));
        variables.insert("str2".to_string(), VariableType::Str("ab".to_string()));
        variables.insert("float1".to_string(), VariableType::Float(1.0));
        variables.insert("float2".to_string(), VariableType::Float(1.0));
        variables.insert("int_vec".to_string(), VariableType::IntVec(vec![1, 2, 3]));
        variables.insert(
            "str_vec".to_string(),
            VariableType::StrVec(vec!["a".to_string(), "b".to_string()]),
        );
        variables.insert(
            "float_vec".to_string(),
            VariableType::FloatVec(vec![1.0, 2.0, 3.0]),
        );
        variables
    }

    // 测试比较
    #[test]
    fn test_compare() {
        let variables = create_test_variables();
        let mut compares = Compares::new();
        compares.add(CompareItem {
            compare: Compare::And,
            compare_type: CompareType::Eq("int1".to_string(), "int2".to_string()),
        });
        compares.add(CompareItem {
            compare: Compare::And,
            compare_type: CompareType::Ne("int1".to_string(), "int3".to_string()),
        });
        assert_eq!(compares.calc(&variables).unwrap(), true);
    }
}
