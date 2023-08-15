use std::env;
use regex::Regex;
use serde::Deserialize;
use dotenv::dotenv;


#[derive(Deserialize,Debug)]
struct HuggingFaceResponse{
  generated_text : String,
}

async fn gpt(args: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
  
  let hugging_face_key = env::var("HUGGINGFACE_ACCESS_TOKEN").expect("HuggingFace Key Not Set");
  let gpt_model = env::var("GPT_MODEL").expect("GPT Module Not Set");

  let mut payload = String::new();
  
  for count in 3..args.len() {
      payload += &(" ".to_owned() + &args[count]);
  }
  
  println!("{}",payload);
  let header = "Bearer ".to_owned() + &hugging_face_key;
  let client = reqwest::Client::new();
  let response = client
      .post(gpt_model)
      .header("Authorization", &header)
      .body(payload)
      .send()
      .await?; 

  
  let hugging_face_response: HuggingFaceResponse = response.json().await?;
  
  println!("\n Generated Text: {}", hugging_face_response.generated_text);

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
  for arg in &args{
    //print!("\n Arrgument : {} ",arg);

    if arg == "-h" || arg == "--help" {
      println!("AssistX CLI Tool \n");
      println!("Usage: AssistX [OPTIONS] [PAYLOAD|INPUT]\n");
      println!("Example: AssistX --img-txt -p 'Man Standing One The Moon'\n");
      println!("Options:");
      println!("  -h,--help        Help");
      println!("  -p,--payload     Used To Denote Input");
      println!("  -?,--gpt         Ask GPT Model");
      println!("  --txt-img        Text To Image");
      println!("  --img-class      Image Classification");
      println!("  --img-txt        Image To Text\n\n");
      println!("  --text-voice     Text To Voice\n\n");

      return;
    }
    if arg == "-?" || arg == "--gpt"{
      //print!("Entered GPT");
      let _ = gpt(&args).await;
    }
    else if arg == "--txt-img"{
      txt_img(&args);
    }
    else if arg == "--img-txt"{
      img_txt(&args)
    }
    else if arg == "--img-class"{
      img_class(&args);
    }else if arg == "--version"{
      println!("\n AssistX CLI Version 1.0")
    }else if arg == "--update"{
      println!("Checking For Updates...")
    }else if arg == "--setup"{
      println!("");
    }else if arg == "--uninstall"{
      println!("Uninstalling The Assistx...");
    }else if arg == "--about"{
      println!("\n AssistX CLI Version 1.0");
      println!("Assistx Tool Helps To Connect To LLM Via Throught CLI , Makes Work Fast And Easy And Streamless ");
    }

  }
}

fn chk_agrs(args: &Vec<String>) -> i32 {

  let options_patter_combined = r"^-h$|^--help$|^--txt-img$|^--img-class$|^--img-txt$||^--gpt$|^-?$";
  
  let regex = Regex::new(&options_patter_combined).unwrap();
  let options_test = regex.is_match(&args[2]);

  //println!("Args[2] : {}", args[2]);
  //println!("{}", options_test);

  if options_test == false{ // if the regex don't match, then the values of options_test would be false 
    return -1;
  }
  return 0;
}


async fn txt_img(args: &Vec<String>) -> Result<(), Box<dyn std::error::Error>>{
  let api_url = "https://api-inference.huggingface.co/models/microsoft/DialoGPT-medium";
  
  let payload = &args[2];

  let client = reqwest::Client::new();
  let response = client
      .post(api_url)
      .header("Authorization", "Bearer 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")
      .body(payload.to_owned())
      .send()
      .await?; 

  
  let hugging_face_response: HuggingFaceResponse = response.json().await?;
  
  println!("\n Generated Text: {}", hugging_face_response.generated_text);

  Ok(())

}
fn img_txt(args: &Vec<String>){}
fn img_class(args: &Vec<String>){}