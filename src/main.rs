use dotenv::dotenv;
use oracle::{
    pool::{self, Pool},
    Error, RowValue,
};
use std::env;

const LIST_VIEWS_QUERY: &str = "SELECT view_name FROM all_views";

// Define a struct to hold the row data
#[derive(Debug, RowValue)]
struct MlsContactsV2 {
    mls_account_party_id: Option<f64>, // Using Option<> to handle possible NULL values
    mls_formal_name: Option<String>,
    mls_id: Option<String>,
    ir_rep: Option<String>,
    mls_internet_name: Option<String>,
    state: Option<String>,
    mls_account_fulfillment_id: Option<String>,
    contact_info: Option<String>,
    doc_id: Option<f64>,
}

// Adjusted query to select specified fields from MLS_CONTACTS_V2 and limit to first 5 rows
const LIST_MLS_CONTACTS_QUERY: &str = "
SELECT MLS_ACCOUNT_PARTY_ID, MLS_FORMAL_NAME, MLS_ID, IR_REP, MLS_INTERNET_NAME, 
       STATE, MLS_ACCOUNT_FULFILLMENT_ID, CONTACT_INFO, DOC_ID 
FROM MLS_CONTACTS_V2 
FETCH FIRST 5 ROWS ONLY";

fn main() -> Result<(), Error> {
    dotenv().ok();
    let db_url = construct_db_url().unwrap();
    let pool = create_connection_pool(&db_url)?;
    let conn = pool.get()?;

    //let mut stmt = conn.statement(LIST_VIEWS_QUERY).build().unwrap();
    // let rows = stmt.query(&[]).unwrap();

    //   for row_result in rows {
    //     let row: oracle::Row = row_result?; // Handle the Result from iterating over rows
    //      let view_name: String = row.get("view_name")?; // Extract the view_name column
    //       println!("View Name: {}", view_name);
    //   }
    //list_views(&conn);

    let mut stmt = conn.statement(LIST_MLS_CONTACTS_QUERY).build()?;
    let rows = stmt.query_as::<MlsContactsV2>(&[])?;

    for row_result in rows {
        let row: MlsContactsV2 = row_result?;
        println!("{:?}", row);
    }
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
