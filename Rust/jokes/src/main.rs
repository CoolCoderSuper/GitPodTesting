use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    println!("Jokes App");
    loop {
        let mut command = String::new();
        match std::io::stdin().read_line(&mut command) {
            Ok(_s) => {
                command = command.trim().to_string();
                if command == "exit"{
                    break;
                }else{                
                    let url = if !command.trim().is_empty() {
                        format!("https://api.chucknorris.io/jokes/random?category={command}")
                    }else{
                        "https://api.chucknorris.io/jokes/random".to_string()
                    };
                    match reqwest::get(url).await {
                        Ok(res) => match res.json::<Joke>().await {
                            Ok(j) => println!("{}", j.value),
                            Err(e) => println!("{e}")
                        },
                        Err(e) => println!("{e}")
                    }
                }
            }
            Err(e) => println!("{e}")
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Joke {
    value: String
}