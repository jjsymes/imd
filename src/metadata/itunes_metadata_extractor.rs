use std::time::Duration;

use chrono::{DateTime, Datelike};
use serde::Deserialize;
use url::Url;
use regex::Regex;
use super::song_metadata::SongMetadata;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ItunesSearchResult {
    #[serde(rename = "resultCount")]
    result_count: u32,
    results: Vec<ItunesSearchResultItem>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ItunesSearchResultItem {
    #[serde(rename = "wrapperType")]
    wrapper_type: Option<String>,
    kind: Option<String>,
    #[serde(rename = "artistId")]
    artist_id: Option<u32>,
    #[serde(rename = "collectionId")]
    collection_id: Option<u32>,
    #[serde(rename = "trackId")]
    track_id: Option<u32>,
    #[serde(rename = "artistName")]
    artist_name: Option<String>,
    #[serde(rename = "collectionName")]
    collection_name: Option<String>,
    #[serde(rename = "trackName")]
    track_name: Option<String>,
    #[serde(rename = "collectionCensoredName")]
    collection_censored_name: Option<String>,
    #[serde(rename = "trackCensoredName")]
    track_censored_name: Option<String>,
    #[serde(rename = "artistViewUrl")]
    artist_view_url: Option<String>,
    #[serde(rename = "collectionViewUrl")]
    collection_view_url: Option<String>,
    #[serde(rename = "trackViewUrl")]
    track_view_url: Option<String>,
    #[serde(rename = "previewUrl")]
    preview_url: Option<String>,
    #[serde(rename = "artworkUrl30")]
    artwork_url30: Option<String>,
    #[serde(rename = "artworkUrl60")]
    artwork_url60: Option<String>,
    #[serde(rename = "artworkUrl100")]
    artwork_url100: Option<String>,
    #[serde(rename = "collectionPrice")]
    collection_price: Option<f32>,
    #[serde(rename = "trackPrice")]
    track_price: Option<f32>,
    #[serde(rename = "releaseDate")]
    release_date: Option<String>,
    #[serde(rename = "collectionExplicitness")]
    collection_explicitness: Option<String>,
    #[serde(rename = "trackExplicitness")]
    track_explicitness: Option<String>,
    #[serde(rename = "discCount")]
    disc_count: Option<u16>,
    #[serde(rename = "discNumber")]
    disc_number: Option<u16>,
    #[serde(rename = "trackCount")]
    track_count: Option<u16>,
    #[serde(rename = "trackNumber")]
    track_number: Option<u16>,
    #[serde(rename = "trackTimeMillis")]
    track_time_millis: Option<u64>,
    country: Option<String>,
    currency: Option<String>,
    #[serde(rename = "primaryGenreName")]
    primary_genre_name: Option<String>,
    #[serde(rename = "isStreamable")]
    is_streamable: Option<bool>,
}

pub fn find_matching_metadata(song_metadata: &SongMetadata) -> Vec<SongMetadata> {
    validate_initial_data(song_metadata);
    let mut matching_items: Vec<SongMetadata> = Vec::new();

    let itunes_metadata_url = build_itunes_metadata_url(song_metadata);
    println!("iTunes metadata URL: {}", itunes_metadata_url);
    let itunes_search_result: Option<ItunesSearchResult> = reqwest::blocking::get(&itunes_metadata_url)
        .expect("Failed to get metadata from iTunes")
        .json()
        .expect("Failed to parse JSON response");

    match itunes_search_result {
        Some(r) => {
            if r.result_count == 0 {
                println!("No results found for matching metadata in iTunes, trying again with simplified search terms");
                let original_title = song_metadata.title.clone().unwrap_or_else(|| "".to_string());
                let original_artist = song_metadata.artist.clone().unwrap_or_else(|| "".to_string());
        
                let simplified_title = simplify_metadata_string(&original_title);
                let simplified_artist = simplify_metadata_string(&original_artist);
        
                if original_title == simplified_title && original_artist == simplified_artist {
                    panic!("ERROR: No results found for metadata matching for song: {} by {}", original_title, original_artist)
                } else {
                    let simplified_metadata = SongMetadata {
                        title: Some(simplified_title.clone().to_owned()),
                        artist: Some(simplified_artist.clone().to_owned()),
                        ..song_metadata.clone()
                    };
                    return find_matching_metadata(&simplified_metadata);
                }
            } else {
                let result_items = r.results;
                println!("Found {} results", result_items.len());
                println!("Results: {:?}", result_items);

                let result_metadata = result_items.iter()
                    .filter(|item| item.wrapper_type.as_ref().map(|s| s == "track").unwrap_or(false))
                    .map(|item| {
                        SongMetadata {
                            title: item.track_name.clone(),
                            artist: item.artist_name.clone(),
                            album: item.collection_name.clone(),
                            album_artist: None,
                            composer: None,
                            genre: item.primary_genre_name.clone(),
                            track_number: item.track_number.clone(),
                            disc_number: item.disc_number.clone(),
                            year: item.release_date.as_ref().map(|s| itunes_release_date_to_year(s)),
                            comment: None,
                            duration: item.track_time_millis.map(|n| Duration::from_millis(n)),
                            total_tracks: item.track_count.clone(),
                            total_discs: item.disc_count.clone(),
                            is_compilation: None,
                        }
                    })
                    .collect::<Vec<SongMetadata>>();
                matching_items.extend(result_metadata);
                return matching_items;
            }
        },
        None => { panic!("Somthing went wrong searching for metadata")}
    }
}

fn build_itunes_metadata_url(song_metadata: &SongMetadata) -> String {
    const ITUNES: &'static str = "https://itunes.apple.com";
    const SEARCH_API_PATH: &'static str = "search";
    let mut url = Url::parse(ITUNES).expect("hardcoded url is valid");
    url.set_path(SEARCH_API_PATH);

    let query_items = format!("{}+{}", song_metadata.title.as_ref().unwrap(), song_metadata.artist.as_ref().unwrap());
    let query = format!("term={}", query_items);

    url.set_query(Some(&query));

    return url.to_string();
}

fn validate_initial_data(initial_song_metadata: &SongMetadata) {
    if initial_song_metadata.title.is_none() {
        panic!("ERROR: Title is required!");
    }
    if initial_song_metadata.artist.is_none() {
        panic!("ERROR: Artist is required!");
    }
}

fn itunes_release_date_to_year(release_date: &str) -> u16 {
    return DateTime::parse_from_rfc3339(release_date).expect("Failed to parse date").year() as u16;
}

fn simplify_metadata_string(metadata_string: &str) -> String {
    let re = Regex::new(r"\s[\(\[].*[\)\]]").unwrap();
    return re.replace_all(metadata_string, "").trim().to_string();
}

#[test]
fn test_simplify_metadata_string() {

    // input, expected
    let test_cases = vec![
        ("BeepBoop (Single)", "BeepBoop"),
        ("f(x)", "f(x)"),
        ("Test Song [Remastered]", "Test Song"),
        ("Song With No Brackets", "Song With No Brackets"),
    ];

    for (input, expected) in test_cases {
        assert_eq!(expected, simplify_metadata_string(input));
    }
}
