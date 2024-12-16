// Sakiko 中的变量操作
// Sakiko 中的变量操作是通过 Operation 类实现的，Operation 类包含了一系列操作
use ::rand::seq::SliceRandom;
use ::rand::Rng;
use ::serde::{Deserialize, Serialize};

use crate::variable::{VariableType, Variables};

// 操作
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum Operation {
    Add(String, String, String), // a = b + c
    Sub(String, String, String), // a = b - c
    Mul(String, String, String), // a = b * c
    Div(String, String, String), // a = b / c

    Get(String, String, String), // a = b[c]
    Set(String, String, String), // b[c] = a

    Let(String, VariableType), // a = b
    Cpy(String, String),       // a = b

    Rnd(String, String, String), // a = random(b, c)
    Shu(String),                 // shuffle a

    Qry(String, String, String), // a = b.query(c)
    Ins(String, String, String), // b.new(a, c)

    Inp(String), // a = input()
}

// 操作集合
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Operations(pub Vec<Operation>);

impl Operation {
    // 检查操作，类型匹配
    pub fn operation_check(&self, variables: &Variables) -> Result<(), String> {
        match self {
            Operation::Add(a, b, c)
            | Operation::Sub(a, b, c)
            | Operation::Mul(a, b, c)
            | Operation::Div(a, b, c) => {
                let a = variables
                    .get(a)
                    .ok_or_else(|| format!("Variable {} not found", a))?;
                let b = variables
                    .get(b)
                    .ok_or_else(|| format!("Variable {} not found", b))?;
                let c = variables
                    .get(c)
                    .ok_or_else(|| format!("Variable {} not found", c))?;
                if a.is_same_type(b) && a.is_same_type(c) && a.is_number() {
                    Ok(())
                } else {
                    Err("Type mismatch".to_string())
                }
            }
            Operation::Get(a, b, c) | Operation::Set(a, b, c) => {
                let a = variables
                    .get(a)
                    .ok_or_else(|| format!("Variable {} not found", a))?;
                let b = variables
                    .get(b)
                    .ok_or_else(|| format!("Variable {} not found", b))?;
                let c = variables
                    .get(c)
                    .ok_or_else(|| format!("Variable {} not found", c))?;
                match (a, b, c) {
                    (VariableType::Str(_), VariableType::StrVec(b), VariableType::Int(c)) => {
                        if *c < 0 || (*c as usize) < b.len() {
                            Ok(())
                        } else {
                            Err("Index out of bounds".to_string())
                        }
                    }
                    (VariableType::Int(_), VariableType::IntVec(b), VariableType::Int(c)) => {
                        if *c < 0 || (*c as usize) < b.len() {
                            Ok(())
                        } else {
                            Err("Index out of bounds".to_string())
                        }
                    }
                    (VariableType::Float(_), VariableType::FloatVec(_), VariableType::Int(_)) => {
                        Ok(())
                    }
                    _ => Err("Type mismatch".to_string()),
                }
            }
            Operation::Let(a, b) => {
                let a = variables
                    .get(a)
                    .ok_or_else(|| format!("Variable {} not found", a))?;
                if a.is_same_type(b) {
                    Ok(())
                } else {
                    Err("Type mismatch".to_string())
                }
            }
            Operation::Cpy(a, b) => {
                let a = variables
                    .get(a)
                    .ok_or_else(|| format!("Variable {} not found", a))?;
                let b = variables
                    .get(b)
                    .ok_or_else(|| format!("Variable {} not found", b))?;
                if a.is_same_type(b) {
                    Ok(())
                } else {
                    Err("Type mismatch".to_string())
                }
            }
            Operation::Rnd(a, b, c) => {
                let b = variables
                    .get(b)
                    .ok_or_else(|| format!("Variable {} not found", b))?;
                let c = variables
                    .get(c)
                    .ok_or_else(|| format!("Variable {} not found", c))?;
                let a = variables
                    .get(a)
                    .ok_or_else(|| format!("Variable {} not found", a))?;
                if (*a).is_same_type(&b) && (*a).is_same_type(&c) {
                    match a {
                        VariableType::Int(_) => Ok(()),
                        VariableType::Float(_) => Ok(()),
                        _ => Err("Type mismatch".to_string()),
                    }
                } else {
                    Err("Type mismatch".to_string())
                }
            }
            Operation::Shu(a) => {
                let a = variables
                    .get(a)
                    .ok_or_else(|| format!("Variable {} not found", a))?;
                match a {
                    VariableType::StrVec(_)
                    | VariableType::IntVec(_)
                    | VariableType::FloatVec(_) => Ok(()),
                    _ => Err("Type mismatch".to_string()),
                }
            }
            Operation::Inp(a) => {
                let a = variables
                    .get(a)
                    .ok_or_else(|| format!("Variable {} not found", a))?;
                match a {
                    VariableType::Str(_) | VariableType::Int(_) | VariableType::Float(_) => Ok(()),
                    _ => Err("Type mismatch".to_string()),
                }
            }
            Operation::Qry(a, b, c) | Operation::Ins(a, b, c) => {
                let a = variables
                    .get(a)
                    .ok_or_else(|| format!("Variable {} not found", a))?;
                let b = variables
                    .get(b)
                    .ok_or_else(|| format!("Variable {} not found", b))?;
                let c = variables
                    .get(c)
                    .ok_or_else(|| format!("Variable {} not found", c))?;
                match (a, b, c) {
                    (VariableType::Str(_), VariableType::StrDic(_), VariableType::Str(_)) => Ok(()),
                    (VariableType::Int(_), VariableType::IntDic(_), VariableType::Str(_)) => Ok(()),
                    (VariableType::Float(_), VariableType::FloatDic(_), VariableType::Str(_)) => {
                        Ok(())
                    }
                    _ => Err("Type mismatch".to_string()),
                }
            }
        }
    }

