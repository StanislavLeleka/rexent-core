use crate::models::models::{
    Cast, Collection, Crew, Genre, Keyword, Movie, ProductionCompany, ProductionCountry,
    SpokenLanguage,
};
use csv::ReaderBuilder;
use std::env;
use std::{collections::HashMap, error::Error, fs::File, io::BufReader};

pub fn read_movies_metadata(file_path: String) -> Result<Vec<Movie>, Box<dyn Error>> {
    log::info!("Reading movies metadata");

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut csv_reader = ReaderBuilder::new().has_headers(true).from_reader(reader);
    let mut movies = vec![];
    let keywords = read_csv_file("out/keywords_clear.csv".to_string(), parse_keywords)?;
    let cast_crew = read_csv_file("out/cast_clear.csv".to_string(), parse_cast_and_crew)?;
    let min_votes_threshold = env::var("MIN_VOTES_THRESHOLD")
        .expect("$MIN_VOTES_THRESHOLD is not set")
        .parse::<u32>()?;

    let mut counter = 0;
    for line in csv_reader.records() {
        if let Ok(line) = line {
            if !line.is_empty() {
                log::info!("Processing movie #: {}", counter);
                let movie = process_movie_line(&line, &keywords, &cast_crew, min_votes_threshold);
                if let Ok(movie) = movie {
                    movies.push(movie);
                }
                counter += 1;
            }
        }
    }

    log::info!("Total movies: {:?}", movies.len());

    Ok(movies)
}

fn process_movie_line(
    line: &csv::StringRecord,
    keywords: &HashMap<i32, Vec<Keyword>>,
    cast_crew: &HashMap<i32, (Vec<Cast>, Vec<Crew>)>,
    min_votes_threshold: u32,
) -> Result<Movie, Box<dyn Error>> {
    let vote_count = line[24].parse::<f32>().unwrap_or_default() as u32;
    if vote_count < min_votes_threshold {
        return Err("low votes count".into());
    }

    let adult = line[1]
        .trim()
        .to_lowercase()
        .parse::<bool>()
        .unwrap_or_default();
    let belongs_to_collection = match serde_json::from_str::<Collection>(&line[2]) {
        Ok(collection) => Some(collection),
        Err(_) => None,
    };
    let budget = match line[3].parse::<u64>() {
        Ok(b) => Some(b),
        Err(_) => None,
    };
    let genres = serde_json::from_str::<Vec<Genre>>(&line[4])?;
    let homepage = line.get(5).filter(|s| !s.is_empty()).map(|s| s.to_string());
    let id = line[6].parse::<u32>()?;
    let imdb_id = line[7].to_string();
    let original_language = line[8].to_string();
    let original_title = line[9].to_string();
    let overview = line[10].to_string();
    let poster_path = line[12].to_string();
    let production_companies = serde_json::from_str::<Vec<ProductionCompany>>(&line[13])?;
    let production_countries = serde_json::from_str::<Vec<ProductionCountry>>(&line[14])?;
    let release_date = line[15].to_string();
    let revenue = line[16].parse::<f64>().unwrap_or_default() as u64;
    let runtime = line[17].parse::<f32>().unwrap_or_default() as u32;
    let spoken_languages = serde_json::from_str::<Vec<SpokenLanguage>>(&line[18])?;
    let tagline = line
        .get(20)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string());
    let title = line[21].to_string();
    let vote_average = line[23].parse::<f32>().unwrap_or_default();

    let id_i32 = id as i32;
    let keywords = keywords.get(&id_i32).cloned().unwrap_or_else(Vec::new);
    let (cast, crew) = cast_crew
        .get(&id_i32)
        .cloned()
        .unwrap_or_else(|| (Vec::new(), Vec::new()));

    let movie = Movie {
        adult,
        belongs_to_collection,
        budget,
        genres,
        homepage,
        id,
        imdb_id,
        original_language,
        original_title,
        overview,
        poster_path,
        production_companies,
        production_countries,
        release_date,
        revenue,
        runtime,
        spoken_languages,
        tagline,
        title,
        vote_average,
        vote_count,
        keywords,
        cast,
        crew,
    };

    Ok(movie)
}

fn read_csv_file<T>(
    file_path: String,
    parser: fn(&csv::StringRecord) -> Result<(i32, T), Box<dyn Error>>,
) -> Result<HashMap<i32, T>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut csv_reader = ReaderBuilder::new().has_headers(true).from_reader(reader);
    let mut map = HashMap::new();

    for line in csv_reader.records() {
        if let Ok(line) = line {
            let (id, data) = parser(&line)?;
            map.insert(id, data);
        }
    }

    Ok(map)
}

fn parse_keywords(line: &csv::StringRecord) -> Result<(i32, Vec<Keyword>), Box<dyn Error>> {
    let id = line[1].trim().parse::<i32>()?;
    let keywords = serde_json::from_str(&line[2])?;

    Ok((id, keywords))
}

fn parse_cast_and_crew(
    line: &csv::StringRecord,
) -> Result<(i32, (Vec<Cast>, Vec<Crew>)), Box<dyn Error>> {
    let id = line[3].trim().parse::<i32>()?;
    let cast = serde_json::from_str(&line[1])?;
    let crew = serde_json::from_str(&line[2])?;

    Ok((id, (cast, crew)))
}
