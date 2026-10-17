use anyhow::{Context, Error, Result};
use askama::Template;
use axum::{
    Form, Router, debug_handler,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tower_sessions::{MemoryStore, Session, SessionManagerLayer};

// Error managment
#[derive(Error, Debug)]
enum AppError {
    #[error("Failed to render template")]
    Template(#[from] askama::Error),

    #[error("Session Error")]
    Session(#[from] tower_sessions::session::Error),
}

// transform AppError to http response for Axum
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("{self}");

        let (status, error_message) = match self {
            AppError::Template(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error while creating the page",
            ),
            AppError::Session(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error session management",
            ),
        };

        (
            status,
            Html(format!("<h1>Error</h1><p>{error_message}</p>")),
        )
            .into_response() // convertit le tuple en Response attendu par Axum
    }
}

// import html template with askama
// send the title to the template
// props are passed to askama template with struct
#[derive(Template)]
#[template(path = "root.html")]
struct RootTemplate<'a> {
    title: &'a str,
    some_data: &'a str,
}

async fn root() -> Result<impl IntoResponse, AppError> {
    tracing::info!("Get / root");
    let root = RootTemplate {
        title: "Some title",
        some_data: "Some data to display",
    };

    let rendered = root.render()?;

    Ok((StatusCode::OK, Html(rendered)))
}

#[derive(Template)]
#[template(path = "ideas.html")]
struct IdeasTemplate {
    title: String,
    ideas: Vec<String>,
}

async fn ideas(session: Session) -> Result<impl IntoResponse, AppError> {
    tracing::info!("Get /home");

    // last unwrap to manage the Option inside the Result
    // Error in Result managed by ?
    let ids: Vec<String> = session.get("ideas").await?.unwrap_or_default();

    let ideas = IdeasTemplate {
        title: "Ideas".to_string(),
        ideas: ids,
    };

    let rendered = ideas.render()?;

    Ok((StatusCode::OK, Html(rendered)))
}

#[derive(Deserialize, Serialize)]
struct IdeaForm {
    idea: String,
}

// Form() allow to decode strings sent from post request
#[debug_handler]
async fn create_idea(session: Session, Form(form): Form<IdeaForm>) -> Result<Redirect, AppError> {
    tracing::info!("{}", format!("Post /ideas : {}", form.idea));

    let mut ids: Vec<String> = session.get("ideas").await?.unwrap_or_default();

    ids.push(form.idea);

    println!("ids: {:?}", ids);

    // Store back
    session.insert("ideas", ids).await?;

    Ok(Redirect::to("/ideas"))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    const PORT: &str = "8080";
    tracing_subscriber::fmt::init(); // display in console

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{PORT}"))
        .await
        .with_context(|| format!("Failed to bind to port {PORT}"))?;

    tracing::info!("Router initialized, now listening port {PORT}");

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store).with_secure(false);

    let app = Router::new()
        .route("/", get(root))
        .route("/ideas", get(ideas).post(create_idea).layer(session_layer));

    axum::serve(listener, app)
        .await
        .context("Failed to start the server")?;

    Ok(())
}