    // 计算操作
    pub fn calculate(&self, variables: &mut Variables) -> Result<(), String> {
        match self {
            Operation::Add(a, b, c)
            | Operation::Sub(a, b, c)
            | Operation::Mul(a, b, c)
            | Operation::Div(a, b, c) => {
                let b = variables
                    .get(b)
                    .ok_or_else(|| format!("Variable {} not found", b))?
                    .clone();
                let c = variables
                    .get(c)
                    .ok_or_else(|| format!("Variable {} not found", c))?
                    .clone();
                let a = variables
                    .get_mut(a)
                    .ok_or_else(|| format!("Variable {} not found", a))?;
                match (a, b, c) {
                    (VariableType::Int(a), VariableType::Int(b), VariableType::Int(c)) => {
                        match self {
                            Operation::Add(_, _, _) => *a = b + c,
                            Operation::Sub(_, _, _) => *a = b - c,
                            Operation::Mul(_, _, _) => *a = b * c,
                            Operation::Div(_, _, _) => *a = b / c,
                            _ => unreachable!(),
                        }
                    }
                    (VariableType::Float(a), VariableType::Float(b), VariableType::Float(c)) => {
                        match self {
                            Operation::Add(_, _, _) => *a = b + c,
                            Operation::Sub(_, _, _) => *a = b - c,
                            Operation::Mul(_, _, _) => *a = b * c,
                            Operation::Div(_, _, _) => *a = b / c,
                            _ => unreachable!(),
                        }
                    }
                    _ => return Err("Type mismatch".to_string()),
                }
            }
            Operation::Get(a, b, c) => {
                let b = variables
                    .get(b)
                    .ok_or_else(|| format!("Variable {} not found", b))?
                    .clone();
                let c = variables
                    .get(c)
                    .ok_or_else(|| format!("Variable {} not found", c))?
                    .clone();
                let a = variables
                    .get_mut(a)
                    .ok_or_else(|| format!("Variable {} not found", a))?;
                match (a, b, c) {
                    (VariableType::Str(a), VariableType::StrVec(b), VariableType::Int(c)) => {
                        *a = b[c as usize].clone();
                    }
                    (VariableType::Int(a), VariableType::IntVec(b), VariableType::Int(c)) => {
                        *a = b[c as usize];
                    }
                    (VariableType::Float(a), VariableType::FloatVec(b), VariableType::Int(c)) => {
                        *a = b[c as usize];
                    }
                    _ => return Err("Type mismatch".to_string()),
                }
            }
            Operation::Set(a, b, c) => {
                let a = variables
                    .get(a)
                    .ok_or_else(|| format!("Variable {} not found", a))?
                    .clone();
                let c = variables
                    .get(c)
                    .ok_or_else(|| format!("Variable {} not found", c))?
                    .clone();
                let b = variables
                    .get_mut(b)
                    .ok_or_else(|| format!("Variable {} not found", b))?;
                match (a, b, c) {
                    (VariableType::Str(a), VariableType::StrVec(b), VariableType::Int(c)) => {
                        b[c as usize] = a.clone();
                    }
                    (VariableType::Int(a), VariableType::IntVec(b), VariableType::Int(c)) => {
                        b[c as usize] = a;
                    }
                    (VariableType::Float(a), VariableType::FloatVec(b), VariableType::Int(c)) => {
                        b[c as usize] = a;
                    }
                    _ => return Err("Type mismatch".to_string()),
                }
            }
            Operation::Let(a, b) => {
                let a = variables
                    .get_mut(a)
                    .ok_or_else(|| format!("Variable {} not found", a))?;
                *a = b.clone();
            }
            Operation::Cpy(a, b) => {
                let b = variables
                    .get(b)
                    .ok_or_else(|| format!("Variable {} not found", b))?
                    .clone();
                let a = variables
                    .get_mut(a)
                    .ok_or_else(|| format!("Variable {} not found", a))?;
                *a = b;
            }
            Operation::Rnd(a, b, c) => {
                let b = variables
                    .get(b)
                    .ok_or_else(|| format!("Variable {} not found", b))?
                    .clone();
                let c = variables
                    .get(c)
                    .ok_or_else(|| format!("Variable {} not found", c))?
                    .clone();
                let a = variables
                    .get_mut(a)
                    .ok_or_else(|| format!("Variable {} not found", a))?;
                let mut rng = ::rand::thread_rng();
                match (a, b, c) {
                    (VariableType::Int(a), VariableType::Int(b), VariableType::Int(c)) => {
                        let l = std::cmp::min(b, c);
                        let r = std::cmp::max(b, c);
                        if l == r {
                            *a = l;
                        } else {
                            *a = rng.gen_range(l..r);
                        }
                    }
                    (VariableType::Float(a), VariableType::Float(b), VariableType::Float(c)) => {
                        let l;
                        let r;
                        if b < c {
                            l = b;
                            r = c;
                        } else {
                            l = c;
                            r = b;
                        }
                        if l == r {
                            *a = l;
                        } else {
                            *a = rng.gen_range(l..r);
                        }
                    }
                    _ => return Err("Type mismatch".to_string()),
                }
            }
            Operation::Shu(a) => {
                let a = variables
                    .get_mut(a)
                    .ok_or_else(|| format!("Variable {} not found", a))?;
                match a {
                    VariableType::StrVec(a) => a.shuffle(&mut ::rand::thread_rng()),
                    VariableType::IntVec(a) => a.shuffle(&mut ::rand::thread_rng()),
                    VariableType::FloatVec(a) => a.shuffle(&mut ::rand::thread_rng()),
                    _ => return Err("Type mismatch".to_string()),
                }
            }
            Operation::Inp(a) => {
                let input = variables
                    .get("input")
                    .ok_or("Variable input not found")?
                    .clone();
                let a = variables
                    .get_mut(a)
                    .ok_or_else(|| format!("Variable {} not found", a))?;
                match (a, input) {
                    (VariableType::Str(a), VariableType::Str(b)) => *a = b.clone(),
                    (VariableType::Int(a), VariableType::Str(b)) => {
                        *a = b.parse().map_err(|_| "Invalid input")?
                    }
                    (VariableType::Float(a), VariableType::Str(b)) => {
                        *a = b.parse().map_err(|_| "Invalid input")?
                    }
                    _ => return Err("Type mismatch".to_string()),
                }
            }
            Operation::Qry(a, b, c) => {
                let b = variables
                    .get(b)
                    .ok_or_else(|| format!("Variable {} not found", b))?
                    .clone();
                let c = variables
                    .get(c)
                    .ok_or_else(|| format!("Variable {} not found", c))?
                    .clone();
                let a = variables
                    .get_mut(a)
                    .ok_or_else(|| format!("Variable {} not found", a))?;
                match (a, b, c) {
                    (VariableType::Str(a), VariableType::StrDic(b), VariableType::Str(c)) => {
                        *a = b.get(&c).cloned().unwrap_or_else(|| "".to_string());
                    }
                    (VariableType::Int(a), VariableType::IntDic(b), VariableType::Str(c)) => {
                        *a = *b.get(&c).unwrap_or(&0);
                    }
                    (VariableType::Float(a), VariableType::FloatDic(b), VariableType::Str(c)) => {
                        *a = *b.get(&c).unwrap_or(&0.0);
                    }
                    _ => return Err("Type mismatch".to_string()),
                }
            }
            Operation::Ins(a, b, c) => {
                let a = variables
                    .get(a)
                    .ok_or_else(|| format!("Variable {} not found", a))?
                    .clone();
                let c = variables
                    .get(c)
                    .ok_or_else(|| format!("Variable {} not found", c))?
                    .clone();
                let b = variables
                    .get_mut(b)
                    .ok_or_else(|| format!("Variable {} not found", b))?;
                match (a, b, c) {
                    (VariableType::Str(a), VariableType::StrDic(b), VariableType::Str(c)) => {
                        b.insert(c.clone(), a.clone());
                    }
                    (VariableType::Int(a), VariableType::IntDic(b), VariableType::Str(c)) => {
                        b.insert(c.clone(), a.clone());
                    }
                    (VariableType::Float(a), VariableType::FloatDic(b), VariableType::Str(c)) => {
                        b.insert(c.clone(), a.clone());
                    }
                    _ => return Err("Type mismatch".to_string()),
                }
            }
        }
        Ok(())
    }
}

