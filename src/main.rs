#[macro_use] extern crate rocket;

use rocket::http::Status;
use rocket::response::{content, status};
use rand::{Rng, thread_rng};

#[get("/<slug>")]
fn json(slug: String) -> status::Custom<content::RawJson<String>> {
    let mut rng = thread_rng();
    let mut generated_text = String::with_capacity(128);
    loop {
        let c = u8::from(rng.gen_range(97..=122)) as char;
        generated_text.push(c);
        if generated_text.len() > slug.len(){
            if generated_text[generated_text.len()-slug.len()..generated_text.len()] == slug { break; }
        }
    }
    
    // let generated_text = &generated_text[generated_text.len()-100..];
    let chars_pushed = generated_text.len()-slug.len();
    let monkey_id = rng.gen_range(0..1000);
    if generated_text.len()>100{
        generated_text = generated_text[generated_text.len()-100..].to_string();
    }
    status::Custom(Status::ImATeapot, content::RawJson(format!(" {{
        \"monkey_id\": \"{}\" 
        \"last_100_generated\": \"{}\" 
        \"chars_pushed\": \"{}\" 
    }}",  
    monkey_id, 
    generated_text, 
    chars_pushed)))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![json])
}