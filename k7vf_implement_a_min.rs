use actix_web::{web, App, HttpServer, Responder};
use notify::{Watcher, Debouncer};
use std::path::Path;
use std::time::Duration;

async fn notify(request: web::HttpRequest) -> impl Responder {
    let dir_path = "/path/to/watch/directory";
    let mut watcher = Watcher::new(move |res| match res {
        Ok(events) => {
            for event in events {
                let file_path = event.paths.get(0).unwrap();
                let file_name = file_path.file_name().unwrap().to_str().unwrap();
                println!("File {} has been modified", file_name);
            }
        }
        Err(e) => println!("watch error: {:?}", e),
    })
    .unwrap();
    watcher.watch(Path::new(dir_path), move |res| match res {
        Ok(event) => {
            println!("change: {:?}", event);
        }
        Err(e) => println!("error: {:?}", e),
    })
    .unwrap();
    "Notifier is running..."
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::resource("/notify").route(web::get().to(notify))))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}