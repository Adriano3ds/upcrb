use std::{fs::{self, DirEntry}, io::Read};
use actix_web::{get, http::header::ContentType, web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "0.0.0.0")]
    host: String,
    #[arg(long, default_value_t = 8080)]
    port: u16,
}

enum PathType {
    Directory,
    File
}

fn get_path_type(path: &str) -> Result<PathType, ()> {
    if let Ok(metadata) = fs::metadata(path) {
        // Verifica se o caminho corresponde a um arquivo
        if metadata.is_file() {
            return Ok(PathType::File);
        }
        // Verifica se o caminho corresponde a um diretório
        else if metadata.is_dir() {
            return Ok(PathType::Directory);
        }
    }
    Err(())
}

#[get("/{path:.*}")]
async fn index(path: web::Path<String>) -> impl Responder {
    let path = path.into_inner();
    let current_entry = "./".to_owned() + &path;
    let current_entry = current_entry.as_str();
    return match get_path_type(current_entry) {
        Ok(PathType::Directory) => {
            let folder_entries = fs::read_dir(current_entry)
                .expect("Error reading directory")
                .map(|entry| {
                    entry
                        .expect("Error reading entry")
                })
                .collect::<Vec<DirEntry>>();

            let mut html_response = String::new();
            html_response.push_str("<!DOCTYPE html><html><head><title>Arquivos no diretório do executável</title></head><body>");
            for entry in &folder_entries {
                let name = entry.file_name().into_string().expect("Error converting file name to string");
                html_response.push_str(&format!("<a href=\"{}\">{}</a><br>", name, name));
            }
            html_response.push_str("</body></html>");

            HttpResponse::Ok().content_type(ContentType::html()).body(html_response)
        }
        Ok(PathType::File) => {
            let mut file = fs::File::open(current_entry).expect("Couldn't open file");
            let mut contents: Vec<u8> = Vec::new();
            file.read_to_end(&mut contents).expect("Could not read file");
            HttpResponse::Ok()
                .content_type(ContentType::octet_stream())
                .body(contents)
        }
        Err(_) => {
            HttpResponse::Ok().content_type(ContentType::plaintext()).body("File/directory not found")
        }
    };
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    println!("Starting server at http://{}:{}/", args.host, args.port);

    HttpServer::new(|| {
        App::new().service(index)
    })
    .bind((args.host, args.port))?
    .run()
    .await
}