use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeezeApiLoginAnonimo {
    pub jwt: String,
    #[serde(rename = "refresh_token")]
    pub refresh_token: String,
}


// INICIO: DATA FROM HTML

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeezerHTMLData {
    #[serde(rename = "DATA")]
    pub data: Data,
    #[serde(rename = "SONGS")]
    pub songs: Songs,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    #[serde(rename = "ART_NAME")]
    pub art_name: String,
    #[serde(rename = "ALB_TITLE")]
    pub alb_title: String,
    #[serde(rename = "ORIGINAL_RELEASE_DATE")]
    pub original_release_date: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Songs {
    pub data: Vec<Daum>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Daum {
    #[serde(rename = "SNG_ID")]
    pub sng_id: String,
    #[serde(rename = "SNG_TITLE")]
    pub sng_title: String,
    #[serde(rename = "LYRICS_ID")]
    pub lyrics_id: i64,
}
// FIN: DATA HTML

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeezerLetraData {
    pub data: DataLetra,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataLetra {
    pub track: Track,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub lyrics: Lyrics,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lyrics {
    pub text: String,
}