// 操作集合继承数组的方法
impl std::ops::Deref for Operations {
    type Target = Vec<Operation>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Operations {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Operations {
    pub fn new() -> Self {
        Operations(Vec::new())
    }

    // 检查操作集合（检测用，会报告所有错误）
    pub fn check(&self, variables: &Variables) -> Result<(), String> {
        let mut errors = Vec::new();
        for operation in &self.0 {
            if let Err(e) = operation.operation_check(variables) {
                errors.push(e);
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join(", "))
        }
    }

    // 计算操作集合
    pub fn calculate(&self, variables: &mut Variables) -> Result<(), String> {
        for operation in &self.0 {
            operation.calculate(variables)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 创建测试变量
    fn create_test_variables() -> Variables {
        let mut variables = Variables::new();
        variables.insert("int1".to_string(), VariableType::Int(1));
        variables.insert("int2".to_string(), VariableType::Int(2));
        variables.insert("ptr".to_string(), VariableType::Int(1));
        variables.insert("str1".to_string(), VariableType::Str("test".to_string()));
        variables.insert("str2".to_string(), VariableType::Str("ab".to_string()));
        variables.insert("float1".to_string(), VariableType::Float(1.0));
        variables.insert("float2".to_string(), VariableType::Float(2.0));
        variables.insert("int_vec".to_string(), VariableType::IntVec(vec![1, 2, 3]));
        variables.insert(
            "str_vec".to_string(),
            VariableType::StrVec(vec!["a".to_string(), "b".to_string()]),
        );
        variables.insert(
            "float_vec".to_string(),
            VariableType::FloatVec(vec![1.0, 2.0, 3.0]),
        );
        variables.insert(
            "str_dic".to_string(),
            VariableType::StrDic({
                let mut dic = std::collections::HashMap::new();
                dic.insert("a".to_string(), "b".to_string());
                dic
            }),
        );
        variables
    }

    // 测试操作检查
    #[test]
    fn test_operation_check() {
        let variables = create_test_variables();

        // Add
        assert_eq!(
            Operation::Add("int1".to_string(), "int1".to_string(), "int1".to_string())
                .operation_check(&variables),
            Ok(())
        );
        assert_eq!(
            Operation::Add("int1".to_string(), "int1".to_string(), "str1".to_string())
                .operation_check(&variables),
            Err("Type mismatch".to_string())
        );
        // Sub
        assert_eq!(
            Operation::Sub("int1".to_string(), "int1".to_string(), "int1".to_string())
                .operation_check(&variables),
            Ok(())
        );
        assert_eq!(
            Operation::Sub("int1".to_string(), "int1".to_string(), "str1".to_string())
                .operation_check(&variables),
            Err("Type mismatch".to_string())
        );
        // Mul
        assert_eq!(
            Operation::Mul("int1".to_string(), "int1".to_string(), "int1".to_string())
                .operation_check(&variables),
            Ok(())
        );
        assert_eq!(
            Operation::Mul("int1".to_string(), "int1".to_string(), "str1".to_string())
                .operation_check(&variables),
            Err("Type mismatch".to_string())
        );
        // Div
        assert_eq!(
            Operation::Div("int1".to_string(), "int1".to_string(), "int1".to_string())
                .operation_check(&variables),
            Ok(())
        );
        assert_eq!(
            Operation::Div("int1".to_string(), "int1".to_string(), "str1".to_string())
                .operation_check(&variables),
            Err("Type mismatch".to_string())
        );
        // Get
        assert_eq!(
            Operation::Get(
                "str1".to_string(),
                "str_vec".to_string(),
                "int1".to_string()
            )
            .operation_check(&variables),
            Ok(())
        );
        assert_eq!(
            Operation::Get(
                "int1".to_string(),
                "str_vec".to_string(),
                "int1".to_string()
            )
            .operation_check(&variables),
            Err("Type mismatch".to_string())
        );
        // Set
        assert_eq!(
            Operation::Set(
                "str1".to_string(),
                "str_vec".to_string(),
                "int1".to_string()
            )
            .operation_check(&variables),
            Ok(())
        );
        assert_eq!(
            Operation::Set(
                "int1".to_string(),
                "str_vec".to_string(),
                "int1".to_string()
            )
            .operation_check(&variables),
            Err("Type mismatch".to_string())
        );
        // Let
        assert_eq!(
            Operation::Let("int1".to_string(), VariableType::Int(1)).operation_check(&variables),
            Ok(())
        );
        assert_eq!(
            Operation::Let("int1".to_string(), VariableType::Str("a".to_string()))
                .operation_check(&variables),
            Err("Type mismatch".to_string())
        );
        // Cpy
        assert_eq!(
            Operation::Cpy("int1".to_string(), "int1".to_string()).operation_check(&variables),
            Ok(())
        );
        assert_eq!(
            Operation::Cpy("int1".to_string(), "str1".to_string()).operation_check(&variables),
            Err("Type mismatch".to_string())
        );
    }

    // 测试操作计算
    #[test]
    fn test_calculate() {
        let mut variables = create_test_variables();

        // Add
        assert_eq!(
            Operation::Add("int1".to_string(), "int1".to_string(), "int2".to_string())
                .calculate(&mut variables),
            Ok(())
        );
        assert_eq!(variables.get("int1"), Some(&VariableType::Int(3)));
        // Sub
        assert_eq!(
            Operation::Sub("int1".to_string(), "int1".to_string(), "int2".to_string())
                .calculate(&mut variables),
            Ok(())
        );
        assert_eq!(variables.get("int1"), Some(&VariableType::Int(1)));
        // Mul
        assert_eq!(
            Operation::Mul("int1".to_string(), "int1".to_string(), "int2".to_string())
                .calculate(&mut variables),
            Ok(())
        );
        assert_eq!(variables.get("int1"), Some(&VariableType::Int(2)));
        // Div
        assert_eq!(
            Operation::Div("int1".to_string(), "int1".to_string(), "int2".to_string())
                .calculate(&mut variables),
            Ok(())
        );
        assert_eq!(variables.get("int1"), Some(&VariableType::Int(1)));
        // Get
        assert_eq!(
            Operation::Get("str1".to_string(), "str_vec".to_string(), "ptr".to_string())
                .calculate(&mut variables),
            Ok(())
        );
        assert_eq!(
            variables.get("str1"),
            Some(&VariableType::Str("b".to_string()))
        );
        // Set
        assert_eq!(
            Operation::Set("str2".to_string(), "str_vec".to_string(), "ptr".to_string())
                .calculate(&mut variables),
            Ok(())
        );
        assert_eq!(
            variables.get("str_vec"),
            Some(&VariableType::StrVec(vec![
                "a".to_string(),
                "ab".to_string()
            ]))
        );
        // Let
        assert_eq!(
            Operation::Let("int1".to_string(), VariableType::Int(10)).calculate(&mut variables),
            Ok(())
        );
        assert_eq!(variables.get("int1"), Some(&VariableType::Int(10)));
        // Cpy
        assert_eq!(
            Operation::Cpy("int1".to_string(), "int2".to_string()).calculate(&mut variables),
            Ok(())
        );
        assert_eq!(variables.get("int1"), Some(&VariableType::Int(2)));
        // Rnd
        let _ = Operation::Add("int2".to_string(), "int1".to_string(), "int2".to_string())
            .calculate(&mut variables);
        assert_eq!(
            Operation::Rnd("int1".to_string(), "int1".to_string(), "int2".to_string())
                .calculate(&mut variables),
            Ok(())
        );
        assert!(matches!(variables.get("int1"), Some(VariableType::Int(_))));
        // Shu
        assert_eq!(
            Operation::Shu("str_vec".to_string()).calculate(&mut variables),
            Ok(())
        );
        // Qry
        Operation::Let("str1".to_string(), VariableType::Str("a".to_string()))
            .calculate(&mut variables)
            .unwrap();
        assert_eq!(
            Operation::Qry(
                "str1".to_string(),
                "str_dic".to_string(),
                "str1".to_string()
            )
            .calculate(&mut variables),
            Ok(())
        );
        assert_eq!(
            variables.get("str1"),
            Some(&VariableType::Str("b".to_string()))
        );

        print!("{:?}", variables);
    }
}
