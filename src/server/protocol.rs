pub trait Protocol {
    fn name(&self) -> &'static str;
    fn supply(&mut self, data: &[u8]);
    fn parse(&mut self) -> Result<Vec<String>, String>;
}

// 实现 ProtocolImpl 类型
pub struct ProtocolImpl {
    buffer: Vec<u8>,
}

impl ProtocolImpl {
    // 构造函数，初始化 ProtocolImpl 实例
    pub fn new() -> Self {
        ProtocolImpl {
            buffer: Vec::new(),
        }
    }
}

impl Protocol for ProtocolImpl {
    fn name(&self) -> &'static str {
        "ProtocolImpl"
    }

    fn supply(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
    }

    fn parse(&mut self) -> Result<Vec<String>, String> {
        let result = Vec::new();
        Ok(result)
    }
}
