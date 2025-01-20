use axum::http::StatusCode;
use axum::response::Html;
use minijinja::{context, Environment};
pub async fn handler_home() -> Result<Html<String>, StatusCode> {
    let mut env = Environment::new();
    env.add_template("home", include_str!("../templates/home.jinja"))
        .unwrap();
    env.add_template("layout", include_str!("../templates/layout.jinja"))
        .unwrap();

    let template = env.get_template("home").unwrap();
    let rendered = template
        .render(context! {
            title => "Home",
            welcome_text => "Hello World!",
        })
        .unwrap();

    Ok(Html(rendered))
}
