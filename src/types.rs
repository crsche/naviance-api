// use reqwest::Response;
use chrono::NaiveDate;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use crate::{
    util::{bool_from_int_opt, none_if_empty_string, none_if_zero, sat_to_act},
    Error, Result,
};

pub enum EndpointType {
    Public,
    Auth,
}

pub trait Endpoint // where
//     T: DeserializeOwned,
{
    const PATH: &'static str;
    const METHOD: http::Method;

    type Response;

    async fn extract(response: reqwest::Response) -> Result<Self::Response>;
}

pub trait PublicEndpoint: Endpoint {
    async fn request(mut base: Url, client: &reqwest::Client) -> Result<reqwest::Response> {
        base.set_path(Self::PATH);
        let request = client.request(Self::METHOD, base).build()?;
        Ok(client.execute(request).await?.error_for_status()?)
    }
}

pub trait AuthEndpoint: Endpoint {
    async fn request(
        mut api: Url,
        client: &reqwest::Client,
        token: &str,
    ) -> Result<reqwest::Response> {
        api.set_path(Self::PATH);
        let request = client
            .request(Self::METHOD, api)
            .bearer_auth(token)
            .build()?;
        Ok(client.execute(request).await?.error_for_status()?)
    }
}

/// ENDPOINT: https://student.naviance.com/rewritten_config.js
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Config {
    pub api_host: Option<String>,
    pub cpui_url: Option<String>,
    pub in_product_messaging_url: Option<String>,
    pub careers2_ui_url: Option<String>,
    pub hubsui_url: Option<String>,
    pub eventsui_url: Option<String>,
    pub scholarshipsui_url: Option<String>,
    pub naviancescholarshipsui_url: Option<String>,
    pub naviancesurveysui_url: Option<String>,
    pub supermatchui_url: Option<String>,
    pub activematchui_url: Option<String>,
    pub feedback_url: Option<String>,
    pub recaptcha_site_key: Option<String>,
    pub inline_manual_url: Option<String>,
    pub common_app_base_url: Option<String>,
    pub heap_analytics_api_code: Option<u32>,
    pub readiness_indicators_ui_url: Option<String>,
    pub phrase_batch_limit: Option<u32>,
    pub headed2_ui_url: Option<String>,
    pub headed2_token_exchange_url: Option<String>,
    pub local_opportunities_url: Option<String>,
    pub headed2_api_domain: Option<String>,
    pub headed2_app_base_url: Option<String>,
    pub my_pathways_ui_url: Option<String>,
    pub my_portfolio_ui_url: Option<String>,
    pub unified_user_login_url: Option<String>,
    pub ingest_raw_event_cta_app_base_url: Option<String>,
    pub portal_api_host: Option<String>,
    pub appily_match_ui_url: Option<String>,
    pub appily_match_api_url: Option<String>,
    pub chatterbox_ui_url: Option<String>,
    pub ace_chatbot_ui_url: Option<String>,
    pub ds_api_url: Option<String>,
    pub ds_api_temp_token: Option<String>,
}

impl Endpoint for Config {
    type Response = Self;

    const METHOD: http::Method = http::Method::GET;
    const PATH: &'static str = "/rewritten_config.js";

    async fn extract(response: reqwest::Response) -> Result<Self::Response> {
        let response = response.text().await?;
        let response = response
            .strip_prefix("window.REWRITTEN_CONFIG = ")
            .ok_or(Error::Other("config.js prefix not found".to_string()))?
            .strip_suffix(";")
            .ok_or(Error::Other("config.js suffix not found".to_string()))?;
        Ok(serde_json::from_str(response)?)
    }
}

impl PublicEndpoint for Config {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Paged<T> {
    pub page:        Option<u32>,
    pub limit:       Option<u32>,
    pub total_items: Option<u32>,
    pub total_pages: Option<u32>,
    pub data:        Vec<T>,
}

pub type SchoolsImThinkingAbout = Paged<School>;

impl Endpoint for SchoolsImThinkingAbout {
    type Response = Self;

    const METHOD: http::Method = http::Method::GET;
    const PATH: &'static str = "/college/colleges-im-thinking-about";

