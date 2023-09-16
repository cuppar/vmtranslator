use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CommandType {
    Arithmetic,
    Push,
    Pop,
    Label,
    Goto,
    If,
    Function,
    Return,
    Call,
}

impl CommandType {
    fn is_arithmetic(cmd: &str) -> bool {
        matches!(cmd, "add" | "sub" | "neg" | "eq" | "gt" | "lt" | "and" | "or" | "not")
    }
    fn is_push(cmd: &str) -> bool {
        cmd.starts_with("push")
    }
    fn is_pop(cmd: &str) -> bool {
        cmd.starts_with("pop")
    }
    fn is_label(cmd: &str) -> bool {
        cmd.starts_with("label")
    }
    fn is_goto(cmd: &str) -> bool {
        cmd.starts_with("goto")
    }
    fn is_if(cmd: &str) -> bool {
        cmd.starts_with("if-goto")
    }
    fn is_function(cmd: &str) -> bool {
        cmd.starts_with("function")
    }
    fn is_return(cmd: &str) -> bool {
        matches!(cmd, "return")
    }
    fn is_call(cmd: &str) -> bool {
        cmd.starts_with("call")
    }

    fn get_type(cmd: &str) -> Self {
        use CommandType::*;
        if Self::is_arithmetic(cmd) {
            return Arithmetic;
        }
        if Self::is_push(cmd) {
            return Push;
        }
        if Self::is_pop(cmd) {
            return Pop;
        }
        if Self::is_label(cmd) {
            return Label;
        }
        if Self::is_goto(cmd) {
            return Goto;
        }
        if Self::is_if(cmd) {
            return If;
        }
        if Self::is_function(cmd) {
            return Function;
        }
        if Self::is_return(cmd) {
            return Return;
        }
        if Self::is_call(cmd) {
            return Call;
        }

        panic!("unknow command");
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Canmand {
    pub cmd_type: CommandType,
    cmd_raw: String,
}

pub struct Parser {
    pub next_cmd_number: usize,
    lines: Vec<String>,
    next_line_number: usize,
    current_cmd: Option<Canmand>,
}

impl Parser {
    pub fn new(path: &Path) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let lines = contents.lines().map(|s| String::from(s)).collect();

        Ok(Self {
            lines,
            next_line_number: 0,
            next_cmd_number: 0,
            current_cmd: None,
        })
    }

    pub fn has_more_lines(&self) -> bool {
        self.next_line_number < self.lines.len()
    }

    pub fn advance(&mut self) {
        loop {
            if !self.has_more_lines() {
                return;
            }
            let mut line = self.lines[self.next_line_number].clone();
            self.next_line_number += 1;

            if let Some(index) = line.find("//") {
                line = line[..index].to_string()
            }
            let line = line.trim();
            if line != "" {
                let cmd_raw = line.to_string();
                let cmd_type = CommandType::get_type(&cmd_raw);
                self.next_cmd_number += 1;
                self.current_cmd = Some(Canmand { cmd_type, cmd_raw });
                return;
            }
        }
    }

    pub fn get_cmd_type(&self) -> Option<CommandType> {
        self.current_cmd.clone().map(|cmd| cmd.cmd_type)
    }

    pub fn cmd(&self) -> String {
        assert!(
            self.current_cmd.is_some(),
            "Can't call cmd() when have no command"
        );
        let cmd = self.current_cmd.clone().unwrap();
        let splited = cmd.cmd_raw.split(" ").collect::<Vec<_>>();
        splited[0].to_string()
    }

    pub fn arg1(&self) -> String {
        use CommandType::*;
        match self.get_cmd_type() {
            Some(Arithmetic) => self.current_cmd.clone().unwrap().cmd_raw,
            Some(Push) | Some(Pop) | Some(Label) | Some(Goto) | Some(If) | Some(Function)
            | Some(Call) => {
                let cmd_raw = self.current_cmd.clone().unwrap().cmd_raw;
                let splited = cmd_raw.split(" ").collect::<Vec<_>>();
                if splited.len() < 2 {
                    panic!("push/pop/label/goto/if-goto/function/call need arg1");
                }
                splited[1].to_string()
            }
            Some(Return) => panic!("Can't call arg1() in `RETURN` command"),
            None => panic!("Can't call arg1() when have no command"),
        }
    }

    pub fn arg2(&self) -> i32 {
        use CommandType::*;
        match self.get_cmd_type() {
            Some(Arithmetic) | Some(Goto) | Some(If) | Some(Label) | Some(Return) => {
                panic!("Should call arg2() in `PUSH` `POP` `FUNCTION` or `CALL` command")
            }
            Some(Push) | Some(Pop) | Some(Function) | Some(Call) => {
                let cmd_raw = self.current_cmd.clone().unwrap().cmd_raw;
                let splited = cmd_raw.split(" ").collect::<Vec<_>>();
                if splited.len() < 3 {
                    panic!("push/pop/function/call need arg2");
                }
                if let Ok(arg2) = splited[2].parse::<i32>() {
                    return arg2;
                } else {
                    panic!("arg2 need a int number");
                }
            }
            None => panic!("Can't call arg2() when have no command"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_file::*;

    use super::*;

    #[test]
    fn test_lines() -> io::Result<()> {
        let test_file = TestFile::new()?;
        let parser = Parser::new(&Path::new(&test_file.path))?;

        assert_eq!(parser.lines.len(), test_file.lines.len());

        Ok(())
    }

    #[test]
    fn test_has_more_lines() -> io::Result<()> {
        let test_file = TestFile::new()?;
        let mut parser = Parser::new(&Path::new(&test_file.path))?;

        assert_eq!(parser.next_line_number, 0);

        parser.next_line_number = test_file.lines.len() - 1;
        assert!(parser.has_more_lines());

        parser.next_line_number = test_file.lines.len();
        assert!(!parser.has_more_lines());

        Ok(())
    }

    #[test]
    fn test_empty_file() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        let parser = Parser::new(&Path::new(&test_file.path))?;

        assert_eq!(parser.current_cmd, None);
        assert_eq!(parser.next_line_number, 0);
        assert_eq!(parser.next_cmd_number, 0);
        assert!(!parser.has_more_lines());

        Ok(())
    }

    #[test]
    fn test_advance_skip_comment() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("add")?;
        test_file.add_line("//comment1")?;
        test_file.add_line("push local 1")?;
        let mut parser = Parser::new(&Path::new(&test_file.path))?;

        assert_eq!(parser.current_cmd, None);

        parser.advance();
        assert_eq!(parser.current_cmd.clone().unwrap().cmd_raw, "add");

        parser.advance();
        assert_eq!(parser.current_cmd.clone().unwrap().cmd_raw, "push local 1");

        Ok(())
    }

    #[test]
    fn test_advance_next_cmd_and_line_number() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("add")?;
        test_file.add_line("//comment1")?;
        test_file.add_line("push local 1")?;
        let mut parser = Parser::new(&Path::new(&test_file.path))?;

        assert_eq!(parser.next_cmd_number, 0);
        assert_eq!(parser.next_line_number, 0);

        parser.advance();
        assert_eq!(parser.next_cmd_number, 1);
        assert_eq!(parser.next_line_number, 1);

        parser.advance();
        assert_eq!(parser.next_cmd_number, 2);
        assert_eq!(parser.next_line_number, 3);

        Ok(())
    }

    #[test]
    #[should_panic = "unknow command"]
    fn test_get_cmd_type_unknow() {
        let mut test_file = TestFile::new().unwrap();
        test_file.clear().unwrap();
        test_file.add_line("???").unwrap();
        let mut parser = Parser::new(&Path::new(&test_file.path)).unwrap();

        assert_eq!(parser.get_cmd_type(), None);

        // should panic
        parser.advance();
    }

    #[test]
    fn test_get_cmd_type_arithmetic() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("add")?;
        test_file.add_line("sub")?;
        test_file.add_line("neg")?;
        test_file.add_line("eq")?;
        test_file.add_line("gt")?;
        test_file.add_line("lt")?;
        test_file.add_line("and")?;
        test_file.add_line("or")?;
        test_file.add_line("not")?;
        let mut parser = Parser::new(&Path::new(&test_file.path))?;

        assert_eq!(parser.get_cmd_type(), None);

        loop {
            if !parser.has_more_lines() {
                return Ok(());
            }
            parser.advance();
            assert_eq!(parser.get_cmd_type(), Some(CommandType::Arithmetic));
        }
    }
    #[test]
    fn test_get_cmd_type_push() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("push local 1")?;
        test_file.add_line("push static 2")?;

