mod code_writer;
mod parser;
mod test_file;

use code_writer::CodeWriter;
use parser::*;
use std::{env::args, error::Error, ffi::OsString, path::Path, result};

fn main() -> result::Result<(), Box<dyn Error>> {
    assert_eq!(args().len(), 2, "VM Translator need a input file arg");
    let input_file_arg = args().nth(1).unwrap();
    let input_file_path = Path::new(&input_file_arg);
    assert!(input_file_path.is_file());
    let input_file_name = input_file_path.file_name().unwrap();
    let input_file_dir = input_file_path.parent().unwrap();

    let input_file_name_str = input_file_name.to_str().unwrap();
    let output_file_name = OsString::from(input_file_name_str.replace(".vm", ".asm"));
    let output_file_path = input_file_dir.join(output_file_name);

    let mut code_writer = CodeWriter::new(&output_file_path)?;

    let mut parser = Parser::new(input_file_path)?;
    loop {
        if !parser.has_more_lines() {
            code_writer.close()?;
            break;
        }
        parser.advance();
        if let Some(cmd) = &parser.get_cmd_type() {
            use CommandType::*;
            match cmd {
                Push | Pop => {
                    code_writer.write_push_pop(&parser.cmd(), &parser.arg1(), parser.arg2())?
                }
                Arithmetic => code_writer
                    .write_arithmetic(&parser.cmd(), &(parser.next_cmd_number - 1).to_string())?,
                _ => (),
            }
        }
    }

    Ok(())
}
