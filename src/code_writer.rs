use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{self, Write},
    path::Path,
};

pub struct CodeWriter {
    file: File,
    mem_seg_map: HashMap<String, String>,
}

impl CodeWriter {
    pub fn new(path: &Path) -> io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;
        let mem_seg_map = HashMap::from([
            ("local".to_string(), "LCL".to_string()),
            ("argument".to_string(), "ARG".to_string()),
            ("this".to_string(), "THIS".to_string()),
            ("that".to_string(), "THAT".to_string()),
            ("temp".to_string(), "TEMP".to_string()),
        ]);

        Ok(Self { file, mem_seg_map })
    }

    fn _write_arithmetic(&mut self, cmd: &str) -> String {
        match cmd {
            "add" => {
                String::new()
                    + &self._write_pop("temp", 0)
                    + &self._write_pop("temp", 1)
                    + "// start ======= temp0 = temp1 + temp0\n"
                    + "@TEMP\n"
                    + "A=A+1\n"
                    + "D=M\n"
                    + "@TEMP\n"
                    + "M=D+M\n"
                    + "// end ======= temp0 = temp1 + temp0\n"
                    + "\n"
                    + &&self._write_push("temp", 0)
            }
            _ => todo!(),
        }
    }

    pub fn write_arithmetic(&mut self, cmd: &str) -> io::Result<()> {
        let buf = self._write_arithmetic(cmd);
        self.file.write_all(buf.as_bytes())?;
        Ok(())
    }

    fn _write_push_pop(&mut self, cmd: &str, arg1: &str, arg2: i32) -> String {
        match cmd {
            "push" => self._write_push(arg1, arg2),
            "pop" => self._write_pop(arg1, arg2),
            _ => panic!("push/pop cmd snytax error"),
        }
    }

    fn _write_push(&mut self, arg1: &str, arg2: i32) -> String {
        match arg1 {
            "constant" => {
                String::new()
                    + &format!("// start ======== push {arg1} {arg2}\n")
                    + &format!("// D={arg2}\n")
                    + &format!("@{arg2}\n")
                    + &"D=A\n"
                    + &format!("// stack[SP]=D\n")
                    + &"@SP\n"
                    + &"A=M\n"
                    + &"M=D\n"
                    + &format!("// SP++\n")
                    + &"@SP\n"
                    + &"M=M+1\n"
                    + &format!("// end ======== push {arg1} {arg2}\n")
                    + &"\n"
            }
            "pointer" => {
                //todo
                String::new()
                    + &format!("// start ======== push {arg1} {arg2}\n")
                    + &format!("// D={arg2}\n")
                    + &format!("@{arg2}\n")
                    + &"D=A\n"
                    + &format!("// stack[SP]=D\n")
                    + &"@SP\n"
                    + &"A=M\n"
                    + &"M=D\n"
                    + &format!("// SP++\n")
                    + &"@SP\n"
                    + &"M=M+1\n"
                    + &format!("// end ======== push {arg1} {arg2}\n")
                    + &"\n"
            }
            _ => {
                assert!(self.mem_seg_map.contains_key(arg1));
                let arg1 = self.mem_seg_map[arg1].clone();
                String::new()
                    + &format!("// start ======== push {arg1} {arg2}\n")
                    + &format!("// D={arg1}+{arg2}\n")
                    + &format!("@{arg1}\n")
                    + &"D=A\n"
                    + &format!("@{arg2}\n")
                    + &"D=D+A\n"
                    + &"A=D\n"
                    + &"D=M\n"
                    + &format!("// stack[SP]=D\n")
                    + &"@SP\n"
                    + &"A=M\n"
                    + &"M=D\n"
                    + &format!("// SP++\n")
                    + &"@SP\n"
                    + &"M=M+1\n"
                    + &format!("// end ======== push {arg1} {arg2}\n")
                    + &"\n"
            }
        }
    }

    fn _write_pop(&mut self, arg1: &str, arg2: i32) -> String {
        match arg1 {
            "pointer" => {
                //todo
                String::new()
                    + &format!("// start ======== pop {arg1} {arg2}\n")
                    + &format!("// SP--\n")
                    + &"@SP\n"
                    + &"M=M-1\n"
                    + &format!("// R13=addr({arg1}+{arg2})\n")
                    + &format!("@{arg1}\n")
                    + &"D=A\n"
                    + &format!("@{arg2}\n")
                    + &"D=D+A\n"
                    + &"@R13\n"
                    + &"M=D\n"
                    + &format!("// D=stack[SP]\n")
                    + &"@SP\n"
                    + &"A=M\n"
                    + &"D=M\n"
                    + &format!("// R13=D\n")
                    + &"@R13\n"
                    + &"A=M\n"
                    + &"M=D\n"
                    + &format!("// end ======== pop {arg1} {arg2}\n")
                    + &"\n"
            }
            _ => {
                assert!(self.mem_seg_map.contains_key(arg1));
                let arg1 = self.mem_seg_map[arg1].clone();
                String::new()
                    + &format!("// start ======== pop {arg1} {arg2}\n")
                    + &format!("// SP--\n")
                    + &"@SP\n"
                    + &"M=M-1\n"
                    + &format!("// R13=addr({arg1}+{arg2})\n")
                    + &format!("@{arg1}\n")
                    + &"D=A\n"
                    + &format!("@{arg2}\n")
                    + &"D=D+A\n"
                    + &"@R13\n"
                    + &"M=D\n"
                    + &format!("// D=stack[SP]\n")
                    + &"@SP\n"
                    + &"A=M\n"
                    + &"D=M\n"
                    + &format!("// *R13=D\n")
                    + &"@R13\n"
                    + &"A=M\n"
                    + &"M=D\n"
                    + &format!("// end ======== pop {arg1} {arg2}\n")
                    + &"\n"
            }
        }
    }

    pub fn write_push_pop(&mut self, cmd: &str, arg1: &str, arg2: i32) -> io::Result<()> {
        let buf = self._write_push_pop(cmd, arg1, arg2);
        self.file.write_all(buf.as_bytes())?;
        Ok(())
    }

    pub fn close(&mut self) -> io::Result<()> {
        let buf =
            String::new() + &format!("// end the program\n") + &"(END)\n" + &"@END\n" + &"0;JMP\n";
        self.file.write_all(buf.as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_write_to_file() -> io::Result<()> {
        let file_path = Path::new("./test.asm");
        let mut code_writer = CodeWriter::new(file_path)?;
        code_writer.write_arithmetic("add")?;
        code_writer.close()?;
        fs::remove_file(file_path)?;
        Ok(())
    }
}