        let mut parser = Parser::new(&Path::new(&test_file.path))?;

        assert_eq!(parser.get_cmd_type(), None);

        loop {
            if !parser.has_more_lines() {
                return Ok(());
            }
            parser.advance();
            assert_eq!(parser.get_cmd_type(), Some(CommandType::Push));
        }
    }

    #[test]
    fn test_get_cmd_type_pop() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("pop local 1")?;
        test_file.add_line("pop static 2")?;

        let mut parser = Parser::new(&Path::new(&test_file.path))?;

        assert_eq!(parser.get_cmd_type(), None);

        loop {
            if !parser.has_more_lines() {
                return Ok(());
            }
            parser.advance();
            assert_eq!(parser.get_cmd_type(), Some(CommandType::Pop));
        }
    }

    #[test]
    fn test_get_cmd_type_label() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("label LABEL1")?;
        test_file.add_line("label LABEL2")?;

        let mut parser = Parser::new(&Path::new(&test_file.path))?;

        assert_eq!(parser.get_cmd_type(), None);

        loop {
            if !parser.has_more_lines() {
                return Ok(());
            }
            parser.advance();
            assert_eq!(parser.get_cmd_type(), Some(CommandType::Label));
        }
    }

    #[test]
    fn test_get_cmd_type_goto() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("goto LABEL1")?;
        test_file.add_line("goto LABEL2")?;

        let mut parser = Parser::new(&Path::new(&test_file.path))?;

        assert_eq!(parser.get_cmd_type(), None);

        loop {
            if !parser.has_more_lines() {
                return Ok(());
            }
            parser.advance();
            assert_eq!(parser.get_cmd_type(), Some(CommandType::Goto));
        }
    }

    #[test]
    fn test_get_cmd_type_if() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("if-goto LABEL1")?;
        test_file.add_line("if-goto LABEL2")?;

        let mut parser = Parser::new(&Path::new(&test_file.path))?;

        assert_eq!(parser.get_cmd_type(), None);

        loop {
            if !parser.has_more_lines() {
                return Ok(());
            }
            parser.advance();
            assert_eq!(parser.get_cmd_type(), Some(CommandType::If));
        }
    }

    #[test]
    fn test_get_cmd_type_function() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("function f1 0")?;
        test_file.add_line("function f2 3")?;

        let mut parser = Parser::new(&Path::new(&test_file.path))?;

        assert_eq!(parser.get_cmd_type(), None);

        loop {
            if !parser.has_more_lines() {
                return Ok(());
            }
            parser.advance();
            assert_eq!(parser.get_cmd_type(), Some(CommandType::Function));
        }
    }

    #[test]
    fn test_get_cmd_type_call() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("call f1 0")?;
        test_file.add_line("call f2 3")?;

        let mut parser = Parser::new(&Path::new(&test_file.path))?;

        assert_eq!(parser.get_cmd_type(), None);

        loop {
            if !parser.has_more_lines() {
                return Ok(());
            }
            parser.advance();
            assert_eq!(parser.get_cmd_type(), Some(CommandType::Call));
        }
    }

    #[test]
    fn test_get_cmd_type_return() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("return")?;

        let mut parser = Parser::new(&Path::new(&test_file.path))?;

        assert_eq!(parser.get_cmd_type(), None);

        parser.advance();
        assert_eq!(parser.get_cmd_type(), Some(CommandType::Return));

        Ok(())
    }

    #[test]
    #[should_panic = "Can't call arg1() in `RETURN` command"]
    fn test_arg1_cant_be_call_in_return() {
        let mut test_file = TestFile::new().unwrap();
        test_file.clear().unwrap();
        test_file.add_line("return").unwrap();

        let mut parser = Parser::new(&Path::new(&test_file.path)).unwrap();

        parser.advance();

        assert_eq!(parser.get_cmd_type(), Some(CommandType::Return));
        // should panic
        parser.arg1();
    }

    #[test]
    #[should_panic = "Can't call arg1() when have no command"]
    fn test_arg1_cant_be_call_in_no_cmd() {
        let mut test_file = TestFile::new().unwrap();
        test_file.clear().unwrap();

        let parser = Parser::new(&Path::new(&test_file.path)).unwrap();

        assert_eq!(parser.get_cmd_type(), None);
        // should panic
        parser.arg1();
    }

    #[test]
    fn test_arg1_arithmetic() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("add")?;
        test_file.add_line("not")?;

        let mut parser = Parser::new(&Path::new(&test_file.path))?;

        parser.advance();
        assert_eq!(parser.arg1(), "add");

        parser.advance();
        assert_eq!(parser.arg1(), "not");

        Ok(())
    }

    #[test]
    fn test_arg1_not_arithmetic() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("push local 1")?;
        test_file.add_line("pop static 2")?;
        test_file.add_line("label LABEL1")?;
        test_file.add_line("goto LABEL2")?;
        test_file.add_line("if-goto LABEL3")?;
        test_file.add_line("function functionName 3")?;
        test_file.add_line("call functionName 3")?;

        let mut parser = Parser::new(&Path::new(&test_file.path))?;

        parser.advance();
        assert_eq!(parser.arg1(), "local");

        parser.advance();
        assert_eq!(parser.arg1(), "static");

        parser.advance();
        assert_eq!(parser.arg1(), "LABEL1");

        parser.advance();
        assert_eq!(parser.arg1(), "LABEL2");

        parser.advance();
        assert_eq!(parser.arg1(), "LABEL3");

        parser.advance();
        assert_eq!(parser.arg1(), "functionName");

        parser.advance();
        assert_eq!(parser.arg1(), "functionName");

        Ok(())
    }

    #[test]
    #[should_panic = "Should call arg2() in `PUSH` `POP` `FUNCTION` or `CALL` command"]
    fn test_arg2_cant_be_call_in_arithmetic() {
        let mut test_file = TestFile::new().unwrap();
        test_file.clear().unwrap();
        test_file.add_line("add").unwrap();

        let mut parser = Parser::new(&Path::new(&test_file.path)).unwrap();

        parser.advance();
        assert_eq!(parser.get_cmd_type(), Some(CommandType::Arithmetic));
        // should panic
        parser.arg2();
    }

    #[test]
    #[should_panic = "Can't call arg2() when have no command"]
    fn test_arg2_cant_be_call_in_no_cmd() {
        let mut test_file = TestFile::new().unwrap();
        test_file.clear().unwrap();

        let parser = Parser::new(&Path::new(&test_file.path)).unwrap();

        assert_eq!(parser.get_cmd_type(), None);
        // should panic
        parser.arg2();
    }

    #[test]
    #[should_panic = "Should call arg2() in `PUSH` `POP` `FUNCTION` or `CALL` command"]
    fn test_arg2_cant_be_call_in_label() {
        let mut test_file = TestFile::new().unwrap();
        test_file.clear().unwrap();
        test_file.add_line("label").unwrap();

        let mut parser = Parser::new(&Path::new(&test_file.path)).unwrap();

        parser.advance();
        assert_eq!(parser.get_cmd_type(), Some(CommandType::Label));
        // should panic
        parser.arg2();
    }
    #[test]
    #[should_panic = "Should call arg2() in `PUSH` `POP` `FUNCTION` or `CALL` command"]
    fn test_arg2_cant_be_call_in_goto() {
        let mut test_file = TestFile::new().unwrap();
        test_file.clear().unwrap();
        test_file.add_line("goto").unwrap();

        let mut parser = Parser::new(&Path::new(&test_file.path)).unwrap();

        parser.advance();
        assert_eq!(parser.get_cmd_type(), Some(CommandType::Goto));
        // should panic
        parser.arg2();
    }
    #[test]
    #[should_panic = "Should call arg2() in `PUSH` `POP` `FUNCTION` or `CALL` command"]
    fn test_arg2_cant_be_call_in_if() {
        let mut test_file = TestFile::new().unwrap();
        test_file.clear().unwrap();
        test_file.add_line("if-goto LABEL").unwrap();

        let mut parser = Parser::new(&Path::new(&test_file.path)).unwrap();

        parser.advance();
        assert_eq!(parser.get_cmd_type(), Some(CommandType::If));
        // should panic
        parser.arg2();
    }
    #[test]
    #[should_panic = "Should call arg2() in `PUSH` `POP` `FUNCTION` or `CALL` command"]
    fn test_arg2_cant_be_call_in_return() {
        let mut test_file = TestFile::new().unwrap();
        test_file.clear().unwrap();
        test_file.add_line("return").unwrap();

        let mut parser = Parser::new(&Path::new(&test_file.path)).unwrap();

        parser.advance();
        assert_eq!(parser.get_cmd_type(), Some(CommandType::Return));
        // should panic
        parser.arg2();
    }

    #[test]
    fn test_arg2_in_push_and_pop() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("push local 1")?;
        test_file.add_line("pop static 2")?;

        let mut parser = Parser::new(&Path::new(&test_file.path))?;

        parser.advance();
        assert_eq!(parser.arg2(), 1);

        parser.advance();
        assert_eq!(parser.arg2(), 2);

        Ok(())
    }

    #[test]
    fn test_arg2_in_function_and_call() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("function functionName 3")?;
        test_file.add_line("call functionName 3")?;

        let mut parser = Parser::new(&Path::new(&test_file.path))?;

        parser.advance();
        assert_eq!(parser.arg2(), 3);

        parser.advance();
        assert_eq!(parser.arg2(), 3);

        Ok(())
    }

    #[test]
    fn test_cmd() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("push local 1")?;
        test_file.add_line("pop static 2")?;

        let mut parser = Parser::new(&Path::new(&test_file.path))?;

        parser.advance();
        assert_eq!(parser.cmd(), "push");

        parser.advance();
        assert_eq!(parser.cmd(), "pop");

        Ok(())
    }

    #[test]
    #[should_panic = "Can't call cmd() when have no command"]
    fn test_cmd_panic_in_no_cmd() {
        let mut test_file = TestFile::new().unwrap();
        test_file.clear().unwrap();

        let parser = Parser::new(&Path::new(&test_file.path)).unwrap();

        // should panic
        parser.cmd();
    }
}
