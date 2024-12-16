/// 客服机器人的核心逻辑
/// Session 结构体，用于保存会话状态
/// 使用 Arc 来共享配置，减少内存占用，同时避免多线程中的数据竞争
/// 支持同步和异步的 IO 操作
/// 为标准输入输出提供了直接的支持
use crate::config::SakikoConfig;
use crate::variable::{Variables, VariableType};
use regex::Regex;
use std::ops::Not;
use std::sync::Arc;
use tokio::io::{self, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

/// 会话结构体
#[derive(Debug, Clone)]
pub struct Session {
    config: Arc<SakikoConfig>,
    variables: Variables,
    now_step: String,
}

// 判断是否为正则表达式
fn is_regex(pattern: &str) -> bool {
    Regex::new(pattern).is_ok()
}

impl Session {
    /// 创建一个新的会话，传入配置
    pub fn new(config: Arc<SakikoConfig>) -> Session {
        let mut tmp = Session {
            variables: config.get_variables().clone(),
            config: Arc::clone(&config),
            now_step: config.get_start_step().to_string(),
        };
        tmp.variables
            .insert("input".to_string(), VariableType::Str("".to_string()));
        tmp
    }

    /// 获取机器人名
    pub fn get_bot_name(&self) -> &str {
        self.config.get_bot_name()
    }

    /// 判断是否结束
    pub fn is_end(&self) -> bool {
        self.now_step == "end"
    }

    /// 获取当前步骤
    pub fn output(&self) -> Result<String, &'static str> {
        let step = self.config.get_step(&self.now_step).ok_or("Invalid step")?;
        Ok(step.description.fmt(&self.variables)?)
    }

    /// 处理空输入情况，即可以自动跳转的情况
    pub fn handle_empty_input(&mut self) -> Result<bool, String> {
        let step = self.config.get_step(&self.now_step).ok_or("Invalid step")?;
        for transaction in &step.transaction {
            if transaction.pattern == "" && transaction.compares.calc(&self.variables)? {
                // Empty pattern
                transaction.operation.calculate(&mut self.variables)?;
                self.now_step = transaction.step.clone();
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// 处理空输出情况
    pub fn handle_empty_output(&mut self) -> Result<(), String> {
        loop {
            if self.now_step == "end" {
                break;
            }
            let step = self.config.get_step(&self.now_step).ok_or("Invalid step")?;
            if step.description.is_empty().not() {
                break;
            }

            let mut found_valid_transaction = false;
            for transaction in &step.transaction {
                if transaction.pattern == "" && transaction.compares.calc(&self.variables)? {
                    // Empty pattern
                    transaction.operation.calculate(&mut self.variables)?;
                    self.now_step = transaction.step.clone();
                    found_valid_transaction = true;
                    break;
                }
            }

            if !found_valid_transaction {
                return Err("No valid transaction found for empty pattern".to_string());
            }
        }
        Ok(())
    }

    /// 处理输入
    pub fn handle_input(&mut self, input: &str) -> Result<(), String> {
        let step = self.config.get_step(&self.now_step).ok_or("Invalid step")?;
        self.variables
            .insert("input".to_string(), VariableType::Str(input.to_string()));
        for transaction in &step.transaction {
            if transaction.pattern == "" {
                // Empty pattern
                continue;
            }
            if is_regex(&transaction.pattern) {
                let re = Regex::new(&transaction.pattern).map_err(|_| "Invalid pattern")?;
                if re.is_match(input) && transaction.compares.calc(&self.variables)? {
                    transaction.operation.calculate(&mut self.variables)?;
                    self.now_step = transaction.step.clone();
                    return Ok(());
                }
            } else {
                if transaction.pattern == input && transaction.compares.calc(&self.variables)? {
                    transaction.operation.calculate(&mut self.variables)?;
                    self.now_step = transaction.step.clone();
                    return Ok(());
                }
            }
        }
        Err("Invalid input".to_string())
    }

    /// 异步版本输出
    pub async fn output_async<W: AsyncWrite + Unpin>(&self, mut writer: W) -> io::Result<()> {
        let output = self
            .output()
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        writer.write_all(output.as_bytes()).await?;
        Ok(())
    }

    /// 异步版本处理输入
    pub async fn handle_input_async<R: AsyncRead + Unpin>(
        &mut self,
        mut reader: R,
    ) -> io::Result<()> {
        let mut input = String::new();
        reader.read_to_string(&mut input).await?;
        self.handle_input(&input)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        Ok(())
    }

    /// 无 IO 步骤
    /// 返回 1 表示 Session 结束，返回 0 表示 Session 未结束
    pub fn need_stop(&mut self) -> Result<bool, String> {
        if self.is_end() {
            return Ok(true);
        }
        self.handle_empty_output()?;
        if self.is_end() {
            return Ok(true);
        }
        Ok(false)
    }

    /// 标准输入输出版本（同步）
    pub fn run_stdio(&mut self) -> Result<(), String> {
        loop {
            if self.need_stop()? {
                break;
            }
            println!("[{}]", self.get_bot_name());
            println!("{}", self.output()?);
            if self.handle_empty_input()? {
                continue;
            }
            println!("[user]");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).map_err(|_| "Failed to read line")?;
            self.handle_input(input.trim())?;
        }
        Ok(())
    }
}
