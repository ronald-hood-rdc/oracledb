use dotenv::dotenv;
use oracle::{
    pool::{self, Pool},
    Connection, Error,
};
use std::env;
use std::fs::File;
use std::io::{Result as IoResult, Write};
use std::thread;
use std::time::Duration;

const LIST_VIEWS_QUERY: &str = "SELECT view_name FROM all_views";

fn main() -> Result<(), Error> {
    dotenv().ok();
    let db_url = construct_db_url().unwrap();
    let pool = create_connection_pool(&db_url)?;
    let conn = pool.get()?;
    execute_and_print_query(&conn, LIST_VIEWS_QUERY)?;
    thread::sleep(Duration::from_secs(5));
    
    let mut stmt = conn.statement(LIST_VIEWS_QUERY).build()?;
    let rows = stmt.query(&[])?;

    let mut views_columns = Vec::new();

    for row_result in rows {
        let row = row_result?;
        let view_name: String = row.get("view_name")?;

        let columns = get_columns_for_view(&conn, &view_name.to_uppercase())?;
        dbg!((&view_name, &columns));
        views_columns.push((view_name, columns));
    }

    write_schema_file(views_columns).unwrap();

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

    pool::PoolBuilder::new(user, password, db_url).build()
}

fn get_columns_for_view(
    conn: &Connection,
    view_name: &str,
) -> Result<Vec<(String, String)>, Error> {
    let sql = "SELECT column_name, data_type FROM all_tab_columns WHERE table_name = :1";
    let mut stmt = conn.statement(sql).build()?;
    let rows = stmt.query(&[&view_name])?;

    let mut columns = Vec::new();
    for row_result in rows {
        let row = row_result?;
        let column_name: String = row.get("column_name")?;
        let data_type: String = row.get("data_type")?;
        columns.push((column_name, data_type));
    }

    Ok(columns)
}

fn oracle_type_to_graphql(oracle_type: &str) -> &'static str {
    match oracle_type {
        "VARCHAR2" | "CHAR" | "NVARCHAR2" | "CLOB" => "String",
        "NUMBER" | "FLOAT" | "DECIMAL" => "Float",
        "INTEGER" | "SMALLINT" => "Int",
        "DATE" | "TIMESTAMP" => "String", // Dates and timestamps are represented as strings in GraphQL; consider using custom scalar types for date/time.
        _ => "String",                    // Default fallback
    }
}

fn write_schema_file(views_columns: Vec<(String, Vec<(String, String)>)>) -> IoResult<()> {
    let mut file = File::create("schema.graphql")?;

    for (view_name, columns) in views_columns {
        writeln!(&mut file, "type {} {{", view_name)?;
        for (column_name, data_type) in columns {
            let graphql_type = oracle_type_to_graphql(&data_type);
            writeln!(&mut file, "  {}: {}", column_name, graphql_type)?;
        }
        writeln!(&mut file, "}}\n")?;
    }

    Ok(())
}
fn execute_and_print_query(conn: &Connection, sql_query: &str) -> Result<(), Error> {
    let mut stmt = conn.statement(sql_query).build()?;
    let rows = stmt.query(&[])?;
    for row_result in rows {
        let row = row_result?;
        let view_name: String = row.get("view_name")?;
        println!("View: {}", view_name);
    }
    Ok(())
}
