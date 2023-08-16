use std::env;
use regex::Regex;
use serde::Deserialize;
use dotenv::dotenv;

#[derive(Deserialize,Debug)]
struct HuggingFaceResponse{
  generated_text : String,
}

async fn gpt(args: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
  
  let hugging_face_key = env::var("KEY").expect("HuggingFace Key Not Set");
  let gpt_model = env::var("MODEL").expect("GPT Module Not Set");

  let mut payload = String::new();
  
  for count in 3..args.len() {
      payload += &(" ".to_owned() + &args[count]);
  }
  
  //println!("{}",payload);
  let header = "Bearer ".to_owned() + &hugging_face_key;
  let client = reqwest::Client::new();
  let response = client
      .post(gpt_model)
      .header("Authorization", &header)
      .body(payload)
      .send()
      .await?; 

  
  let hugging_face_response: HuggingFaceResponse = response.json().await?;
  
  println!("\n Model: {}", hugging_face_response.generated_text);

  Ok(())
}

pub async fn start(){
  dotenv().ok();
  fillter_args().await;
}

async fn fillter_args(){
  let args:Vec<String> = env::args().collect();

  if chk_agrs(&args) == -1 || args.len() < 3{
    println!("Arrugment Formation Is Worng... Use -h For Help");
    return;
  }
    //print!("\n Arrgument : {} ",args[2]);

    if args[2] == "-h" || args[2] == "--help" {
      println!("AssistX CLI Tool \n");
      println!("Usage: AssistX [OPTIONS] [PAYLOAD|INPUT]\n");
      println!("Example: AssistX --img-txt -p 'Man Standing One The Moon'\n");
      println!("Options:");
      println!("  -h,--help        Help");
      println!("  -?,--gpt         Ask GPT Model");
      println!("  --version        Check Version");
      println!("  --update         Check For Update");
      println!("  --setup          Setup Assistx");
      println!("  --uninstall      Uninstall The Assist-Tool");
      println!("  --about          About The Assistx-Tool");
      
      return;
    }
    if args[2] == "-h" || args[2] == "--help" {
      println!("AssistX CLI Tool \n");
      println!("Usage: AssistX [OPTIONS] [PAYLOAD|INPUT]\n");
      println!("Example: AssistX --img-txt -p 'Man Standing One The Moon'\n");
      println!("Options:");
      println!("  -h,--help        Help");
      println!("  -?,--gpt         Ask GPT Model");
    
      return;
    }

    if args[2] == "-?" || args[2] == "--gpt"{
      let _ = gpt(&args).await;
    }
  
    else if args[2] == "--version"{println!("\n AssistX CLI Version 1.0");}
    else if args[2] == "--update"{println!("\n No Update Available");}
    else if args[2] == "--setup"{
      println!("\n Setup : You Can Find A .env File In The Folder You Have Installed The CLI-Tool Open It , Then Past The API Key In The KEY field and Model api in Model \n Field Then Save And Exit , Know Add The Assist.exe or assistx.rs With The Full Path Of The File \n To The Enviroment Variable You Can Find In Youtube How To Add Enviroment Variable , After That You Can Open \n The cmd And Excute Assistx --version to check if it works");
    }
    else if args[2] == "--uninstall"{println!("\n Uninstall The Eniter Folder Assistx-CLI From The Folder You Installed It \n Note: You Need To Remove The assist.exe or assist.rs From The Enviroment Variables To Uninstall It Fully...");}
    else if args[2] == "--about"{
      println!("\n AssistX CLI Version 1.0");
      println!("\n Assistx Tool Helps To Connect To LLM Via Throught CLI , Makes Work Fast And Easy And Streamless ");
    }

    }

fn chk_agrs(args: &Vec<String>) -> i32 {

  let options_patter_combined = r"^-h$|^--help$|^--txt-img$|^--img-class$|^--img-txt$||^--gpt$|^-?$";
  
  let regex = Regex::new(&options_patter_combined).unwrap();
  let options_test = regex.is_match(&args[2]);


  if options_test == false{ 
    return -1;
  }
  return 0;
}
