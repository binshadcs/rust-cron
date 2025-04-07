use chrono::Utc;
use chrono_tz::America::Los_Angeles;
use cron::Schedule;
use dotenv::dotenv;
use jsonwebtoken::{encode, Header, EncodingKey};
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::str::FromStr;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize, Serialize)]
struct SchemaResponse {
    #[serde(rename = "?column?")]
    column: String,
    mobile: String,
}

#[derive(Debug)]
struct CustomerInfo {
    name: String,
    due_date: String,
    mobile: String,
    full_data: String,
}

#[derive(Debug, Serialize)]
struct Claims {
    exp: usize,
    data: SchemaResponse,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let schema_api = env::var("SCHEMA_API").expect("SCHEMA_API must be set");
    let _webhook_api = env::var("WEBHOOK_API").expect("WEBHOOK_API must be set");
    let _jwt_token = env::var("JWT_TOKEN").expect("JWT_TOKEN must be set");
    
    println!("Job started timestamp: {}", Utc::now().to_rfc3339());
    
    // Parse cron expression
    let schedule = Schedule::from_str("* * * * *").expect("Failed to parse cron expression");
    
    loop {
        let now = Utc::now();
        let next = schedule.upcoming(Los_Angeles).next().unwrap();
        
        // Calculate the duration to sleep until the next cron job execution
        let duration_until_next = next.signed_duration_since(now);
        let sleep_duration = if duration_until_next.num_milliseconds() > 0 {
            Duration::from_millis(duration_until_next.num_milliseconds() as u64)
        } else {
            Duration::from_secs(1)
        };
        
        thread::sleep(sleep_duration);
        
        println!("Cron timestamp: {}", Utc::now().to_rfc3339());
        // fetch_schema_table(&schema_api).await;
    }
}

// async fn fetch_schema_table(schema_api: &str) {
//     let client = Client::new();
    
//     match client.get(&format!("{}/alwtsp/m_remind.php", schema_api)).send().await {
//         Ok(response) => {
//             match response.json::<Vec<SchemaResponse>>().await {
//                 Ok(user_data) => {
//                     let _result = parse_customer_data(&user_data);
//                     println!("{:?}", user_data);
//                 },
//                 Err(e) => {
//                     println!("Failed to parse response: {:?}", e);
//                 }
//             }
//         },
//         Err(error) => {
//             println!(
//                 "Schema fetching failed, time: {}, error: {:?}",
//                 Utc::now().to_rfc3339(),
//                 error
//             );
//         }
//     }
// }

// fn parse_customer_data(data: &[SchemaResponse]) -> Vec<CustomerInfo> {
//     let name_regex = Regex::new(r"Dear (.*?) Your").unwrap();
//     let date_regex = Regex::new(r"Due on (\d{2}-\d{2}-\d{4})").unwrap();
    
//     data.iter().map(|item| {
//         let name = match name_regex.captures(&item.column) {
//             Some(captures) => captures.get(1).unwrap().as_str().to_string(),
//             None => "Unknown".to_string(),
//         };
        
//         let due_date = match date_regex.captures(&item.column) {
//             Some(captures) => captures.get(1).unwrap().as_str().to_string(),
//             None => "Unknown".to_string(),
//         };
        
//         CustomerInfo {
//             name,
//             due_date,
//             mobile: item.mobile.clone(),
//             full_data: item.column.clone(),
//         }
//     }).collect()
// }

// async fn create_sign(data: SchemaResponse, jwt_token: &str) -> Option<String> {
//     let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize;
//     let claims = Claims {
//         exp: now + 60 * 60,
//         data,
//     };
    
//     match encode(
//         &Header::default(),
//         &claims,
//         &EncodingKey::from_secret(jwt_token.as_bytes()),
//     ) {
//         Ok(token) => Some(token),
//         Err(_) => None,
//     }
// }

// async fn update_to_whatsapp_api(key: &str, webhook_api: &str) -> Result<(), reqwest::Error> {
//     let client = Client::new();
    
//     let _response = client
//         .post(&format!("{}/send-reminder", webhook_api))
//         .json(&serde_json::json!({ "key": key }))
//         .send()
//         .await?;
    
//     Ok(())
// }