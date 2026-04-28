use poem::{Error, Route, Server, get, handler, listener::TcpListener, post, web::{Json, Path}};

use crate::{request_input::CreateWebsiteRequest, request_output::CreateWebsiteOutput};
pub mod request_input;
pub mod request_output;



#[handler]
fn get_website(Path(website_id): Path<String>) -> String {
    format!("Website ID: {}", website_id)
}
#[handler]
fn create_website(Json(data): Json<CreateWebsiteRequest>) -> Json<CreateWebsiteOutput> {
    let url = data.website_id;
    // persist this in db

    let response: CreateWebsiteOutput = CreateWebsiteOutput { 
        id: url 
    };

    Json(response)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/website/:website_id", get(get_website))
        .at("/website", post(create_website));
    Server::new(TcpListener::bind("0.0.0.0:3001"))
      .run(app)
      .await
}