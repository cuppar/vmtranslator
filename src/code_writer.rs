use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{self, Write},
    path::Path,
};

pub struct CodeWriter {
    file: File,
    filename: String,
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
        ]);

        let filename = path
            .file_name()
            .ok_or(io::Error::new(io::ErrorKind::NotFound, "no filename"))?
            .to_str()
            .unwrap()
            .to_string();

        Ok(Self {
            file,
            filename,
            mem_seg_map,
        })
    }

    fn _write_arithmetic(&mut self, cmd: &str, id: &str) -> String {
        const TEMP_BASE: i32 = 5;
        match cmd {
            "add" => {
                String::new()
                    + "// start ======= add\n"
                    + &self._write_pop("temp", 0)
                    + &self._write_pop("temp", 1)
                    + "// start ======= temp0 = temp1 + temp0\n"
                    + &format!("@{TEMP_BASE}\n")
                    + "A=A+1\n"
                    + "D=M\n"
                    + &format!("@{TEMP_BASE}\n")
                    + "M=D+M\n" // D: x, M: y
                    + "// end ======= temp0 = temp1 + temp0\n"
                    + "\n"
                    + &&self._write_push("temp", 0)
                    + "// end ======= add\n"
                    + "\n"
            }
            "sub" => {
                String::new()
                    + "// start ======= sub\n"
                    + &self._write_pop("temp", 0)
                    + &self._write_pop("temp", 1)
                    + "// start ======= temp0 = temp1 - temp0\n"
                    + &format!("@{TEMP_BASE}\n")
                    + "A=A+1\n"
                    + "D=M\n"
                    + &format!("@{TEMP_BASE}\n")
                    + "M=D-M\n" // D: x, M: y
                    + "// end ======= temp0 = temp1 - temp0\n"
                    + "\n"
                    + &&self._write_push("temp", 0)
                    + "// end ======= sub\n"
                    + "\n"
            }
            "neg" => {
                String::new()
                    + "// start ======= neg\n"
                    + &self._write_pop("temp", 0)
                    + "// start ======= temp0 = -temp0\n"
                    + &format!("@{TEMP_BASE}\n")
                    + "M=-M\n"
                    + "// end ======= temp0 = -temp0\n"
                    + "\n"
                    + &&self._write_push("temp", 0)
                    + "// end ======= neg\n"
                    + "\n"
            }
            "eq" => {
                String::new()
                    + "// start ======= eq\n"
                    + &self._write_pop("temp", 0)
                    + &self._write_pop("temp", 1)
                    + "// start ======= temp0 := temp1 == temp0\n"
                    + &format!("@{TEMP_BASE}\n")
                    + "A=A+1\n"
                    + "D=M\n"
                    + &format!("@{TEMP_BASE}\n")
                    + "M=D-M\n" // D: x, M: y
                    + "D=M\n" // if D(x-y)==0, M=true(-1), else M=false(0)
                    + "M=0\n" // bool=false(0)
                    + &format!("@HIT_{}\n", id)
                    + "D;JEQ\n"
                    + &format!("@CONTINUE_{}\n", id)
                    + "0;JMP\n"
                    + &format!("(HIT_{})\n", id) // test hit, bool=true(-1)
                    + &format!("@{TEMP_BASE}\n")
                    + "M=-1\n"
                    + &format!("(CONTINUE_{})\n", id)
                    + "// end ======= temp0 := temp1 == temp0\n"
                    + "\n"
                    + &&self._write_push("temp", 0)
                    + "// end ======= eq\n"
                    + "\n"
            }
            "gt" => {
                String::new()
                    + "// start ======= gt\n"
                    + &self._write_pop("temp", 0)
                    + &self._write_pop("temp", 1)
                    + "// start ======= temp0 := temp1 > temp0\n"
                    + &format!("@{TEMP_BASE}\n")
                    + "A=A+1\n"
                    + "D=M\n"
                    + &format!("@{TEMP_BASE}\n")
                    + "M=D-M\n" // D: x, M: y
                    + "D=M\n" // if D(x-y) > 0, M=true(-1), else M=false(0)
                    + "M=0\n" // bool=false(0)
                    + &format!("@HIT_{}\n", id)
                    + "D;JGT\n"
                    + &format!("@CONTINUE_{}\n", id)
                    + "0;JMP\n"
                    + &format!("(HIT_{})\n", id) // test hit, bool=true(-1)
                    + &format!("@{TEMP_BASE}\n")
                    + "M=-1\n"
                    + &format!("(CONTINUE_{})\n", id)
                    + "// end ======= temp0 := temp1 > temp0\n"
                    + "\n"
                    + &&self._write_push("temp", 0)
                    + "// end ======= gt\n"
                    + "\n"
            }
            "lt" => {
                String::new()
                    + "// start ======= lt\n"
                    + &self._write_pop("temp", 0)
                    + &self._write_pop("temp", 1)
                    + "// start ======= temp0 := temp1 < temp0\n"
                    + &format!("@{TEMP_BASE}\n")
                    + "A=A+1\n"
                    + "D=M\n"
                    + &format!("@{TEMP_BASE}\n")
                    + "M=D-M\n" // D: x, M: y
                    + "D=M\n" // if D(x-y) < 0, M=true(-1), else M=false(0)
                    + "M=0\n" // bool=false(0)
                    + &format!("@HIT_{}\n", id)
                    + "D;JLT\n"
                    + &format!("@CONTINUE_{}\n", id)
                    + "0;JMP\n"
                    + &format!("(HIT_{})\n", id) // test hit, bool=true(-1)
                    + &format!("@{TEMP_BASE}\n")
                    + "M=-1\n"
                    + &format!("(CONTINUE_{})\n", id)
                    + "// end ======= temp0 := temp1 < temp0\n"
                    + "\n"
                    + &&self._write_push("temp", 0)
                    + "// end ======= lt\n"
                    + "\n"
            }
            "and" => {
                String::new()
                    + "// start ======= and\n"
                    + &self._write_pop("temp", 0)
                    + &self._write_pop("temp", 1)
                    + "// start ======= temp0 = temp1 & temp0\n"
                    + &format!("@{TEMP_BASE}\n")
                    + "A=A+1\n"
                    + "D=M\n"
                    + &format!("@{TEMP_BASE}\n")
                    + "M=D&M\n" // D: x, M: y
                    + "// end ======= temp0 = temp1 & temp0\n"
                    + "\n"
                    + &&self._write_push("temp", 0)
                    + "// end ======= and\n"
                    + "\n"
            }
            "or" => {
                String::new()
                    + "// start ======= or\n"
                    + &self._write_pop("temp", 0)
                    + &self._write_pop("temp", 1)
                    + "// start ======= temp0 = temp1 | temp0\n"
                    + &format!("@{TEMP_BASE}\n")
                    + "A=A+1\n"
                    + "D=M\n"
                    + &format!("@{TEMP_BASE}\n")
                    + "M=D|M\n" // D: x, M: y
                    + "// end ======= temp0 = temp1 | temp0\n"
                    + "\n"
                    + &&self._write_push("temp", 0)
                    + "// end ======= or\n"
                    + "\n"
            }
            "not" => {
                String::new()
                    + "// start ======= not\n"
                    + &self._write_pop("temp", 0)
                    + "// start ======= temp0 = !temp0\n"
                    + &format!("@{TEMP_BASE}\n")
                    + "M=!M\n"
                    + "// end ======= temp0 = !temp0\n"
                    + "\n"
                    + &&self._write_push("temp", 0)
                    + "// end ======= not\n"
                    + "\n"
            }
            cmd => panic!("arithmetic command syntax error: unknow command `{cmd}`"),
        }
    }

    pub fn write_arithmetic(&mut self, cmd: &str, id: &str) -> io::Result<()> {
        let buf = self._write_arithmetic(cmd, id);
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
            "temp" => {
                let ram_address = arg2 + 5;
                String::new()
                    + &format!("// start ======== push {arg1} {arg2}\n")
                    + &format!("// D={arg1}+{arg2}\n")
                    + &format!("@{ram_address}\n")
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
            "static" => {
                let static_var_id = format!("{}.{arg2}", self.filename);
                String::new()
                    + &format!("// start ======== push {arg1} {arg2}\n")
                    + &format!("// D={arg1}+{arg2}\n")
                    + &format!("@{static_var_id}\n")
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
            "pointer" => {
                let pointer_address = if arg2 == 0 { 3 } else { 4 };
                String::new()
                    + &format!("// start ======== push {arg1} {arg2}\n")
                    + &format!("// D= value of {arg1} {arg2}\n")
                    + &format!("@{pointer_address}\n")
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
            _ => {
                assert!(self.mem_seg_map.contains_key(arg1));
                let arg1 = self.mem_seg_map[arg1].clone();
                String::new()
                    + &format!("// start ======== push {arg1} {arg2}\n")
                    + &format!("// D={arg1}+{arg2}\n")
                    + &format!("@{arg1}\n")
                    + &"D=M\n"
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
            "temp" => {
                let ram_address = arg2 + 5;
                String::new()
                    + &format!("// start ======== pop {arg1} {arg2}\n")
                    + &format!("// SP--\n")
                    + &"@SP\n"
                    + &"M=M-1\n"
                    + &format!("// R13=addr({arg1}+{arg2})\n")
                    + &format!("@{ram_address}\n")
                    + &"D=A\n" // temp hold the register itself, not address
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
            "static" => {
                let static_var_id = format!("{}.{arg2}", self.filename);
                String::new()
                    + &format!("// start ======== pop {arg1} {arg2}\n")
                    + &format!("// SP--\n")
                    + &"@SP\n"
                    + &"M=M-1\n"
                    + &format!("// D=stack[SP]\n")
                    + &"@SP\n"
                    + &"A=M\n"
                    + &"D=M\n"
                    + &format!("// *{{filename}}.i=D\n")
                    + &format!("@{static_var_id}\n")
                    + &"M=D\n"
                    + &format!("// end ======== pop {arg1} {arg2}\n")
                    + &"\n"
            }
            "pointer" => {
                let pointer_address = if arg2 == 0 { 3 } else { 4 };
                String::new()
                    + &format!("// start ======== pop {arg1} {arg2}\n")
                    + &format!("// SP--\n")
                    + &"@SP\n"
                    + &"M=M-1\n"
                    + &format!("// D=stack[SP]\n")
                    + &"@SP\n"
                    + &"A=M\n"
                    + &"D=M\n"
                    + &format!("// *({arg1} {arg2})=D\n")
                    + &format!("@{pointer_address}\n")
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
                    + &"D=M\n"
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
        code_writer.write_arithmetic("add", "1")?;
        code_writer.close()?;
        fs::remove_file(file_path)?;
        Ok(())
    }
}
