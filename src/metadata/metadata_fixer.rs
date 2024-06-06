use crate::metadata::itunes_metadata_extractor::find_matching_metadata;
use crate::metadata::metadata_comparator::MetadataComparator;
use super::song_metadata::SongMetadata;


pub fn get_fixed_metadata(metadata: &SongMetadata) -> SongMetadata {
    let matching_metadata_candidates: Vec<SongMetadata> = find_matching_metadata(metadata);

    let mut metadata_scores: Vec<(&SongMetadata, f64)> = matching_metadata_candidates.iter()
        .map(|metadata_candidate| {
            let score = MetadataComparator::new(metadata.clone(), metadata_candidate.clone()).get_overall_score();
            (metadata_candidate, score)
        })
        .collect();

    metadata_scores.sort_by_key(|(_, score)| (*score * 1000.0) as i64);

    // print top 5 matches
    println!("########################################################################################");
    println!("Top 5 matches:");
    for (metadata_candidate, score) in metadata_scores.iter().take(5) {
        println!("Score: {:.2} - {:?}", score, metadata_candidate);
    }
    println!("########################################################################################");

    let best_match = metadata_scores.last().unwrap();

    let best_match_song_metadata = best_match.0;

    println!("########################################################################################");
    println!("Best match: {:?}", best_match_song_metadata);
    println!("########################################################################################");

    return combine_metadata(metadata, best_match_song_metadata);
}

fn combine_metadata(original_song_metadata: &SongMetadata, best_match: &SongMetadata) -> SongMetadata {
    SongMetadata {
        title: best_match.title.clone().or(original_song_metadata.title.clone()),
        artist: best_match.artist.clone().or(original_song_metadata.artist.clone()),
        album: best_match.album.clone().or(original_song_metadata.album.clone()),
        album_artist: best_match.album_artist.clone().or(original_song_metadata.album_artist.clone()),
        composer: best_match.composer.clone(),
        genre: best_match.genre.clone().or(original_song_metadata.genre.clone()),
        track_number: best_match.track_number.or(original_song_metadata.track_number.clone()),
        disc_number: best_match.disc_number.or(original_song_metadata.disc_number.clone()),
        year: best_match.year.or(original_song_metadata.year.clone()),
        comment: best_match.comment.clone(),
        duration: original_song_metadata.duration.clone(),
        total_tracks: best_match.total_tracks.or(original_song_metadata.total_tracks.clone()),
        total_discs: best_match.total_discs.or(original_song_metadata.total_discs.clone()),
        is_compilation: best_match.is_compilation.or(original_song_metadata.is_compilation.clone()),
    }
}