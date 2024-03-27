use dotenv::dotenv;
use oracle::{
    pool::{self, Pool},
    Error, RowValue,
};
use std::env;
use std::fs::File;
use std::io::Write;
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
    let mut file = File::create("views.txt").expect("error");

    /*let mut stmt = conn.statement(LIST_VIEWS_QUERY).build().unwrap();
    let rows = stmt.query(&[]).unwrap();


    for row_result in rows {
        let row: oracle::Row = row_result?; // Handle the Result from iterating over rows
        let view_name: String = row.get("view_name")?; // Extract the view_name column


    }

    //list_views(&conn);

    /*  let mut stmt = conn.statement(LIST_VIEWS_QUERY).build()?;
    let rows = stmt.query_as::<MlsContactsV2>(&[])?;

    for row_result in rows {
        let row: MlsContactsV2 = row_result?;
        println!("{:?}", row);
    }*/

    const GET_COLUMN_NAMES_QUERY: &str = "
        SELECT COLUMN_NAME
        FROM ALL_TAB_COLUMNS
        WHERE TABLE_NAME = 'PARTY_INFO_V2_V'";
    const GET_FIRST_ROWS_QUERY: &str = "
        SELECT *
        FROM PARTY_INFO_V2_V
        FETCH FIRST 5 ROWS ONLY";

    // Query and display column names for PARTY_INFO_V2_V
    let mut stmt = conn.statement(GET_COLUMN_NAMES_QUERY).build().unwrap();
    let cols = stmt.query(&[]).unwrap();

    println!("Column Names for PARTY_INFO_V2_V:");
    for col_result in cols {
        let col: oracle::Row = col_result?;
        let column_name: String = col.get("COLUMN_NAME")?;
        println!("{}", column_name);
    }
    // here*/
    let sql = "select party_id,party_info from CCD.party_info_v2_v";
    //et sql =
    //"SELECT * FROM DBA_TAB_PRIVS WHERE TABLE_NAME = 'PARTY_INFO_V2_V' AND PRIVILEGE = 'SELECT'"; //"SELECT VIEW_NAME FROM ALL_VIEWS WHERE VIEW_NAME = 'PARTY_INFO_V2_V'";
    // Query and display the first few rows of PARTY_INFO_V2_V
    let mut stmt = conn.statement(sql).build().unwrap();
    let rows = stmt.query(&[])?;

    println!("\nFirst Few Rows of PARTY_INFO_V2_V:");
    for row_result in rows {
        // Here we handle the Result type for each row
        let row = row_result?;
        // Now, safely extract the expected tuple from the row

        writeln!(&mut file, "View Name: {:?}", row).expect("msg");
    }

    Ok(())
}

fn construct_db_url() -> Result<String, env::VarError> {
    let host_name = env::var("HOST_NAME")?;
    let port = env::var("PORT")?;
    let sid = env::var("SID")?;
    let stringg = format!("//{}:{}/{}", host_name, port, sid);
    dbg!(stringg);
    Ok(format!("//{}:{}/{}", host_name, port, sid))
}

fn create_connection_pool(db_url: &str) -> Result<Pool, Error> {
    let user = env::var("USER").expect("USER not set in .env file");
    let password = env::var("PASSWORD").expect("PASSWORD not set in .env file");

    pool::PoolBuilder::new(user, password, db_url)
        .max_connections(20)
        .build()
}
// account hierarchy table and account response has a acouple attributes. Look at schema, partyid is the objectid.
/*
 * subject id is the party id for that account, partyid means that the account might be parent of another account.
 relationship type explains how the subject id is related to the party id, contact or member or parernt etc.
 In CIS, go toa ccount, getch infromation and based on the relationship it foes to the hierarchy table. Trying to fetch from different views then does the stitching work
 If there is a view it will just fetch from the view, graphql may not be optimal.
 Graphql is better at one time fetch rather than stitching things together.
 If try to keep flatten file, it will be overwhelming for the update of the file. Updates the view so that specific informaiton si updated,
 the entire response is then generated lazely.
not everyone is interested in document type, if yuo could expose partyid. logic will be handled by the client.
can fetch information for indiviudal account for the mls sets. If they want the entire hierarchy then they can go to legqcy CIS.

 */
