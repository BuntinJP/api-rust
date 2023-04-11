use std::fs;
use std::io::{BufReader, Read};
//use serde::{Deserialize, Serialize};
use serde::Deserialize;
use std::process::Command;
extern crate serde_derive;
extern crate toml;


#[derive(Debug, Deserialize)]
struct Config {
    services: Vec<String>,
}

/* use actix_web::body::BoxBody;
use actix_web::services;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{
    get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, ResponseError,
};



use std::fmt::Display;
use std::sync::Mutex;


#[derive(Serialize, Deserialize)]
struct Ticket {
    id: u32,
    author: String,
}

impl Responder for Ticket {
    type Body = BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let res_body = serde_json::to_string(&self).unwrap();
        // Create HttpResponse and set Content Type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(res_body)
    }
}
// toml file struction

#[derive(Debug, Serialize)]
struct ErrNoId {
    id: u32,
    err: String,
}

// Implement ResponseError for ErrNoId
impl ResponseError for ErrNoId {
    fn status_code(&self) -> StatusCode {
        StatusCode::NOT_FOUND
    }
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let body = serde_json::to_string(&self).unwrap();
        let res = HttpResponse::new(self.status_code());
        res.set_body(BoxBody::new(body))
    }
}

// Implement Display for ErrNoId
impl Display for ErrNoId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

struct AppState {
    tickets: Mutex<Vec<Ticket>>,
}

// Create a ticket
#[post("/tickets")]
async fn post_ticket(req: web::Json<Ticket>, data: web::Data<AppState>) -> impl Responder {
    let new_ticket = Ticket {
        id: req.id,
        author: String::from(&req.author),
    };
    let mut tickets = data.tickets.lock().unwrap();
    let response = serde_json::to_string(&new_ticket).unwrap();
    tickets.push(new_ticket);
    HttpResponse::Created()
        .content_type(ContentType::json())
        .body(response)
}

// Get all tickets
#[get("/tickets")]
async fn get_tickets(data: web::Data<AppState>) -> impl Responder {
    let tickets = data.tickets.lock().unwrap();
    let response = serde_json::to_string(&(*tickets)).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response)
}

// Get a ticket with the corresponding id
#[get("/tickets/{id}")]
async fn get_ticket(id: web::Path<u32>, data: web::Data<AppState>) -> Result<Ticket, ErrNoId> {
    let ticket_id: u32 = *id;
    let tickets = data.tickets.lock().unwrap();
    let ticket: Vec<_> = tickets.iter().filter(|x| x.id == ticket_id).collect();
    if !ticket.is_empty() {
        Ok(Ticket {
            id: ticket[0].id,
            author: String::from(&ticket[0].author),
        })
    } else {
        let response = ErrNoId {
            id: ticket_id,
            err: String::from("ticket not found"),
        };
        Err(response)
    }
}

#[actix_web::main]
async fn seMain() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        tickets: Mutex::new(vec![
            Ticket {
                id: 1,
                author: String::from("Jane Doe"),
            },
            Ticket {
                id: 2,
                author: String::from("Patrick Star"),
            },
        ]),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(post_ticket)
            .service(get_ticket)
            .service(get_tickets)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
} */

fn systemctl_research(service: String) -> String {
    let output = Command::new("systemctl")
        .arg("status")
        .arg(service)
        .output()
        .expect("failed to execute process");
    let output = String::from_utf8_lossy(&output.stdout);
    let output = output.to_string();
    output
}

fn read_file(path: String) -> Result<String, String> {
    let mut file_content = String::new();
    let mut fr = fs::File::open(path)
        .map(|f| BufReader::new(f))
        .map_err(|e| e.to_string())?;
    fr.read_to_string(&mut file_content)
        .map_err(|e| e.to_string())?;
    Ok(file_content)
}

fn main() {
    let s = match read_file("./config.toml".to_owned()) {
        Ok(s) => s,
        Err(e) => panic!("fail to read file: {}", e),
    };
    let config: Result<Config, toml::de::Error> = toml::from_str(&s);
    let target = match config {
        Ok(p) => p.services,
        Err(e) => panic!("fail to parse toml: {}", e),
    };
    for service in target {
        let output = systemctl_research(service);
        println!("{}", output);
    }
}
