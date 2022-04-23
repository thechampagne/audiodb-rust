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
//! TheAudioDB API client
//!
//! TheAudioDB is a community Database of
//! audio artwork and metadata
mod audiodb;
mod types;
mod error;
pub use error::AudioDBError;
pub use audiodb::search_artist;
pub use audiodb::search_album_by_id;
pub use audiodb::discography;
pub use audiodb::search_artist_by_id;
pub use audiodb::search_tracks_by_album_id;
pub use audiodb::search_albums_by_artist_id;
pub use audiodb::search_music_videos_by_artist_id;
pub use audiodb::search_track_by_id;
pub use types::Track;
pub use types::MusicVideo;
pub use types::Discography;
pub use types::Album;
pub use types::Artist;