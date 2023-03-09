#[macro_use] extern crate rocket;

use rocket::form::Form;
use rocket::response::content;
use rocket_dyn_templates::{Template, context};
use rand::{Rng, thread_rng};


#[derive(FromForm)]
struct InputWord {
    #[field(default = "hello")]
    word: String,
}


#[post("/generate", data = "<input>")]
fn generate(input: Form<InputWord>) -> content::RawHtml<String> {
    let slug: String = input.word.clone();
    let mut rng = thread_rng();
    let mut generated_text = String::with_capacity(128);
    loop {
        let c = u8::from(rng.gen_range(97..=122)) as char;
        generated_text.push(c);
        if generated_text.len() > slug.len(){
            if generated_text[generated_text.len()-slug.len()..generated_text.len()] == slug { break; }
        }
    }
    
    let chars_pushed = generated_text.len()-slug.len();
    if generated_text.len()>300{
        generated_text = generated_text[generated_text.len()-300..].to_string();
    }

    content::RawHtml(format!("
    <div class=\"p-4 grow bg-gray-800/70 md:w-3/6 md:rounded\">
    <h1 class=\"text-3xl\">
    Results
    </h1>
    <p>Monkey has written {} characters in search of the word '{}'! </p>
    </div>
    <div class=\"p-4 grow bg-gray-800/70 md:w-3/6 md:rounded\">
    Monkey's prompt (last 300 characters):<br/>
    <code class=\"rounded p-1 bg-gray-700 w-fit break-words\">
    {}
    </code>
    </div>
    ",chars_pushed, slug, generated_text))
}


#[get("/")]
fn index() -> Template {
    Template::render("index", context!{foo:123})
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![generate, index])
    .attach(Template::fairing())
}