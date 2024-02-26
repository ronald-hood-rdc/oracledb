fn create_connections_until_max(db_url: &str) -> Result<Vec<Connection>, Error> {
    let mut connections = Vec::new();
    let mut error_occurred = false;
    let user = env::var("USER").expect("USER not set in .env file");
    let password = env::var("PASSWORD").expect("PASSWORD not set in .env file");

    while !error_occurred {
        match Connection::connect(&user, &password, db_url) {
            Ok(conn) => {
                println!(
                    "Successfully connected to the database. {}",
                    connections.len()
                );
                connections.push(conn);
                // Simulate workload or pause to avoid overwhelming the DB
                //thread::sleep(Duration::from_millis(100));
            }
            Err(e) => match e {
                Error::OciError(oracle_error) => {
                    if oracle_error.code() == 12516 {
                        // Adjust based on your DB's error code for max connections
                        println!("Maximum number of connections reached.");
                        error_occurred = true;
                    } else {
                        println!("An unexpected error occurred: {:?}", oracle_error);
                        error_occurred = true;
                    }
                }
                _ => {
                    println!("An unexpected error occurred: {:?}", e);
                    error_occurred = true;
                }
            },
        }
    }

    Ok(connections)
}

fn list_views(conn: &Connection) {
    let mut stmt = conn.statement(LIST_VIEWS_QUERY).build().unwrap();
    let rows = stmt.query(&[]).unwrap();

    /*let mut views_columns = Vec::new();

    for row_result in rows {
        let row = row_result?;
        let view_name: String = row.get("view_name")?;

        let columns = get_columns_for_view(&conn, &view_name.to_uppercase())?;
        dbg!((&view_name, &columns));
        views_columns.push((view_name, columns));
    }

    write_schema_file(views_columns).unwrap();*/
}

fn print_setting(conn: &Connection, query: &str, setting_name: &str) -> Result<(), Error> {
    let mut stmt = conn.statement(query).build()?;
    let mut rows = stmt.query(&[])?;

    if let Some(row_result) = rows.next() {
        let row = row_result?;
        let value: String = row.get("value")?;
        println!("{}: {}", setting_name, value);
    }

    Ok(())
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
