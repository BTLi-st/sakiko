// Module: variable
use ::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum VariableType {
    Str(String),
    Int(i32),
    Float(f32),
    StrVec(Vec<String>),
    IntVec(Vec<i32>),
    FloatVec(Vec<f32>),
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Variables (pub std::collections::HashMap<String, VariableType>);

// 继承自 HashMap 的方法
impl std::ops::Deref for Variables {
    type Target = std::collections::HashMap<String, VariableType>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Variables {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Variables {
    pub fn new() -> Variables {
        Variables(std::collections::HashMap::new())
    }

    pub fn insert(&mut self, key: String, value: VariableType) {
        self.0.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&VariableType> {
        self.0.get(key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut VariableType> {
        self.0.get_mut(key)
    }

    pub fn has_input(&self) -> bool {
        self.0.contains_key("input")
    }
}

impl VariableType {
    pub fn is_same_type(&self, other: &VariableType) -> bool {
        matches!(
            (self, other),
            (VariableType::Str(_), VariableType::Str(_))
                | (VariableType::Int(_), VariableType::Int(_))
                | (VariableType::Float(_), VariableType::Float(_))
                | (VariableType::StrVec(_), VariableType::StrVec(_))
                | (VariableType::IntVec(_), VariableType::IntVec(_))
                | (VariableType::FloatVec(_), VariableType::FloatVec(_))
        )
    }

    pub fn is_number(&self) -> bool {
        matches!(self, VariableType::Int(_) | VariableType::Float(_))
    }

    pub fn is_vector(&self) -> bool {
        matches!(
            self,
            VariableType::StrVec(_) | VariableType::IntVec(_) | VariableType::FloatVec(_)
        )
    }

    pub fn to_string(&self) -> String {
        match self {
            VariableType::Str(s) => s.clone(),
            VariableType::Int(i) => i.to_string(),
            VariableType::Float(f) => format!("{:.3}", f),
            VariableType::StrVec(v) => v.join(","),
            VariableType::IntVec(v) => v.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(","),
            VariableType::FloatVec(v) => v.iter().map(|f| f.to_string()).collect::<Vec<String>>().join(","),
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_same_type() {
        // same type
        assert!(VariableType::Int(1).is_same_type(&VariableType::Int(2)));
        assert!(
            VariableType::Str("a".to_string()).is_same_type(&VariableType::Str("b".to_string()))
        );
        assert!(VariableType::Float(1.0).is_same_type(&VariableType::Float(2.0)));
        assert!(
            VariableType::IntVec(vec![1, 2, 3]).is_same_type(&VariableType::IntVec(vec![4, 5, 6]))
        );
        assert!(
            VariableType::StrVec(vec!["a".to_string(), "b".to_string()]).is_same_type(
                &VariableType::StrVec(vec!["c".to_string(), "d".to_string()])
            )
        );
        assert!(VariableType::FloatVec(vec![1.0, 2.0, 3.0])
            .is_same_type(&VariableType::FloatVec(vec![4.0, 5.0, 6.0])));
        // not same type
        assert!(!VariableType::Int(1).is_same_type(&VariableType::Str("a".to_string())));
    }
}