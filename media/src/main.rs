mod content;

use content::{catalog::Catalog, media::Media};

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
}
