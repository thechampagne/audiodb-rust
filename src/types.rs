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
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseArtist {
    pub artists: Vec<Artist>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    pub id_artist: Option<String>,
    pub str_artist: Option<String>,
    pub str_artist_stripped: Option<String>,
    pub str_artist_alternate: Option<String>,
    pub str_label: Option<String>,
    pub id_label: Option<String>,
    pub int_formed_year: Option<String>,
    pub int_born_year: Option<String>,
    pub int_died_year: Option<String>,
    pub str_disbanded: Option<String>,
    pub str_style: Option<String>,
    pub str_genre: Option<String>,
    pub str_mood: Option<String>,
    pub str_website: Option<String>,
    pub str_facebook: Option<String>,
    pub str_twitter: Option<String>,
    #[serde(rename = "strBiographyEN")]
    pub str_biography_en: Option<String>,
    #[serde(rename = "strBiographyDE")]
    pub str_biography_de: Option<String>,
    #[serde(rename = "strBiographyFR")]
    pub str_biography_fr: Option<String>,
    #[serde(rename = "strBiographyCN")]
    pub str_biography_cn: Option<String>,
    #[serde(rename = "strBiographyIT")]
    pub str_biography_it: Option<String>,
    #[serde(rename = "strBiographyJP")]
    pub str_biography_jp: Option<String>,
    #[serde(rename = "strBiographyRU")]
    pub str_biography_ru: Option<String>,
    #[serde(rename = "strBiographyES")]
    pub str_biography_es: Option<String>,
    #[serde(rename = "strBiographyPT")]
    pub str_biography_pt: Option<String>,
    #[serde(rename = "strBiographySE")]
    pub str_biography_se: Option<String>,
    #[serde(rename = "strBiographyNL")]
    pub str_biography_nl: Option<String>,
    #[serde(rename = "strBiographyHU")]
    pub str_biography_hu: Option<String>,
    #[serde(rename = "strBiographyNO")]
    pub str_biography_no: Option<String>,
    #[serde(rename = "strBiographyIL")]
    pub str_biography_il: Option<String>,
    #[serde(rename = "strBiographyPL")]
    pub str_biography_pl: Option<String>,
    pub str_gender: Option<String>,
    pub int_members: Option<String>,
    pub str_country: Option<String>,
    pub str_country_code: Option<String>,
    pub str_artist_thumb: Option<String>,
    pub str_artist_logo: Option<String>,
    pub str_artist_cutout: Option<String>,
    pub str_artist_clearart: Option<String>,
    pub str_artist_wide_thumb: Option<String>,
    pub str_artist_fanart: Option<String>,
    pub str_artist_fanart2: Option<String>,
    pub str_artist_fanart3: Option<String>,
    pub str_artist_fanart4: Option<String>,
    pub str_artist_banner: Option<String>,
    #[serde(rename = "strMusicBrainzID")]
    pub str_music_brainz_id: Option<String>,
    #[serde(rename = "strISNIcode")]
    pub str_isnicode: Option<String>,
    #[serde(rename = "strLastFMChart")]
    pub str_last_fmchart: Option<String>,
    pub int_charted: Option<String>,
    pub str_locked: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseDiscography {
    pub album: Vec<Discography>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Discography {
    pub str_album: Option<String>,
    pub int_year_released: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseAlbum {
    pub album: Vec<Album>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    pub id_album: Option<String>,
    pub id_artist: Option<String>,
    pub id_label: Option<String>,
    pub str_album: Option<String>,
    pub str_album_stripped: Option<String>,
    pub str_artist: Option<String>,
    pub str_artist_stripped: Option<String>,
    pub int_year_released: Option<String>,
    pub str_style: Option<String>,
    pub str_genre: Option<String>,
    pub str_label: Option<String>,
    pub str_release_format: Option<String>,
    pub int_sales: Option<String>,
    pub str_album_thumb: Option<String>,
    #[serde(rename = "strAlbumThumbHQ")]
    pub str_album_thumb_hq: Option<String>,
    pub str_album_thumb_back: Option<String>,
    #[serde(rename = "strAlbumCDart")]
    pub str_album_cdart: Option<String>,
    pub str_album_spine: Option<String>,
    #[serde(rename = "strAlbum3DCase")]
    pub str_album3dcase: Option<String>,
    #[serde(rename = "strAlbum3DFlat")]
    pub str_album3dflat: Option<String>,
    #[serde(rename = "strAlbum3DFace")]
    pub str_album3dface: Option<String>,
    #[serde(rename = "strAlbum3DThumb")]
    pub str_album3dthumb: Option<String>,
    #[serde(rename = "strDescriptionEN")]
    pub str_description_en: Option<String>,
    #[serde(rename = "strDescriptionDE")]
    pub str_description_de: Option<String>,
    #[serde(rename = "strDescriptionFR")]
    pub str_description_fr: Option<String>,
    #[serde(rename = "strDescriptionCN")]
    pub str_description_cn: Option<String>,
    #[serde(rename = "strDescriptionIT")]
    pub str_description_it: Option<String>,
    #[serde(rename = "strDescriptionJP")]
    pub str_description_jp: Option<String>,
    #[serde(rename = "strDescriptionRU")]
    pub str_description_ru: Option<String>,
    #[serde(rename = "strDescriptionES")]
    pub str_description_es: Option<String>,
    #[serde(rename = "strDescriptionPT")]
    pub str_description_pt: Option<String>,
    #[serde(rename = "strDescriptionSE")]
    pub str_description_se: Option<String>,
    #[serde(rename = "strDescriptionNL")]
    pub str_description_nl: Option<String>,
    #[serde(rename = "strDescriptionHU")]
    pub str_description_hu: Option<String>,
    #[serde(rename = "strDescriptionNO")]
    pub str_description_no: Option<String>,
    #[serde(rename = "strDescriptionIL")]
    pub str_description_il: Option<String>,
    #[serde(rename = "strDescriptionPL")]
    pub str_description_pl: Option<String>,
    pub int_loved: Option<String>,
    pub int_score: Option<String>,
    pub int_score_votes: Option<String>,
    pub str_review: Option<String>,
    pub str_mood: Option<String>,
    pub str_theme: Option<String>,
    pub str_speed: Option<String>,
    pub str_location: Option<String>,
    #[serde(rename = "strMusicBrainzID")]
    pub str_music_brainz_id: Option<String>,
    #[serde(rename = "strMusicBrainzArtistID")]
    pub str_music_brainz_artist_id: Option<String>,
    #[serde(rename = "strAllMusicID")]
    pub str_all_music_id: Option<String>,
    #[serde(rename = "strBBCReviewID")]
    pub str_bbcreview_id: Option<String>,
    #[serde(rename = "strRateYourMusicID")]
    pub str_rate_your_music_id: Option<String>,
    #[serde(rename = "strDiscogsID")]
    pub str_discogs_id: Option<String>,
    #[serde(rename = "strWikidataID")]
    pub str_wikidata_id: Option<String>,
    #[serde(rename = "strWikipediaID")]
    pub str_wikipedia_id: Option<String>,
    #[serde(rename = "strGeniusID")]
    pub str_genius_id: Option<String>,
    #[serde(rename = "strLyricWikiID")]
    pub str_lyric_wiki_id: Option<String>,
    #[serde(rename = "strMusicMozID")]
    pub str_music_moz_id: Option<String>,
    #[serde(rename = "strItunesID")]
    pub str_itunes_id: Option<String>,
    #[serde(rename = "strAmazonID")]
    pub str_amazon_id: Option<String>,
    pub str_locked: Option<String>,
    pub str_description: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseTrack {
    pub track: Vec<Track>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub id_track: Option<String>,
    pub id_album: Option<String>,
    pub id_artist: Option<String>,
    pub id_lyric: Option<String>,
    #[serde(rename = "idIMVDB")]
    pub id_imvdb: Option<String>,
    pub str_track: Option<String>,
    pub str_album: Option<String>,
    pub str_artist: Option<String>,
    pub str_artist_alternate: Option<String>,
    #[serde(rename = "intCD")]
    pub int_cd: Option<String>,
    pub int_duration: Option<String>,
    pub str_genre: Option<String>,
    pub str_mood: Option<String>,
    pub str_style: Option<String>,
    pub str_theme: Option<String>,
    #[serde(rename = "strDescriptionEN")]
    pub str_description_en: Option<String>,
    #[serde(rename = "strDescriptionDE")]
    pub str_description_de: Option<String>,
    #[serde(rename = "strDescriptionFR")]
    pub str_description_fr: Option<String>,
    #[serde(rename = "strDescriptionCN")]
    pub str_description_cn: Option<String>,
    #[serde(rename = "strDescriptionIT")]
    pub str_description_it: Option<String>,
    #[serde(rename = "strDescriptionJP")]
    pub str_description_jp: Option<String>,
    #[serde(rename = "strDescriptionRU")]
    pub str_description_ru: Option<String>,
    #[serde(rename = "strDescriptionES")]
    pub str_description_es: Option<String>,
    #[serde(rename = "strDescriptionPT")]
    pub str_description_pt: Option<String>,
    #[serde(rename = "strDescriptionSE")]
    pub str_description_se: Option<String>,
    #[serde(rename = "strDescriptionNL")]
    pub str_description_nl: Option<String>,
    #[serde(rename = "strDescriptionHU")]
    pub str_description_hu: Option<String>,
    #[serde(rename = "strDescriptionNO")]
    pub str_description_no: Option<String>,
    #[serde(rename = "strDescriptionIL")]
    pub str_description_il: Option<String>,
    #[serde(rename = "strDescriptionPL")]
    pub str_description_pl: Option<String>,
    pub str_track_thumb: Option<String>,
    #[serde(rename = "strTrack3DCase")]
    pub str_track3dcase: Option<String>,
    pub str_track_lyrics: Option<String>,
    pub str_music_vid: Option<String>,
    pub str_music_vid_director: Option<String>,
    pub str_music_vid_company: Option<String>,
    pub str_music_vid_screen1: Option<String>,
    pub str_music_vid_screen2: Option<String>,
    pub str_music_vid_screen3: Option<String>,
    pub int_music_vid_views: Option<String>,
    pub int_music_vid_likes: Option<String>,
    pub int_music_vid_dislikes: Option<String>,
    pub int_music_vid_favorites: Option<String>,
    pub int_music_vid_comments: Option<String>,
    pub int_track_number: Option<String>,
    pub int_loved: Option<String>,
    pub int_score: Option<String>,
    pub int_score_votes: Option<String>,
    pub int_total_listeners: Option<String>,
    pub int_total_plays: Option<String>,
    #[serde(rename = "strMusicBrainzID")]
    pub str_music_brainz_id: Option<String>,
    #[serde(rename = "strMusicBrainzAlbumID")]
    pub str_music_brainz_album_id: Option<String>,
    #[serde(rename = "strMusicBrainzArtistID")]
    pub str_music_brainz_artist_id: Option<String>,
    pub str_locked: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseMusicVideo {
    pub mvids: Vec<MusicVideo>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicVideo {
    pub id_artist: Option<String>,
    pub id_album: Option<String>,
    pub id_track: Option<String>,
    pub str_track: Option<String>,
    pub str_track_thumb: Option<String>,
    pub str_music_vid: Option<String>,
    #[serde(rename = "strDescriptionEN")]
    pub str_description_en: Option<String>,
}