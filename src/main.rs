use rocket::{get, routes, fs::{NamedFile, FileServer}};
#[get("/<path>")]
async fn index(path: &str)->Option<NamedFile>{
    if path.is_empty(){
        NamedFile::open("index.html").await.ok()
    } else {
        match path {
            "matex" => NamedFile::open("matex.html").await.ok(), 
            "tex-rs" => NamedFile::open("tex-rs.html").await.ok(),
            "texcreate" => NamedFile::open("texcreate.html").await.ok(),
            _ => NamedFile::open("index.html").await.ok()
        }
    }
}

#[tokio::main]
async fn main(){
    rocket::build().mount("/", routes![index]).mount("/", FileServer::from(".")).launch().await.unwrap();
}