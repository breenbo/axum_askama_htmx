use anyhow::{Context, Error, Result};
use askama::Template;
use axum::{
    Form, Json, Router,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
};
use serde::{Deserialize, Serialize};

// import html template with askama
// send the title to the template
// props are passed to askama template with struct
#[derive(Template)]
#[template(path = "root.html")]
struct RootTemplate<'a> {
    title: &'a str,
    some_data: &'a str,
}

async fn root() -> impl IntoResponse {
    tracing::info!("Get / root");
    let root = RootTemplate {
        title: "Some title",
        some_data: "Some data to display",
    };

    match root.render() {
        Ok(val) => (StatusCode::OK, Html(val)),
        Err(_) => {
            tracing::error!("Root component unavailable");
            (
                StatusCode::NO_CONTENT,
                Html("Content unavailable".to_string()),
            )
        }
    }
}

#[derive(Template)]
#[template(path = "ideas.html")]
struct IdeasTemplate<'a> {
    title: &'a str,
    ideas: Vec<&'a str>,
}

async fn ideas() -> impl IntoResponse {
    tracing::info!("Get /home");
    let home = IdeasTemplate {
        title: "Ideas",
        ideas: Vec::new(),
    };

    match home.render() {
        Ok(val) => (StatusCode::OK, Html(val)),
        Err(_) => {
            tracing::error!("Home component unavailable");
            (
                StatusCode::NO_CONTENT,
                Html("Content unavailable".to_string()),
            )
        }
    }
}

#[derive(Deserialize, Serialize)]
struct IdeaForm {
    idea: String,
}

// Form() allow to decode strings sent from post request
async fn create_idea(Form(form): Form<IdeaForm>) -> (StatusCode, String) {
    tracing::info!("{}", format!("Post /ideas : {}", form.idea));

    (StatusCode::OK, form.idea)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    const PORT: &str = "8080";
    tracing_subscriber::fmt::init(); // display in console

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{PORT}"))
        .await
        .with_context(|| format!("Failed to bind to port {PORT}"))?;

    tracing::info!("Router initialized, now listening port {PORT}");

    let app = Router::new()
        .route("/", get(root))
        .route("/ideas", get(ideas).post(create_idea));

    axum::serve(listener, app)
        .await
        .context("Failed to start the server")?;

    Ok(())
}
