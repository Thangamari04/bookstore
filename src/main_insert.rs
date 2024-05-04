use std::error::Error;

struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
}

async fn create(book: &Book, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO book (title, author, isbn) VALUES ($1, $2, $3)";

    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .execute(pool)
        .await?;

    Ok(()) // Add return statement
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://postgres:postgres@localhost:5432/bookstore"; // Include the database name
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let book = Book {
        title: "Salem's Lot".to_string(),
        author: "Stephen".to_string(),
        isbn: "978-0-3851-1".to_string(),
    };

    create(&book, &pool).await?;

    Ok(())
}
