use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::server::protocol::Protocol;
use crate::server::command::CommandManager;
use crate::server::storage::Storage;

pub struct Client<P: Protocol> {
    stream: TcpStream,
    protocol: P,
}

impl<P: Protocol> Storage for Client<P> {
    fn store_data(&mut self, data: &str) {
        // 实现存储逻辑，这里只是示例，可按需修改
        println!("Storing data: {}", data);
    }
}

impl<P: Protocol> Protocol for Client<P> {
    fn name(&self) -> &'static str {
        self.protocol.name()
    }

    fn supply(&mut self, data: &[u8]) {
        self.protocol.supply(data);
    }

    fn parse(&mut self) -> Result<Vec<String>, String> {
        self.protocol.parse()
    }
}

impl<P: Protocol> Client<P> {
    /// 通过 TcpStream 初始化 Client 实例
    pub fn new(stream: TcpStream, protocol: P) -> Self {
        Client { stream, protocol }
    }

    /// 异步读取数据并将其写回
    pub async fn process_command(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut buffer = [0; 1024];
        // 异步读取数据
        let n = self.stream.read(&mut buffer).await?;
        if n == 0 {
            return Ok(());
        }
        self.protocol.supply(&buffer[0..n]);
        let result = self.protocol.parse()?;
        let cm = CommandManager::new();
        if let Ok(results) = self.protocol.parse() {
            cm.execute(results[0].as_str(), self);
            // for result in results {
            //     // 这里简单地将处理结果写回客户端，实际中可按需修改
            //     self.stream.write_all(result.as_bytes()).await?;
            // }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::{TcpListener, TcpStream};
    use crate::server::protocol::ProtocolImpl;

    #[tokio::test]
    async fn test_client_process_command() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // 启动一个简单的 TCP 服务器用于测试
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;

        // 启动一个任务来接受客户端连接
        let server_task = tokio::spawn(async move {
            let (stream, _) = listener.accept().await?;
            let protocol = ProtocolImpl::new();
            let mut client = Client::new(stream, protocol);
            client.process_command().await?;
            Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
        });

        // 客户端连接到服务器
        let mut client_stream = TcpStream::connect(addr).await?;

        // 发送测试数据
        let test_data = b"Hello, Server!";
        client_stream.write_all(test_data).await?;

        // 读取服务器回显的数据
        let mut buffer = vec![0; test_data.len()];
        client_stream.read_exact(&mut buffer).await?;

        // 验证回显数据是否正确
        assert_eq!(buffer, test_data);

        // 等待服务器任务完成
        server_task.await??;

        Ok(())
    }
}