use std::{path::Path, sync::{Mutex, Arc}, env};

use actix_web::{get, put, web, App, HttpResponse, HttpServer, Responder, http::header::ContentType};
use obs_services::counter::Counter;

struct AppState {
    pub counter: Arc<Mutex<Counter>>,
}

#[get("/ui")]
async fn ui() -> impl Responder {
    let body = String::from("
    <html>
    <head>
    <style>
    h1 {
        font-family: Impact;
        font-size: 100px;
        -webkit-text-fill-color: #f0f0f0;
        -webkit-text-stroke: 1px;
    }
    </style>
    </head>
    <body>
        <h1></h1>
        <script>
            const updateCounter = () => {
                const h = document.getElementsByTagName('h1')[0];
                fetch('/val')
                    .then(res => res.text())
                    .then(count => {
                        h.textContent = count;
                        setTimeout(() => updateCounter(), 500);
                    })
                    .catch(console.error);
            };
            updateCounter();
        </script>
    </body>
</html>
");
    HttpResponse::Ok().insert_header(ContentType::html()).body(body)
}

#[get("/val")]
async fn value(data: web::Data<AppState>) -> String {
    let counter = data.counter.lock().unwrap();
    format!("{}", counter.value().to_string())
}

#[put("/inc")]
async fn increment(data: web::Data<AppState>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    match counter.increment() {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[put("/res")]
async fn reset(data: web::Data<AppState>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    match counter.reset() {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let counter_path = match args.len() > 1 {
        true => Path::new(&args[1]),
        false => Path::new("foo.txt"),
    };
    match Counter::from_file(counter_path) {
        Ok(counter) => {
            let rc_counter = Arc::new(Mutex::new(counter));
            HttpServer::new(move || {
                App::new()
                    .app_data(web::Data::new(AppState {
                        counter: Arc::clone(&rc_counter),
                    }))
                    .service(value)
                    .service(increment)
                    .service(reset)
                    .service(ui)
            })
            .bind(("0.0.0.0", 8080))?
            .run()
            .await
        },
        Err(err) => Err(err),
    }
}
