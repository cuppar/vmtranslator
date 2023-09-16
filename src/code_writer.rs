use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{self, Write},
    path::Path,
};

pub struct CodeWriter {
    file: File,
    target_filename: String,
    source_filename: Option<String>,
    current_function: Option<String>,
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

        let target_filename = path
            .file_name()
            .ok_or(io::Error::new(io::ErrorKind::NotFound, "no filename"))?
            .to_str()
            .unwrap()
            .to_string();

        Ok(Self {
            file,
            target_filename,
            source_filename: None,
            current_function: None,
            mem_seg_map,
        })
    }

    pub fn set_source_file(&mut self, source_file: &str) {
        self.source_filename = Some(source_file.to_string());
    }

    fn set_current_function(&mut self, current_function: &str) {
        self.current_function = Some(current_function.to_string());
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
                assert!(self.source_filename.is_some());
                let static_var_id = format!("{}.{arg2}", self.source_filename.clone().unwrap());
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
                assert!(self.source_filename.is_some());
                let static_var_id = format!("{}.{arg2}", self.source_filename.clone().unwrap());
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

    fn _gen_label(&self, label: &str) -> String {
        let mut file = "".to_string();
        if let Some(f) = self.source_filename.clone() {
            file = f;
        };
        let mut function = "".to_string();
        if let Some(fun) = self.current_function.clone() {
            function = fun;
        };

        format!("{file}.{function}${label}")
    }

    fn _write_label(&mut self, label: &str) -> String {
        format!("({})\n", self._gen_label(label))
    }

    pub fn write_label(&mut self, label: &str) -> io::Result<()> {
        let buf = self._write_label(label);
        self.file.write_all(buf.as_bytes())?;
        Ok(())
    }

    fn _write_goto(&mut self, label: &str) -> String {
        format!("@{}\n", self._gen_label(label)) + "0;JMP\n"
    }

    pub fn write_goto(&mut self, label: &str) -> io::Result<()> {
        let buf = self._write_goto(label);
        self.file.write_all(buf.as_bytes())?;
        Ok(())
    }

    fn _write_if(&mut self, label: &str) -> String {
        String::new()
            + "@SP\n"
            + "AM=M-1\n"
            + "D=M\n"
            + &format!("@{}\n", self._gen_label(label))
            + "D;JNE\n"
    }

    pub fn write_if(&mut self, label: &str) -> io::Result<()> {
        let buf = self._write_if(label);
        self.file.write_all(buf.as_bytes())?;
        Ok(())
    }

    fn _write_function(&mut self, function_name: &str, n_vars: u32) -> String {
        "".to_string()
    }

    pub fn write_function(&mut self, function_name: &str, n_vars: u32) -> io::Result<()> {
        let buf = self._write_function(function_name, n_vars);
        self.file.write_all(buf.as_bytes())?;
        Ok(())
    }

    fn _write_call(&mut self, function_name: &str, n_args: u32) -> String {
        "".to_string()
    }

    pub fn write_call(&mut self, function_name: &str, n_args: u32) -> io::Result<()> {
        let buf = self._write_call(function_name, n_args);
        self.file.write_all(buf.as_bytes())?;
        Ok(())
    }

    fn _write_return(&mut self) -> String {
        "".to_string()
    }

    pub fn write_return(&mut self) -> io::Result<()> {
        let buf = self._write_return();
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
