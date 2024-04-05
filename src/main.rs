use actix_web::{get, web, App, HttpServer, Responder};
use dotenv::dotenv;
use oracle::{
    pool::{self, Pool},
    Connection, Error,
};
use std::env;
use std::sync::Arc;

#[get("/parties/{party_id}")]
async fn index(web::Path(party_id): web::Path<u32>, pool: web::Data<Arc<Pool>>) -> impl Responder {
    let conn = pool
        .get_ref()
        .get()
        .expect("Failed to get connection from pool");
    get_party_info(party_id, &conn).await;
    format!("Hello {}! id", party_id)
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let db_url = construct_db_url().unwrap();
    let pool = Arc::new(create_connection_pool(&db_url)?);

    let _ = HttpServer::new(move || App::new().data(Arc::clone(&pool)).service(index))
        .bind("0.0.0.0:8080")
        .expect("Failed to bind to 0.0.0.0:8080")
        .run()
        .await;

    Ok(())
}

fn construct_db_url() -> Result<String, env::VarError> {
    let host_name = env::var("HOST_NAME")?;
    let port = env::var("PORT")?;
    let sid = env::var("SID")?;
    Ok(format!("//{}:{}/{}", host_name, port, sid))
}

fn create_connection_pool(db_url: &str) -> Result<Pool, Error> {
    let user = env::var("USER").expect("USER not set in .env file");
    let password = env::var("PASSWORD").expect("PASSWORD not set in .env file");

    pool::PoolBuilder::new(user, password, db_url)
        .max_connections(20)
        .build()
}

async fn get_party_info(party_id: u32, conn: &Connection) {
    let sql = format!(
        "select party_id,party_info from CCD.party_info_v2_v where party_id = {}",
        party_id
    );
    let mut stmt = conn.statement(&sql).build().unwrap();
    let rows = stmt.query(&[]).expect("Failed to execute query");

    println!("\nFirst Few Rows of PARTY_INFO_V2_V:");
    for row_result in rows {
        let row = row_result.expect("Failed to get row");
        dbg!(row);
    }
}
