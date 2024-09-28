use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use regex::Regex;

struct RegexPatterns{
    h1:regex::Regex,
    h2:regex::Regex ,
    h3:regex::Regex,
    h4:regex::Regex,
    h5:regex::Regex,
    h6:regex::Regex,

    italic:regex::Regex,
    bold:regex::Regex,
    bold_italic:regex::Regex,
    highlight1:regex::Regex ,
    highlight2:regex::Regex,
    highlight3:regex::Regex,
    link:regex::Regex,
    quote_block:regex::Regex,
    quote_header:regex::Regex,
}

impl RegexPatterns {
    fn new() -> RegexPatterns {
        RegexPatterns { 
 h1: Regex::new(r"^#\s*[a-zA-Z0-9]+").unwrap(),
 h2: Regex::new(r"^##\s*[a-zA-Z0-9]+").unwrap(),
 h3: Regex::new(r"^###\s*[a-zA-Z0-9]+").unwrap(),
 h4: Regex::new(r"^####\s*[a-zA-Z0-9]+").unwrap(),
 h5:Regex::new(r"^#####\s*[a-zA-Z0-9]+").unwrap(),
 h6: Regex::new(r"^######\s*[a-zA-Z0-9]+").unwrap(),
 italic: Regex::new(r"[^\*]\*[^\*]+\*[^\*]").unwrap(),
 bold: Regex::new(r"[^\*]\*\*[^\*]+\*\*[^\*]").unwrap(),
 bold_italic:Regex::new(r"[^\*]\*\*\*[^\*]+\*\*\*[^\*]").unwrap(),
 highlight1: Regex::new(r"\{[^\{]\s*[a-zA-Z0-9\s]+\s*\}").unwrap(),
 highlight2: Regex::new(r"[^\{]\{{2}[^(\{|\})]+\}{2}").unwrap(),
 highlight3: Regex::new(r"\{\{\{\s*[a-zA-Z0-9\s]+\s*\}\}\}").unwrap(),
 link:Regex::new(r"[^\[]*\[{2}[^(\[|\])]+\]{2}").unwrap(),
 quote_block:Regex::new(r"^>+.+").unwrap(),
 quote_header: Regex::new(r">\[\!+.+").unwrap(),

         }
    }
}



fn main() {
    create_file("confsport.html"); 
}


fn final_write(file_name:&str, package:String) -> std::io::Result<()> {
    let package_as_bytes = package.into_bytes();
    let mut html_file = File::create(file_name)?;
    html_file.write_all(&package_as_bytes)?;
    Ok(())
}




