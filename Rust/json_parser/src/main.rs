mod token;
mod lexer;

use token::*;
use lexer::*;

fn main() {
    let json = r#"
    [{
,:true,false,null/*dfvfdvdvfvfddfvdfv */,23432423432.324,-22332E323232,"sdcdsdcsdcs",fiudsibdsibibusiubvsdw3rg7837r238wefuyewf,'dfdfvdvfdfvvfd'//fddfvdfvvdfdvfv
   /*ee */ }]//sdcdsdcsdcs
 //dscdcsdcsdsc   
 {}/*fe f   ewfe    f   e   fewf    e   efwf
 tyhytehyetyht
 hyteehtytyteythe"#;
    let mut lexer = Lexer::new(json);    
    loop {
        let token = lexer.lex();
        println!("{:?}", token);
        if token.kind == SyntaxKind::EndOfFileToken {
            break;
        }
    }
}