    async fn extract(response: reqwest::Response) -> Result<Self::Response> {
        Ok(response.json().await?)
    }
}

impl AuthEndpoint for SchoolsImThinkingAbout {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct School {
    pub id:                     Option<u32>,
    pub interest_level:         Option<u32>,
    pub expected_outcome:       Option<u32>,
    pub added_by_type:          Option<u32>,
    pub date_added:             Option<NaiveDate>,
    #[serde(deserialize_with = "none_if_empty_string")]
    pub college_id:             Option<String>,
    pub college:                Option<College>,
    pub expected_outcome_label: Option<String>,
    pub interest_level_label:   Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct College {
    pub featured: Option<bool>,
    #[serde(deserialize_with = "none_if_empty_string")]
    pub id: Option<String>,
    pub hobsons_id: Option<u32>,
    #[serde(deserialize_with = "none_if_empty_string")]
    pub name: Option<String>,
    #[serde(deserialize_with = "none_if_empty_string")]
    pub nces_id: Option<String>,
    #[serde(deserialize_with = "none_if_empty_string")]
    pub short_name: Option<String>,
    #[serde(deserialize_with = "none_if_empty_string")]
    pub nickname: Option<String>,
    #[serde(deserialize_with = "none_if_empty_string")]
    pub alpha_name: Option<String>,
    #[serde(deserialize_with = "none_if_empty_string")]
    pub address_line1: Option<String>,
    #[serde(deserialize_with = "none_if_empty_string")]
    pub city: Option<String>,
    #[serde(deserialize_with = "none_if_empty_string")]
    pub state: Option<String>,
    #[serde(deserialize_with = "none_if_empty_string")]
    pub country: Option<String>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    #[serde(deserialize_with = "none_if_empty_string")]
    pub url: Option<String>,
    pub sector: Option<u32>,
    #[serde(deserialize_with = "none_if_empty_string")]
    pub admissions_email: Option<String>,
    #[serde(deserialize_with = "none_if_empty_string")]
    pub intl_admissions_email: Option<String>,
    // pub hobsons_ext_profile: Option<serde_json::Value>,
    #[serde(deserialize_with = "bool_from_int_opt")]
    pub ssr_required: Option<bool>,
    #[serde(deserialize_with = "bool_from_int_opt")]
    pub teacher_recs_required: Option<bool>,
    #[serde(deserialize_with = "bool_from_int_opt")]
    pub initial_transcript_required: Option<bool>,
    // #[serde(deserialize_with = "bool_from_int_opt")]
    // pub continuous: Option<bool>,
    #[serde(deserialize_with = "bool_from_int_opt")]
    pub is_college_active: Option<bool>,
    #[serde(deserialize_with = "none_if_empty_string")]
    pub parchment_id: Option<String>,
    // pub scribbles_id: Option<serde_json::Value>,
    pub edocs_college: Option<EdocsCollege>,
    pub school_area: Option<SchoolArea>,
    pub core_mapping: Option<CoreMapping>,
    pub deadlines: Option<Vec<Deadline>>,
    #[serde(deserialize_with = "none_if_empty_string")]
    pub address_line2: Option<String>,
    pub uuid: Option<Uuid>,
}

impl Endpoint for College {
    type Response = Self;

    const METHOD: http::Method = http::Method::GET;
    const PATH: &'static str = "/college/uuid";

    async fn extract(response: reqwest::Response) -> Result<Self::Response> {
        Ok(response.json().await?)
    }
}

impl College {
    pub async fn request(
        mut api: Url,
        client: &reqwest::Client,
        token: &str,
        uuid: &Uuid,
    ) -> Result<reqwest::Response> {
        api.set_path(Self::PATH);
        api.path_segments_mut()
            .map_err(|_| Error::Other("API url is cannot-be-a-base".to_string()))?
            .push(&uuid.to_string());
        let request = client
            .request(Self::METHOD, api)
            .bearer_auth(token)
            .build()?;
        Ok(client.execute(request).await?.error_for_status()?)
    }

