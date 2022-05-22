type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

use select::document::Document;
use select::predicate::{Class, Name};

#[derive(Debug)]
#[allow(dead_code)]
struct Movie {
    title: String,
    release_date: String,
    rated: String,
    metascore: String,
    summary: String,
    poster_url: String,
    movie_url: String,
}

fn main() -> Result<()> {
    let base_url = "https://metacritic.com";
    let req = reqwest::blocking::get(
        base_url.to_owned() + "/browse/movies/release-date/coming-soon/date",
    )?
    .text()?;
    let data = Document::from(req.as_str());
    let mut movies: Vec<Movie> = Vec::new();
    for info in data.find(Class("clamp-list")) {
        for info2 in info.find(Name("tr")) {
            if info2.attrs().next().is_some() {
                continue;
            }
            let title = info2.find(Name("h3")).next().unwrap().text().to_owned();
            let info3 = info2.find(Class("clamp-details")).next().unwrap().text();
            let info4: Vec<&str> = info3.split(" | ").collect();
            let release_date = info4[0][174..info4[0].len() - 91].to_owned();
            let rated = if info4.len() == 1 {
                String::from("Not Rated")
            } else {
                info4[1].split("\n\n").collect::<Vec<&str>>()[0].to_owned()
            };
            let metascore = info2.find(Class("metascore_anchor")).next().unwrap().text();
            let metascore = metascore[1..metascore.len() - 5].replace("tbd", "To Be Determined"); // see https://www.metacritic.com/faq#item13 for more inforamation.
            let summary_raw = info2.find(Class("summary")).next().unwrap().text();
            let summary = summary_raw[25..summary_raw.len() - 25].trim().to_owned();
            let info5 = info2.find(Class("clamp-image-wrap")).next().unwrap();
            let poster_url = info5
                .find(Name("img"))
                .next()
                .unwrap()
                .attr("src")
                .unwrap()
                .to_owned();
            let movie_url =
                base_url.to_owned() + info5.find(Name("a")).next().unwrap().attr("href").unwrap();
            let movie = Movie {
                title: title,
                release_date: release_date,
                rated: rated,
                metascore: metascore,
                summary: summary,
                poster_url: poster_url,
                movie_url: movie_url,
            };
            movies.push(movie)
        }
    }
    println!("{movies:#?}");
    Ok(())
}