fn create_file(file_name_to:&str){
  let mut package = String::new();
  let file_title:&str =file_name_to.split('.').collect::<Vec<&str>>()[0]; 
  package.push_str("<!DOCTYPE html> \n
<html lang = \"fra\"> \n
    <head> \n
        <title>");
  package.push_str(file_title);
  package.push_str("</title> \n
        <style> 
        body {background-color: powderblue; padding-left: 3em;} \n
        h1  {color: blue;}  \n
        h2  {color: red;} \n
        h3 {color: yellow;} \n
        h4 {color: pink;} \n
        h5 {color: green;} \n
        h6 {color: orange;} \n
        hgh1 {color: purple;} \n
        hgh2 {color: brown;} \n
        hgh3 {color: pink;} \n
        link1 {color: cyan;} \n
        qh {color: red;} \n
        q1 {color: red;} \n
        </style> \n
        <meta charset = \"UTF-8\"> \n
        <meta http-equiv=\"Content-type\" content = \"text/html; charset=UTF-8\"> \n
    </head> \n
    <body> \n ");
   package.push_str(&parse_file("test.mynote").unwrap());
   package.push_str("<script src=\"script.js\"></script>
    </body>
</html>");
   final_write(file_name_to, package).unwrap();

}


fn parse_file(from_file:&str) -> std::io::Result<String> {

    let mut packages= vec![String::new()];
    let r = RegexPatterns::new();
    let header_reg = [r.h1, r.h2, r.h3, r.h4, r.h5, r.h6];
    let file = File::open(from_file)?; 
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let mut package = String::new();
    for line in contents.lines() {
         let mut i:usize = 1;
         packages[0] = line.to_string();
         // headers to do 
         for h_reg in &header_reg {
           if h_reg.is_match(&packages[0]){
                let mut temp_package = packages.pop().unwrap();
                temp_package = temp_package.replace('#', ""); 
                let header_layer = String::from("h") + &i.to_string();
                let mut very_temp_package = String::new();
                very_temp_package.push_str(&(String::from("<") + &header_layer + ">"));
                very_temp_package.push_str(&temp_package);
                very_temp_package.push_str(&(String::from("</") + &header_layer + ">"));
                packages.push(very_temp_package);
            };
            i += 1;
        };
         if r.italic.is_match(&packages[0]){
            let temp_package = packages.pop().unwrap();
            packages.push(add_style_simple(temp_package, '*' , "*" , "i"));
         }
        if r.bold.is_match(&packages[0]){
            let temp_package = packages.pop().unwrap();
            packages.push(add_style_simple(temp_package, '*' , "**" , "b"));
         }
        if r.bold_italic.is_match(&packages[0]){
            let temp_package = packages.pop().unwrap();
            packages.push(add_style_simple(temp_package, '*' , "***" , "em"));
         }
        if r.highlight2.is_match(&packages[0]){
            let temp_package = packages.pop().unwrap();
            packages.push(add_style_double(temp_package, ['{', '}'] , ["{{", "}}"] , "hgh2"));
         }
        if r.highlight3.is_match(&packages[0]){
            let temp_package = packages.pop().unwrap();
            packages.push(add_style_double(temp_package, ['{', '}'] , ["{{{", "}}}"] , "hgh3"));
         }
        if r.highlight1.is_match(&packages[0]){
            let temp_package = packages.pop().unwrap();
            packages.push(add_style_double(temp_package, ['{', '}'] , ["{", "}"] , "hgh1"));
         }
        if r.link.is_match(&packages[0]){
            let temp_package = packages.pop().unwrap();
            packages.push(add_style_double(temp_package, ['[', ']'] , ["[[", "]]"] , "link1"));
         }

        if r.quote_header.is_match(&packages[0]){
            let mut temp_package = packages.pop().unwrap();
            temp_package = temp_package.replace(">[!", ""); 
            temp_package = temp_package.replace(']', ""); 
            let header_layer = String::from("qh");
            let mut very_temp_package = String::new(); 
            very_temp_package.push_str(&(String::from("<") + &header_layer + ">"));
            very_temp_package.push_str(&temp_package);
            very_temp_package.push_str(&(String::from("</") + &header_layer + "> \n" ));
            packages.push(very_temp_package);

        }

        if r.quote_block.is_match(&packages[0]){ //not the right regex --> doesnt handle > or BOL/EOL
            let mut temp_package = packages.pop().unwrap();
            //temp_package = temp_package.replace(">", ""); 
            let header_layer = String::from("q1");
            let mut very_temp_package = String::new();
            very_temp_package.push_str(&(String::from("<") + &header_layer + ">"));
            very_temp_package.push_str(&temp_package);
            very_temp_package.push_str(&(String::from("</") + &header_layer + "> \n" ));
            packages.push(very_temp_package);
        }
        


        package.push_str(&packages[0]);
        package.push_str("<br></br> \n");
    }
    Ok(package)
}


fn add_style_double(line:String, style_mod_char:[char; 2], style_mod_str:[&str; 2], mod_html:&str) -> String {
    let mut temp_package = String::new();
    let line_parts:Vec<&str> = line.split(style_mod_str[0]).collect();
            for element in line_parts {
                if element.contains(style_mod_str[1]) {
                    let temp_part:Vec<&str> = element.split(style_mod_str[1]).collect();
                    let header_layer = String::from(mod_html);
                    temp_package.push_str(&(String::from("<") + &header_layer + ">"));
                    temp_package.push_str(&temp_part[0].replace([style_mod_char[0], style_mod_char[1]], ""));
                    temp_package.push_str(&(String::from("</") + &header_layer + ">" ));
                    temp_package.push_str(&temp_part[1].replace([style_mod_char[0],style_mod_char[1]], "")); 
                }
                else {
                    temp_package.push_str(&element.replace([style_mod_char[0], style_mod_char[1]], ""));
                }
            }
    return temp_package
}


fn add_style_simple(line:String, style_mod_char:char, style_mod_str:&str,mod_html:&str) -> String {
    let mut temp_package = String::new();
    let line_parts:Vec<&str> = line.split(style_mod_str).collect();
    temp_package.push_str(&line_parts[0].replace(style_mod_char, ""));
    let header_layer = String::from(mod_html);
    temp_package.push_str(&(String::from("<") + &header_layer + ">"));
    temp_package.push_str(&line_parts[1].replace(style_mod_char, ""));
    temp_package.push_str(&(String::from("</") + &header_layer + ">" ));
    temp_package.push_str(&line_parts[2].replace(style_mod_char, ""));
    return temp_package
}


