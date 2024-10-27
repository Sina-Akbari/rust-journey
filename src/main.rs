#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    fn description(&self) -> String {
        // if let Media::Book { title, author } = self {
        //     format!("Book: {} {}", title, author)
        // } else if let Media::Movie { title, director } = self {
        //     format!("Movie: {} {}", title, director)
        // } else if let Media::Audiobook { title } = self {
        //     format!("Audiobook: {}", title)
        // } else {
        //     String::from("Media Description")
        // }

        match self {
            Media::Book { title, author } => {
                format!("Book: {} {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie: {} {}", title, director)
            }
            Media::Audiobook { title } => {
                format!("Audiobook: {}", title)
            }
            Media::Podcast(id) => {
                format!("Podcast: {}", id)
            }
            Media::Placeholder => {
                format!("Placeholder")
            }
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    fn get_by_index(&self, idx: usize) -> MightHaveAValue {
        if self.items.len() > idx {
            MightHaveAValue::ThereIsAValue(&self.items[idx])
        } else {
            MightHaveAValue::NoValueAvailable
        }
    }
}

enum MightHaveAValue<'a> {
    ThereIsAValue(&'a Media),
    NoValueAvailable,
}

fn print_media(media: &Media) {
    println!("{:#?}", media);
}

fn main() {
    let mut catalog = Catalog::new();

    let audiobook = Media::Audiobook {
        title: String::from("An Audiobook"),
    };

    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Director"),
    };

    let bad_book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Author"),
    };

    let podcast = Media::Podcast(223);
    let placeholder = Media::Placeholder;

    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    println!("{:#?}", catalog.items.get(100));

    let item = catalog.get_by_index(3);

    match item {
        MightHaveAValue::ThereIsAValue(media) => {
            println!("{:#?}", media)
        }
        MightHaveAValue::NoValueAvailable => {
            println!("Nothing at that index")
        }
    }

    // match catalog.items.get(0) {
    //     Some(value) => {
    //         println!("Found an item: {:#?}", value)
    //     }
    //     None => {
    //         println!("Nothing at that index!")
    //     }
    // }

    // println!("{}", audiobook.description());
    // println!("{}", good_movie.description());
    // println!("{}", bad_book.description());

    // print_media(&audiobook);
    // print_media(&good_movie);
    // print_media(&bad_book);
}