    // pub async fn extract(response: reqwest::Response) -> Result<Self> {
    // Ok(response.json().await?) }
}

// impl AuthEndpoint for College {
//     async fn request(
//         mut api: Url,
//         client: &reqwest::Client,
//         uuid: &str,
//         token: &str,
//     ) -> Result<reqwest::Response> {
//         api.set_path(Self::PATH);
//         api.path_segments_mut()?.push(uuid);
//         let request = client
//             .request(Self::METHOD, api)
//             .bearer_auth(token)
//             .build()?;
//         Ok(client.execute(request).await?.error_for_status()?)
//     }
// }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoreMapping {
    pub uuid: Option<Uuid>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deadline {
    pub id: Option<u32>,
    pub day: Option<u32>,
    pub month: Option<u32>,
    #[serde(deserialize_with = "none_if_empty_string")]
    pub deadline_label: Option<String>,
    #[serde(deserialize_with = "none_if_empty_string")]
    pub college_id: Option<String>,
    pub deadline_type_id: Option<u32>,
    #[serde(deserialize_with = "none_if_empty_string")]
    pub deadline_term_description: Option<String>,
    #[serde(rename = "type")]
    pub deadline_type: Option<String>,
    #[serde(deserialize_with = "none_if_empty_string")]
    pub label: Option<String>,
    #[serde(deserialize_with = "none_if_empty_string")]
    pub deadline_date: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EdocsCollege {
    #[serde(deserialize_with = "bool_from_int_opt")]
    pub is_electronic:          Option<bool>,
    #[serde(deserialize_with = "none_if_empty_string")]
    pub college_id:             Option<String>,
    pub commonapp_id:           Option<u32>,
    #[serde(deserialize_with = "bool_from_int_opt")]
    pub commonapp_is_exclusive: Option<bool>,
    #[serde(deserialize_with = "bool_from_int_opt")]
    pub coalition_app_type:     Option<bool>,
    #[serde(deserialize_with = "none_if_empty_string")]
    pub ceeb_code:              Option<String>,
    #[serde(deserialize_with = "none_if_empty_string")]
    pub delivery_type:          Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchoolArea {
    pub hobsons_id: Option<u32>,
    pub area_id:    Option<u32>,
}

pub type ScattergramSources = Vec<ScattergramSource>;

impl Endpoint for ScattergramSources {
    type Response = Self;

    const METHOD: http::Method = http::Method::GET;
    const PATH: &'static str = "/college/scattergram";

    async fn extract(response: reqwest::Response) -> Result<Self::Response> {
        Ok(response.json().await?)
    }
}

impl AuthEndpoint for ScattergramSources {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScattergramSource {
    #[serde(deserialize_with = "none_if_empty_string")]
    pub id:             Option<String>,
    #[serde(deserialize_with = "none_if_empty_string")]
    pub name:           Option<String>,
    pub core_mapping:   Option<CoreMapping>,
    pub total_applying: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationStatistics {
    pub scattergrams: Option<Scattergrams>,
    // pub application_statistics: Option<serde_json::Value>,
    // pub applications_by_year:   Option<serde_json::Value>,
    pub user_info:    Option<UserInfo>,
    // pub peer_gpa_map:           Option<Vec<Option<serde_json::Value>>>,
}

impl Endpoint for ApplicationStatistics {
    type Response = Self;

    const METHOD: http::Method = http::Method::GET;
    const PATH: &'static str = "/application-statistics/uuid";

    async fn extract(response: reqwest::Response) -> Result<Self::Response> {
        Ok(response.json().await?)
    }
}

impl ApplicationStatistics {
    pub async fn request(
        mut api: Url,
        client: &reqwest::Client,
        token: &str,
        uuid: &Uuid,
    ) -> Result<reqwest::Response> {
        api.set_path(Self::PATH);
        api.path_segments_mut()
            .map_err(|_| Error::Other("API url is cannot-be-a-base".to_string()))?
            .push(&uuid.to_string());
        let request = client
            .request(Self::METHOD, api)
            .bearer_auth(token)
            .build()?;
        Ok(client.execute(request).await?.error_for_status()?)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scattergrams {
    pub gpa:          Option<GpaSpecific>,
    pub weighted_gpa: Option<GpaSpecific>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GpaSpecific {
    pub gpa_count:    Option<u32>,
    pub gpa_sum:      Option<f64>,
    pub gpa_avg:      Option<f64>,
    pub gpa_conv_sum: Option<f64>,
    pub gpa_conv_avg: Option<f64>,
    pub act:          Option<TestSpecific<ACT>>,
    pub sat:          Option<TestSpecific<SAT>>,
}

pub trait TestType {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ACT;
impl TestType for ACT {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SAT;
impl TestType for SAT {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestSpecific<T: TestType> {
    pub count:        Option<u32>,
    pub sum:          Option<u32>,
    pub avg:          Option<f64>,
    pub gpa_count:    Option<u32>,
    pub gpa_sum:      Option<f64>,
    pub gpa_avg:      Option<f64>,
    pub gpa_conv_sum: Option<f64>,
    pub gpa_conv_avg: Option<f64>,
    pub apps:         Option<Apps<T>>,
    #[serde(skip)]
    _marker:          std::marker::PhantomData<T>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Apps<T: TestType> {
    pub denied:              Option<Vec<App<T>>>,
    pub waitlisted_denied:   Option<Vec<App<T>>>,
    pub waitlisted_accepted: Option<Vec<App<T>>>,
    pub waitlisted_unknown:  Option<Vec<App<T>>>,
    pub accepted:            Option<Vec<App<T>>>,
    #[serde(skip)]
    _marker:                 std::marker::PhantomData<T>,
}

impl<T: TestType> Apps<T> {
    pub fn all(&self) -> Vec<&App<T>> {
        let mut accepted = self.accepted();
        let mut denied = self.denied();
        accepted.append(&mut denied);
        accepted
    }

    pub fn denied(&self) -> Vec<&App<T>> {
        let mut total_denied = Vec::new();
        if let Some(denied) = &self.denied {
            total_denied.extend(denied.iter());
        }
        if let Some(waitlisted_unknown) = &self.waitlisted_unknown {
            total_denied.extend(waitlisted_unknown.iter());
        }
        if let Some(waitlisted_denied) = &self.waitlisted_unknown {
            total_denied.extend(waitlisted_denied.iter());
        }
        total_denied
    }

    pub fn accepted(&self) -> Vec<&App<T>> {
        let mut total_accepted = Vec::new();
        if let Some(accepted) = &self.accepted {
            total_accepted.extend(accepted.iter());
        }
        if let Some(waitlisted_accepted) = &self.waitlisted_accepted {
            total_accepted.extend(waitlisted_accepted.iter());
        }
        total_accepted
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct App<T: TestType> {
    pub current_student: Option<bool>,
    pub type_name: Option<TypeName>,
    #[serde(deserialize_with = "none_if_zero")]
    pub act_composite: Option<u32>,
    #[serde(deserialize_with = "none_if_zero")]
    pub act_composite_student: Option<u32>,
    #[serde(deserialize_with = "none_if_zero")]
    pub highest_combo_sat: Option<u32>,
    #[serde(rename = "studentSAT1600Composite")]
    #[serde(deserialize_with = "none_if_zero")]
    pub student_sat1600_composite: Option<u32>,
    // pub is_test_optional: Option<serde_json::Value>,
    pub gpa: Option<f64>,
    #[serde(skip)]
    _marker: std::marker::PhantomData<T>,
}

impl App<SAT> {
    pub fn to_act(&self) -> App<ACT> {
        App {
            current_student: self.current_student,
            type_name: self.type_name.clone(),
            act_composite: self.highest_combo_sat.map(|sat| sat_to_act(sat)),
            act_composite_student: None,
            highest_combo_sat: self.highest_combo_sat,
            student_sat1600_composite: self.student_sat1600_composite,
            gpa: self.gpa,
            _marker: std::marker::PhantomData,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Hash, Eq)]
pub enum TypeName {
    REA,
    EA,
    EA2,
    ED,
    ED2,
    RD,
    ROLL,
    #[serde(rename = "OTH")]
    OTH,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub user_id:   Option<u32>,
    pub academics: Option<Academics>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Academics {
    pub gpa:                Option<f64>,
    // pub weighted_gpa:       Option<serde_json::Value>,
    pub raw_cumulative_gpa: Option<f64>,
    #[serde(deserialize_with = "none_if_zero")]
    pub raw_weighted_gpa:   Option<u32>,
    #[serde(deserialize_with = "none_if_zero")]
    pub sat:                Option<u32>,
    #[serde(deserialize_with = "none_if_zero")]
    pub psat:               Option<u32>,
    #[serde(deserialize_with = "none_if_zero")]
    pub act:                Option<u32>,
}
