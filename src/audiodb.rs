/*
 * Copyright 2022 XXIV
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use std::io::Read;
use urlencoding::encode;
use crate::error::AudioDBError;
use crate::types::{Album,
                   Artist,
                   BaseAlbum,
                   BaseArtist,
                   BaseDiscography,
                   BaseMusicVideo,
                   BaseTrack,
                   Discography,
                   MusicVideo,
                   Track};

fn http(endpoint: &str) -> Option<String> {
    match reqwest::blocking::Client::new().get(format!("https://theaudiodb.com/api/v1/json/2/{}", endpoint))
        .send() {
        Ok(mut response) => {
            let mut body = String::new();
            match response.read_to_string(&mut body) {
                Ok(_) => Some(body),
                Err(_) => None
            }
        },
        Err(_) => None
    }
}

/// Return Artist details from artist name
pub fn search_artist(s: &str) -> Result<Artist, AudioDBError> {
    match http(&format!("search.php?s={}", encode(s))) {
        Some(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let data: BaseArtist = json;
                    if data.artists.is_empty() {
                        Err(AudioDBError::Error(String::from("null")))
                    } else {
                        Ok(data.artists[0].clone())
                    }
                },
                Err(_) => Err(AudioDBError::Error(String::from("null")))
            }
        },
        None => Err(AudioDBError::Error(String::from("null")))
    }
}

/// Return Discography for an Artist with Album names and year only
pub fn discography(s: &str) -> Result<Vec<Discography>, AudioDBError> {
    match http(&format!("discography.php?s={}", encode(s))) {
        Some(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let mut vector = vec![];
                    let data: BaseDiscography = json;
                    for i in data.album {
                        vector.push(i)
                    }
                    if vector.is_empty() {
                        Err(AudioDBError::Error(String::from("null")))
                    } else {
                        Ok(vector)
                    }
                },
                Err(_) => Err(AudioDBError::Error(String::from("null")))
            }
        },
        None => Err(AudioDBError::Error(String::from("null")))
    }
}

/// Return individual Artist details using known Artist ID
pub fn search_artist_by_id(i: i64) -> Result<Artist, AudioDBError> {
    match http(&format!("artist.php?i={}", i)) {
        Some(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let data: BaseArtist = json;
                    if data.artists.is_empty() {
                        Err(AudioDBError::Error(String::from("null")))
                    } else {
                        Ok(data.artists[0].clone())
                    }
                },
                Err(_) => Err(AudioDBError::Error(String::from("null")))
            }
        },
        None => Err(AudioDBError::Error(String::from("null")))
    }
}

/// Return individual Album info using known Album ID
pub fn search_album_by_id(i: i64) -> Result<Album, AudioDBError> {
    match http(&format!("album.php?m={}", i)) {
        Some(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let data: BaseAlbum = json;
                    if data.album.is_empty() {
                        Err(AudioDBError::Error(String::from("null")))
                    } else {
                        Ok(data.album[0].clone())
                    }
                },
                Err(_) => Err(AudioDBError::Error(String::from("null")))
            }
        },
        None => Err(AudioDBError::Error(String::from("null")))
    }
}

/// Return All Albums for an Artist using known Artist ID
pub fn search_albums_by_artist_id(i: i64) -> Result<Vec<Album>, AudioDBError> {
    match http(&format!("album.php?i={}", i)) {
        Some(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let mut vector = vec![];
                    let data: BaseAlbum = json;
                    for i in data.album {
                        vector.push(i)
                    }
                    if vector.is_empty() {
                        Err(AudioDBError::Error(String::from("null")))
                    } else {
                        Ok(vector)
                    }
                },
                Err(_) => Err(AudioDBError::Error(String::from("null")))
            }
        },
        None => Err(AudioDBError::Error(String::from("null")))
    }
}

/// Return All Tracks for Album from known Album ID
pub fn search_tracks_by_album_id(i: i64) -> Result<Vec<Track>, AudioDBError> {
    match http(&format!("track.php?m={}", i)) {
        Some(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let mut vector = vec![];
                    let data: BaseTrack = json;
                    for i in data.track {
                        vector.push(i)
                    }
                    if vector.is_empty() {
                        Err(AudioDBError::Error(String::from("null")))
                    } else {
                        Ok(vector)
                    }
                },
                Err(_) => Err(AudioDBError::Error(String::from("null")))
            }
        },
        None => Err(AudioDBError::Error(String::from("null")))
    }
}

/// Return individual track info using a known Track ID
pub fn search_track_by_id(i: i64) -> Result<Track, AudioDBError> {
    match http(&format!("track.php?h={}", i)) {
        Some(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let data: BaseTrack = json;
                    if data.track.is_empty() {
                        Err(AudioDBError::Error(String::from("null")))
                    } else {
                        Ok(data.track[0].clone())
                    }
                },
                Err(_) => Err(AudioDBError::Error(String::from("null")))
            }
        },
        None => Err(AudioDBError::Error(String::from("null")))
    }
}

/// Return all the Music videos for a known Artist ID
pub fn search_music_videos_by_artist_id(i: i64) -> Result<Vec<MusicVideo>, AudioDBError> {
    match http(&format!("mvid.php?i={}", i)) {
        Some(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let mut vector = vec![];
                    let data: BaseMusicVideo = json;
                    for i in data.mvids {
                        vector.push(i)
                    }
                    if vector.is_empty() {
                        Err(AudioDBError::Error(String::from("null")))
                    } else {
                        Ok(vector)
                    }
                },
                Err(_) => Err(AudioDBError::Error(String::from("null")))
            }
        },
        None => Err(AudioDBError::Error(String::from("null")))
    }
}