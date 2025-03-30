use chrono::{TimeZone, Utc};
use hmac::{Hmac, Mac};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};

use crate::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct AsrResult {
    pub result: String,
    pub word_size: u64,
    pub word_list: Vec<String>,
}

type HmacSha256 = Hmac<Sha256>;

fn sha256hex(s: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(s.as_bytes());
    hex::encode(hasher.finalize())
}

fn hmacsha256(message: &str, key: &[u8]) -> Vec<u8> {
    let mut mac = HmacSha256::new_from_slice(key).unwrap();
    mac.update(message.as_bytes());
    mac.finalize().into_bytes().to_vec()
}

pub async fn asr(secret_id: &str, secret_key: &str, data: &str) -> Result<AsrResult> {
    let service = "asr";
    let version = "2019-06-14";
    let action = "SentenceRecognition";
    let region = "";

    // 实例化一个认证对象，入参需要传入腾讯云账户 SecretId 和 SecretKey
    let token = "";
    let host = "asr.tencentcloudapi.com";
    let algorithm = "TC3-HMAC-SHA256";
    let timestamp = Utc::now().timestamp();

    // ************* 步骤 1：拼接规范请求串 *************
    let http_request_method = "POST";
    let canonical_uri = "/";
    let canonical_query_string = "";
    let content_type = "application/json; charset=utf-8";
    let canonical_headers = format!(
        "content-type:{}\nhost:{}\nx-tc-action:{}\n",
        content_type,
        host,
        action.to_lowercase()
    );
    let signed_headers = "content-type;host;x-tc-action";
    let payload = json!({
        "EngSerViceType":"16k_zh",
        "SourceType":1,
        "VoiceFormat":"wav",
        "Data":data,"WordInfo":1
    })
    .to_string();
    let hashed_request_payload = sha256hex(&payload);
    let canonical_request = format!(
        "{}\n{}\n{}\n{}\n{}\n{}",
        http_request_method,
        canonical_uri,
        canonical_query_string,
        canonical_headers,
        signed_headers,
        hashed_request_payload
    );
    // println!("{}", canonical_request);

    // ************* 步骤 2：拼接待签名字符串 *************
    let date = Utc
        .timestamp_opt(timestamp, 0)
        .unwrap()
        .format("%Y-%m-%d")
        .to_string();
    let credential_scope = format!("{}/{}/tc3_request", date, service);
    let hashed_canonical_request = sha256hex(&canonical_request);
    let string_to_sign = format!(
        "{}\n{}\n{}\n{}",
        algorithm, timestamp, credential_scope, hashed_canonical_request
    );
    // println!("{}", string_to_sign);

    // ************* 步骤 3：计算签名 *************
    let secret_date = hmacsha256(&date, format!("TC3{}", secret_key).as_bytes());
    let secret_service = hmacsha256(service, &secret_date);
    let secret_signing = hmacsha256("tc3_request", &secret_service);
    let signature = hex::encode(hmacsha256(&string_to_sign, &secret_signing));
    // println!("{}", signature);

    // ************* 步骤 4：拼接 Authorization *************
    let authorization = format!(
        "{} Credential={}/{}, SignedHeaders={}, Signature={}",
        algorithm, secret_id, credential_scope, signed_headers, signature
    );
    // println!("{}", authorization);

    // ************* 步骤 5：构造并发起请求 *************
    let url = format!("https://{}", host);
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert("Host", HeaderValue::from_str(host)?);
    headers.insert("X-TC-Action", HeaderValue::from_str(action)?);
    headers.insert("X-TC-Version", HeaderValue::from_str(version)?);
    headers.insert(
        "X-TC-Timestamp",
        HeaderValue::from_str(&timestamp.to_string())?,
    );
    headers.insert("Content-Type", HeaderValue::from_str(content_type)?);
    headers.insert("Authorization", HeaderValue::from_str(&authorization)?);

    if !region.is_empty() {
        headers.insert("X-TC-Region", HeaderValue::from_str(region)?);
    }
    if !token.is_empty() {
        headers.insert("X-TC-Token", HeaderValue::from_str(token)?);
    }

    let rsp = client
        .post(&url)
        .headers(headers)
        .body(payload.to_string())
        .send()
        .await?;

    let json: serde_json::Value = rsp.json().await?;
    let json = json["Response"].as_object().ok_or("识别失败!")?;
    if json.contains_key("Error") {
        let error = json["Error"].as_object().ok_or("无错误信息")?;
        let code = error["Code"].as_str().ok_or("无错误码")?;
        return Err(format!("识别失败! code:{code}").into());
    }
    let result = json["Result"].as_str().ok_or("无结果")?;
    let word_size = json["WordSize"].as_u64().ok_or("无结果")?;
    let word_list = json["WordList"]
        .as_array()
        .ok_or("无结果")?
        .iter()
        .filter_map(|item| item["Word"].as_str())
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    // println!("{:#?}", json);
    // println!("{:#?}", result);
    // println!("{:#?}", word_list);
    Ok(AsrResult {
        result: result.into(),
        word_size,
        word_list,
    })
}
