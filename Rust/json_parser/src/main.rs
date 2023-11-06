mod token;
mod lexer;
mod parser;
mod syntax;

use parser::*;

fn main() {
    /*let json = r#"
    [{
,:true,false,null/*dfvfdvdvfvfddfvdfv */,23432423432.324,-22332E323232,"sdcdsdcsdcs",fiudsibdsibibusiubvsdw3rg7837r238wefuyewf,'dfdfvdvfdfvvfd'//fddfvdfvvdfdvfv
   /*ee */ }]//sdcdsdcsdcs
 //dscdcsdcsdsc   
 {}/*fe f   ewfe    f   e   fewf    e   efwf
 tyhytehyetyht
 hyteehtytyteythe"#;*/*/
    let json = r#"[23432,,,,,,,423,4,23,423,4,32,423,"35345435","32423",true,false,null,[233423223,3243432,243234,423423],{test:2332,,,,,"eee":null},{}]"#;
    let mut parser = Parser::new(json);    
    match parser.parse() {
        Ok(val) => println!("{:?}", val),
        Err(e) => println!("{e}")
    }
}