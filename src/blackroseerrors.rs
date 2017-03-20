use std::{process, collections};
let mut get_err_sig = collections::HashMap::new();
get_err_sig.insert("NoError", (0,1));
get_err_sig.insert("ArgumentError", (1,2));
get_err_sig.insert("InvalidCharacterError", (1,3));
get_err_sig.insert("WriteOutError", (1,4));

pub struct Error<'a> {
    pub error_type: &'a str,
    pub linenum: usize,
    pub loc: usize,
    pub line: &'a str,
    pub filename: &'a str,
    pub info: &'a str,
}

pub fn execute(err: Error) {
    let mut interactive = false;
    let mut finalvec = vec![];
    if err.filename == "" {
        interactive = true;
    }
    if err.linenum == 0 {
        finalvec.push("\n".to_string()+err.error_type+" at command prompt:\n")
    } else if err.loc == 0 {
        let s = err.linenum.to_string();
        let ss: &str = &s;
        finalvec.push(err.error_type.to_string()+" at line "+ss+" in file "+err.filename+":")
    } else {
        let mut s = err.linenum.to_string();
        let l = err.loc.to_string();
        let ll :&str = &l;
        s.push(':');
        s = s+ ll;
        if interactive == true {
            finalvec.push(err.error_type.to_string()+" at "+&s+" during interactive session:");
        } else {
            finalvec.push(err.error_type.to_string()+" at "+&s+" in file "+err.filename+":");
        }
    }
    {
        let templine = "  ".to_string()+err.line;
        let t: &str = &templine;
        finalvec.push(t.to_string());
    }
    if err.linenum == 0 {
        let mut templine = "  ".to_string();
        templine = templine + ("-".repeat(err.line.len())).as_str()+"\n";
        finalvec.push(templine)
    } else {
        let mut templine = "  ".to_string();
        templine = templine + ("-".repeat(err.loc-1)).as_str()+"^\n";
        finalvec.push(templine);
    }
    finalvec.push(err.info.to_string());
    println!("{}", finalvec.join("\n"));
    if interactive == false {
        process::exit(get_err_sig(err.error_type).0);
    } else {
        println!("Error Signature: {}", get_err_sig(err.error_type).1);
    }
}
