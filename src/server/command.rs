use std::collections::HashSet;
use std::collections::HashMap;
use std::rc::Rc;
use crate::server::protocol::Protocol;
use crate::server::storage::Storage;

pub struct Command<T>
where
    T: Protocol + Storage,
{
    pub name: String,
    pub flags: HashSet<String>,
    pub proc: Rc<dyn Fn(&T) -> ()>,
}

// 定义 CommandManager 结构体
pub struct CommandManager<T>
where
    T: Protocol + Storage,
{
    commands: HashMap<String, Command<T>>,
}

impl<T> CommandManager<T>
where
    T: Protocol + Storage,
{
    // 创建一个新的 CommandManager 实例
    pub fn new() -> Self {
        CommandManager {
            commands: HashMap::new(),
        }
    }

    // 注册一个新的命令
    pub fn register(&mut self, cmd: Command<T>) {
        self.commands.insert(cmd.name.clone(), cmd);
    }

    // 根据命令名执行命令
    pub fn execute(&self, name: &str, ctx: &T) {
        // if let Some(cmd) = self.commands.get(name) {
        //     (cmd.proc)(ctx);
        // }
    }
}