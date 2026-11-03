//! Experimental bounded feature parsers, not session negotiation or admission.
use serde::{Deserialize, Deserializer};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureFormatError(pub &'static str);
impl std::fmt::Display for FeatureFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}
impl std::error::Error for FeatureFormatError {}
const NATIVE_INVALID: FeatureFormatError = FeatureFormatError("NCP-FEATURE-NEGOTIATION-INVALID");
const HTTP_INVALID: FeatureFormatError = FeatureFormatError("NWP-FEATURE-REQUIREMENTS-INVALID");

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FeatureRevision {
    pub revision: u16,
    pub lifecycle: String,
}
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct FeatureSelection {
    pub id: String,
    pub revision: u16,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureOffer {
    features: BTreeMap<String, Vec<FeatureRevision>>,
    required: Vec<String>,
}
impl FeatureOffer {
    pub fn features(&self) -> &BTreeMap<String, Vec<FeatureRevision>> {
        &self.features
    }
    pub fn required(&self) -> &[String] {
        &self.required
    }
}
// Missing and explicit null are different. Serde Option alone would accept null.
fn present<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(deserializer).map(Some)
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WireOffer {
    #[serde(default, deserialize_with = "present")]
    feature_negotiation: Option<u16>,
    #[serde(default, deserialize_with = "present")]
    feature_offer: Option<Vec<WireFeature>>,
    #[serde(default, deserialize_with = "present")]
    required_features: Option<Vec<String>>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WireFeature {
    id: String,
    revisions: Vec<FeatureRevision>,
}

fn valid_id(id: &str) -> bool {
    if id.len() > 96 {
        return false;
    }
    let segments: Vec<_> = id.split('.').collect();
    segments.len() >= 2
        && segments.iter().enumerate().all(|(index, segment)| {
            let mut bytes = segment.bytes();
            matches!(bytes.next(), Some(b'a'..=b'z'))
                && bytes.all(|b| {
                    b.is_ascii_lowercase()
                        || b.is_ascii_digit()
                        || (index > 0 && (b == b'_' || b == b'-'))
                })
        })
}

/// Standalone feature block only. Full Hello carrier size/selection is not verified.
pub fn parse_feature_offer(raw: &str) -> Result<FeatureOffer, FeatureFormatError> {
    if raw.len() > 16384 {
        return Err(NATIVE_INVALID);
    }
    // Deserialize directly into strict structs: going through Value first would erase
    // duplicate members. u16 rejects fractions/exponents/bools and overflow.
    let wire: WireOffer = serde_json::from_str(raw).map_err(|_| NATIVE_INVALID)?;
    if wire.feature_negotiation != Some(1)
        && (wire.feature_negotiation.is_some()
            || wire.feature_offer.is_some()
            || wire.required_features.is_some())
    {
        return Err(NATIVE_INVALID);
    }
    let offers = wire.feature_offer.unwrap_or_default();
    let required = wire.required_features.unwrap_or_default();
    if offers.len() > 32 || required.len() > 32 {
        return Err(NATIVE_INVALID);
    }
    let mut features = BTreeMap::new();
    for item in offers {
        if !valid_id(&item.id)
            || features.contains_key(&item.id)
            || item.revisions.is_empty()
            || item.revisions.len() > 16
        {
            return Err(NATIVE_INVALID);
        }
        let mut last = 0;
        for revision in &item.revisions {
            if revision.revision <= last
                || !matches!(
                    revision.lifecycle.as_str(),
                    "experimental" | "stable" | "deprecated"
                )
            {
                return Err(NATIVE_INVALID);
            }
            last = revision.revision;
        }
        features.insert(item.id, item.revisions);
    }
    let mut seen = BTreeSet::new();
    for id in &required {
        if !valid_id(id) || !features.contains_key(id) || !seen.insert(id) {
            return Err(NATIVE_INVALID);
        }
    }
    Ok(FeatureOffer { features, required })
}

/// Raw header instances, before framework joining. Absent differs from empty.
pub fn parse_required_feature_header(
    instances: &[&str],
) -> Result<Vec<FeatureSelection>, FeatureFormatError> {
    if instances.is_empty() {
        return Ok(Vec::new());
    }
    if instances.len() != 1 {
        return Err(HTTP_INVALID);
    }
    let raw = instances[0];
    if raw.len() > 4096 || !raw.is_ascii() {
        return Err(HTTP_INVALID);
    }
    let pairs: Vec<_> = raw.split(',').collect();
    if pairs.len() > 32 {
        return Err(HTTP_INVALID);
    }
    let mut selected = Vec::new();
    let mut last = "";
    for pair in pairs {
        let (id, digits) = pair.split_once('=').ok_or(HTTP_INVALID)?;
        if !valid_id(id)
            || id <= last
            || digits.is_empty()
            || digits.len() > 5
            || digits.starts_with('0')
            || !digits.bytes().all(|b| b.is_ascii_digit())
        {
            return Err(HTTP_INVALID);
        }
        let revision = digits.parse::<u16>().map_err(|_| HTTP_INVALID)?;
        selected.push(FeatureSelection {
            id: id.to_owned(),
            revision,
        });
        last = id;
    }
    Ok(selected)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    #[test]
    fn shared_feature_corpus() {
        let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .map(|root| root.join("spec/conformance/alpha20/feature_negotiation_vectors.json"))
            .find(|path| path.is_file())
            .expect("shared feature corpus must be distributed with the SDK");
        let raw = std::fs::read_to_string(fixture).unwrap();
        let corpus: Value = serde_json::from_str(&raw).unwrap();
        let cases = corpus["cases"].as_array().unwrap();
        assert_eq!(cases.len(), 44);
        for vector in cases {
            let id = vector["id"].as_str().unwrap();
            let error = vector["expected"]["error"].as_str();
            if vector["kind"] == "native-offer" {
                let raw = vector["raw"].as_str().unwrap();
                let parsed = parse_feature_offer(raw);
                if error == Some(NATIVE_INVALID.0) {
                    assert_eq!(parsed.unwrap_err(), NATIVE_INVALID, "{id}");
                } else {
                    let parsed = parsed.unwrap();
                    let wire: WireOffer = serde_json::from_str(raw).unwrap();
                    let expected: BTreeMap<_, _> = wire
                        .feature_offer
                        .unwrap_or_default()
                        .into_iter()
                        .map(|v| (v.id, v.revisions))
                        .collect();
                    assert_eq!(parsed.features(), &expected, "{id}");
                    assert_eq!(
                        parsed.required(),
                        wire.required_features.unwrap_or_default(),
                        "{id}"
                    );
                }
            } else {
                let headers: Vec<_> = vector["headers"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_str().unwrap())
                    .collect();
                let parsed = parse_required_feature_header(&headers);
                if error == Some(HTTP_INVALID.0) {
                    assert_eq!(parsed.unwrap_err(), HTTP_INVALID, "{id}");
                } else {
                    let expected: Vec<FeatureSelection> =
                        serde_json::from_value(vector["expected"]["required"].clone()).unwrap();
                    assert_eq!(parsed.unwrap(), expected, "{id}");
                }
            }
        }
    }
    #[test]
    fn raw_regressions() {
        for token in ["1.0", "1e0", "-1", "null", "true"] {
            assert!(parse_feature_offer(&format!("{{\"feature_negotiation\":{token}}}")).is_err());
        }
        for raw in [
            r#"{"feature_negotiation":1,"feature_\u006eegotiation":1}"#,
            r#"{"feature_offer":[]}"#,
            r#"{"required_features":[]}"#,
            r#"{"feature_negotiation":1,"feature_offer":null}"#,
            r#"{"feature_negotiation":1,"required_features":["example.echo\ud800"]}"#,
        ] {
            assert!(parse_feature_offer(raw).is_err(), "{raw}");
        }
        for raw in [
            "example.echo=1\n",
            "example.echo\r=1",
            "example.echo=1\u{2028}",
        ] {
            assert!(parse_required_feature_header(&[raw]).is_err());
        }
    }
}
