// Publication enum tanımı

enum Publication {
    Book(Book),
    Magazine(Magazine),
}

// Book struct tanımı
struct Book {
    title: String,
    author: String,
    page_count: u32,
}

// Magazine struct tanımı
struct Magazine {
    title: String,
    issue: u32,
    topic: String,
}

//  Yayını türüne göre yazdır
fn print_publication(publication: &Publication) {
    match publication {
        Publication::Book(book) => {
            println!(
                "Book: {} author: {}, {} page",
                book.title, book.author, book.page_count
            );
        }
        Publication::Magazine(magazine) => {
            println!(
                "Magazine: {}  issue: {}, topic: {}",
                magazine.title, magazine.issue, magazine.topic
            );
        }
    }
}

fn main() {
    // Kitap ve dergi örnekleri 
    let book1 = Book {
        title: String::from("Gazap Üzümleri"),
        author: String::from("John Steinbeck"),
        page_count: 600,
    };

    let magazine1 = Magazine {
        title: String::from("Vogue"),
        issue: 25,
        topic: String::from("Moda"),
    };

    // Vec<Publication> dizisi 
    let publications: Vec<Publication> = vec![
        Publication::Book(book1),
        Publication::Magazine(magazine1),
       
    ];

    // Yayınları türüne göre yazdır
    for publication in &publications {
        print_publication(publication);
    }
}
