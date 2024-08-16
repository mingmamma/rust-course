#![allow(unused)]

use std::{env, fs};
use std::fmt::Display;
use std::io;
use std::path::PathBuf;
use std::fmt::Formatter;

use syn::token::For;
use syn::parse_file;

fn main() {
    fn main() {
        if let Err(err_string) = try_main() {
            // write to stderr
            // proc exit
        }
    }
}

fn try_main() -> Result<(), DumpErr> {
    let mut exec_args = env::args_os();
    // skip the first arg of executable name. Apart from that, only one
    // arg containing the source file path is expected as execution argument
    let _ = exec_args.next();
    let file_to_dump_path: PathBuf = match (exec_args.next(), exec_args.next()) {
        (Some(exec_arg_os_str), None) => {
            exec_arg_os_str.into()
        }
        (_, _) => { panic!("Unexpected arguement for this program") }
    };

    // read source code at the given file path into a String.
    // parse that as Rust code with syn::parse_file
    // retain relevant info for reporting in case of parse error
    let source_code_of_dump_file = fs::read_to_string(&file_to_dump_path).expect("failed to read desinated file");
    let parsed_file = parse_file(&source_code_of_dump_file)
        .map_err(|syn_parse_err| {
            DumpErr::ParseFileErr {
                syn_err: syn_parse_err,
                dump_file_path: file_to_dump_path,
                src_code_string: source_code_of_dump_file
            }
        })?;

    Ok(())
}

enum DumpErr {
    ParseFileErr {
        syn_err: syn::Error,
        dump_file_path: PathBuf,
        src_code_string: String,
    },
}

impl Display for DumpErr {
    fn fmt(&self, fmter: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        todo!()
        // in case of ParseFileError, an implementation is given to
        // Error::ParseFileErr {
        //      syn_err,
        //      dump_file_path,
        //      src_code_string,
        // } => display_par_loc_impl(fmter, syn_err, dump_file_path, src_code_string)
    }
}

fn display_par_loc_impl(
    fmter: &mut Formatter<'_>, 
    syn_err: &syn::Error, 
    dump_file_path: &PathBuf, 
    src_code_string: &str) -> () 
{
    // todo!()
    let start = syn_err.span().start();
    let mut end = syn_err.span().end();

    let code_line = match start.line.checked_sub(1).and_then(|zero_idx_start_line| src_code_string.lines().nth(zero_idx_start_line)) {
        Some(line) => line,
        None => "wtf"
    };

    if end.line > start.line {
        end.line = start.line;
        end.column = code_line.len();
    }

    let underline = "^".repeat(end.column.saturating_sub(start.column).max(1));
}
