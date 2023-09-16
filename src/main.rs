mod code_writer;
mod parser;
mod test_file;

use code_writer::CodeWriter;
use parser::*;
use std::{env::args, error::Error, ffi::OsString, fs, path::Path, result};

fn main() -> result::Result<(), Box<dyn Error>> {
    assert_eq!(
        args().len(),
        2,
        "VM Translator need a input file or folder arg"
    );

    let input_arg = args().nth(1).unwrap();
    let input_path = Path::new(&input_arg);

    if input_path.is_file() {
        let input_file_name = input_path.file_name().unwrap();
        let input_file_dir = input_path.parent().unwrap();

        let input_file_name_str = input_file_name.to_str().unwrap();
        let output_file_name = OsString::from(input_file_name_str.replace(".vm", ".asm"));
        let output_file_path = input_file_dir.join(output_file_name);

        let mut code_writer = CodeWriter::new(&output_file_path)?;

        translate_file(&mut code_writer, input_path)?;

        code_writer.close()?;
    } else if input_path.is_dir() {
        let output_file_name = input_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            + ".asm";
        let output_file_path = input_path.join(output_file_name);

        let mut code_writer = CodeWriter::new(&output_file_path)?;

        for entry in fs::read_dir(input_path)? {
            let entry = entry?;

            if entry.path().is_file() && entry.path().extension().unwrap() == "vm" {
                translate_file(&mut code_writer, &entry.path())?;
            }
        }
        code_writer.close()?;
    }

    Ok(())
}

fn translate_file(
    code_writer: &mut CodeWriter,
    file_path: &Path,
) -> result::Result<(), Box<dyn Error>> {
    let mut parser = Parser::new(file_path)?;
    let source_file_name = file_path.file_name().unwrap().to_str().unwrap();
    code_writer.set_source_file(source_file_name);

    loop {
        if !parser.has_more_lines() {
        break;
        }
        parser.advance();
        if let Some(cmd) = &parser.get_cmd_type() {
            use CommandType::*;
            match cmd {
                Push | Pop => {
                    code_writer.write_push_pop(&parser.cmd(), &parser.arg1(), parser.arg2())?
                }
                Arithmetic => code_writer.write_arithmetic(
                    &parser.cmd(),
                    &format!("{source_file_name}.{}", parser.next_cmd_number - 1),
                )?,
                Label => code_writer.write_label(&parser.arg1())?,
                Goto => code_writer.write_goto(&parser.arg1())?,
                If => code_writer.write_if(&parser.arg1())?,
                Function => code_writer.write_function(&parser.arg1(), parser.arg2() as u32)?,
                Call => code_writer.write_call(&parser.arg1(), parser.arg2() as u32)?,
                Return => code_writer.write_return()?,
            }
        }
    }

    Ok(())
}
