use serde::{Deserialize, Serialize};

// for create website route
#[derive(Serialize, Deserialize)]
pub struct CreateWebsiteRequest{
   pub website_id: String,
}