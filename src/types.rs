// Generated by https://quicktype.io

pub type CoronaData = Vec<CoronaDatum>;

#[derive(Serialize, Deserialize)]
pub struct CoronaDatum {
    #[serde(rename = "country")]
    country: String,

    #[serde(rename = "countryInfo")]
    country_info: CountryInfo,

    #[serde(rename = "cases")]
    cases: i64,

    #[serde(rename = "todayCases")]
    today_cases: i64,

    #[serde(rename = "deaths")]
    deaths: i64,

    #[serde(rename = "todayDeaths")]
    today_deaths: i64,

    #[serde(rename = "recovered")]
    recovered: i64,

    #[serde(rename = "active")]
    active: i64,

    #[serde(rename = "critical")]
    critical: i64,

    #[serde(rename = "casesPerOneMillion")]
    cases_per_one_million: i64,
}

#[derive(Serialize, Deserialize)]
pub struct CountryInfo {
    #[serde(rename = "iso2")]
    iso2: String,

    #[serde(rename = "iso3")]
    iso3: String,

    #[serde(rename = "_id")]
    id: IdUnion,

    #[serde(rename = "lat")]
    lat: f64,

    #[serde(rename = "long")]
    long: f64,

    #[serde(rename = "flag")]
    flag: String,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum IdUnion {
    Enum(IdEnum),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum IdEnum {
    #[serde(rename = "NO DATA")]
    NoData,
}
