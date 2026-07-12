use anyhow::Result;
use chrono::{DateTime, Utc};
use proto::messages::{ApiInfo, NEXT_SUBWAY_ERROR_VALUE};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct NextSubwayArrivalsResponse {
    #[serde(rename = "Siri")]
    pub siri: Siri,
}

impl NextSubwayArrivalsResponse {
    pub fn into_next_arrivals(self) -> Vec<DateTime<Utc>> {
        self.siri
            .service_delivery
            .stop_monitoring_delivery
            .into_iter()
            .flat_map(|d| d.monitored_stop_visit)
            .map(|v| {
                v.monitored_vehicle_journey
                    .monitored_call
                    .expected_arrival_time
            })
            .collect::<Vec<_>>()
    }
}

#[derive(Debug, Deserialize)]
struct Siri {
    #[serde(rename = "ServiceDelivery")]
    pub service_delivery: ServiceDelivery,
}

#[derive(Debug, Deserialize)]
struct ServiceDelivery {
    #[serde(rename = "StopMonitoringDelivery")]
    pub stop_monitoring_delivery: Vec<StopMonitoringDelivery>,
}

#[derive(Debug, Deserialize)]
struct StopMonitoringDelivery {
    #[serde(rename = "MonitoredStopVisit")]
    pub monitored_stop_visit: Vec<MonitoredStopVisit>,
}

#[derive(Debug, Deserialize)]
struct MonitoredStopVisit {
    #[serde(rename = "MonitoredVehicleJourney")]
    pub monitored_vehicle_journey: MonitoredVehicleJourney,
}

#[derive(Debug, Deserialize)]
struct MonitoredVehicleJourney {
    #[serde(rename = "MonitoredCall")]
    pub monitored_call: MonitoredCall,
}

#[derive(Debug, Deserialize)]
struct MonitoredCall {
    #[serde(rename = "ExpectedArrivalTime")]
    pub expected_arrival_time: DateTime<Utc>,
}

/// PRIM API URL to get next subway arrivals
const API_URL: &str = "https://prim.iledefrance-mobilites.fr/marketplace/stop-monitoring";
/// Header name to associate with the API token.
const API_KEY_HEADER_NAME: &str = "apiKey";
/// Header name to associate with the line reference.
const LINE_REF_QUERY_NAME: &str = "LineRef";
/// Header name to associate with the monitoring reference.
const MONITORING_REF_QUERY_NAME: &str = "MonitoringRef";

pub struct PrimApiManager {
    api_token: String,
    line_ref: String,
    monitoring_ref: String,
}

impl PrimApiManager {
    pub const fn new(api_token: String, line_ref: String, monitoring_ref: String) -> Self {
        Self {
            api_token,
            line_ref,
            monitoring_ref,
        }
    }

    pub fn get_next_subway_arrivals(&self) -> Result<ApiInfo> {
        let response = ureq::get(API_URL)
            .query(MONITORING_REF_QUERY_NAME, &self.monitoring_ref)
            .query(LINE_REF_QUERY_NAME, &self.line_ref)
            .header(API_KEY_HEADER_NAME, &self.api_token)
            .call()?;

        let body = response
            .into_body()
            .read_json::<NextSubwayArrivalsResponse>()?;

        let mut offsets_minutes = body
            .into_next_arrivals()
            .into_iter()
            .map(|t| (t - Utc::now()).num_minutes());

        let first = offsets_minutes.next().unwrap_or(NEXT_SUBWAY_ERROR_VALUE);
        let second = offsets_minutes.next().unwrap_or(NEXT_SUBWAY_ERROR_VALUE);

        Ok(ApiInfo {
            next_subway_interval_mins: u8::try_from(first)?,
            second_subway_interval_mins: u8::try_from(second)?,
        })
    }
}
