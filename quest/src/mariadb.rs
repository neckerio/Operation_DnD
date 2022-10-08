use sqlx::mysql::MySqlPoolOptions;

#[tokio::main]

pub async fn connect_to_database () -> Result<(), sqlx::Error> {
    
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://vagrant:vagrant@127.0.0.1/mariadb").await?;

    let row: (i64,) = sqlx::query_as("SELECT $?")
        .bind(150_i64)
        .fetch_one(&pool).await?;

    assert_eq!(row.0, 150);

    Ok(())
}




