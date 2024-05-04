use std::error::Error;
use sqlx::Row;

struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
}

async fn retrieve(pool: &sqlx::PgPool) -> Result<Vec<Book>, Box<dyn Error>> {
    let query = "SELECT title, author, isbn FROM book";
    let mut books = Vec::new();

    let rows = sqlx::query(query)
        .fetch_all(pool)
        .await?;

    for row in rows {
       let title: String = row.try_get("title")?;
       let author: String = row.try_get("author")?;
       let isbn: String = row.try_get("isbn")?;

        let book = Book {
            title,
            author,
            isbn,
        };

        books.push(book);
    }

    Ok(books)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://postgres:postgres@localhost:5432/bookstore"; 
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let books = retrieve(&pool).await?;

    for book in books {
        println!("Title: {}, Author: {}, ISBN: {}", book.title, book.author, book.isbn);
    }

    Ok(())
}
