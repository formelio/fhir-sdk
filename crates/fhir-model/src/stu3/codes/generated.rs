//! Generated code! Take a look at the generator-crate for changing this file!
#![allow(clippy::too_many_lines)]
use std::hash::Hash;
use serde::{Serialize, Deserialize};
use super::super::types::{Coding, CodingInner, CodeableConcept, CodeableConceptInner};
#[doc = "**[AdministrativeGender](http://hl7.org/fhir/ValueSet/administrative-gender)**. The gender of a person used for administrative purposes.\n\nFHIR version: 3.0.2."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum AdministrativeGender {
    /** **female**

Female. Female */
    Female,
    /** **male**

Male. Male */
    Male,
    /** **other**

Other. Other */
    Other,
    /** **unknown**

Unknown. Unknown */
    Unknown,
}
impl ::core::str::FromStr for AdministrativeGender {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "female" => Ok(Self::Female),
            "male" => Ok(Self::Male),
            "other" => Ok(Self::Other),
            "unknown" => Ok(Self::Unknown),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for AdministrativeGender {
    fn as_ref(&self) -> &str {
        match self {
            Self::Female => "female",
            Self::Male => "male",
            Self::Other => "other",
            Self::Unknown => "unknown",
        }
    }
}
impl ::std::fmt::Debug for AdministrativeGender {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for AdministrativeGender {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for AdministrativeGender {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for AdministrativeGender {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<AdministrativeGender> for Coding {
    fn from(code: AdministrativeGender) -> Self {
        CodingInner {
            system: Some(
                "http://hl7.org/fhir/ValueSet/administrative-gender".to_owned(),
            ),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<AdministrativeGender> for CodeableConcept {
    fn from(code: AdministrativeGender) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
#[doc = "**[AllergyIntoleranceClinicalStatus](http://hl7.org/fhir/ValueSet/allergy-clinical-status)**. The clinical status of the allergy or intolerance.\n\nFHIR version: 3.0.2."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum AllergyIntoleranceClinicalStatus {
    /** **active**

Active. An active record of a risk of a reaction to the identified substance. */
    Active,
    /** **inactive**

Inactive. An inactivated record of a risk of a reaction to the identified substance. */
    Inactive,
    /** **resolved**

Resolved. A reaction to the identified substance has been clinically reassessed by testing or re-exposure and considered to be resolved. */
    Resolved,
}
impl ::core::str::FromStr for AllergyIntoleranceClinicalStatus {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "resolved" => Ok(Self::Resolved),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for AllergyIntoleranceClinicalStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Resolved => "resolved",
        }
    }
}
impl ::std::fmt::Debug for AllergyIntoleranceClinicalStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for AllergyIntoleranceClinicalStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for AllergyIntoleranceClinicalStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for AllergyIntoleranceClinicalStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<AllergyIntoleranceClinicalStatus> for Coding {
    fn from(code: AllergyIntoleranceClinicalStatus) -> Self {
        CodingInner {
            system: Some(
                "http://hl7.org/fhir/ValueSet/allergy-clinical-status".to_owned(),
            ),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<AllergyIntoleranceClinicalStatus> for CodeableConcept {
    fn from(code: AllergyIntoleranceClinicalStatus) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
#[doc = "**[AllergyIntoleranceCriticality](http://hl7.org/fhir/ValueSet/allergy-intolerance-criticality)**. Estimate of the potential clinical harm, or seriousness, of a reaction to an identified substance.\n\nFHIR version: 3.0.2."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum AllergyIntoleranceCriticality {
    /** **high**

High Risk. Worst case result of a future exposure is assessed to be life-threatening or having high potential for organ system failure. */
    High,
    /** **low**

Low Risk. Worst case result of a future exposure is not assessed to be life-threatening or having high potential for organ system failure. */
    Low,
    /** **unable-to-assess**

Unable to Assess Risk. Unable to assess the worst case result of a future exposure. */
    UnableToAssess,
}
impl ::core::str::FromStr for AllergyIntoleranceCriticality {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "high" => Ok(Self::High),
            "low" => Ok(Self::Low),
            "unable-to-assess" => Ok(Self::UnableToAssess),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for AllergyIntoleranceCriticality {
    fn as_ref(&self) -> &str {
        match self {
            Self::High => "high",
            Self::Low => "low",
            Self::UnableToAssess => "unable-to-assess",
        }
    }
}
impl ::std::fmt::Debug for AllergyIntoleranceCriticality {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for AllergyIntoleranceCriticality {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for AllergyIntoleranceCriticality {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for AllergyIntoleranceCriticality {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<AllergyIntoleranceCriticality> for Coding {
    fn from(code: AllergyIntoleranceCriticality) -> Self {
        CodingInner {
            system: Some(
                "http://hl7.org/fhir/ValueSet/allergy-intolerance-criticality".to_owned(),
            ),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<AllergyIntoleranceCriticality> for CodeableConcept {
    fn from(code: AllergyIntoleranceCriticality) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
#[doc = "**[AllergyIntoleranceSeverity](http://hl7.org/fhir/ValueSet/reaction-event-severity)**. Clinical assessment of the severity of a reaction event as a whole, potentially considering multiple different manifestations.\n\nFHIR version: 3.0.2."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum AllergyIntoleranceSeverity {
    /** **mild**

Mild. Causes mild physiological effects. */
    Mild,
    /** **moderate**

Moderate. Causes moderate physiological effects. */
    Moderate,
    /** **severe**

Severe. Causes severe physiological effects. */
    Severe,
}
impl ::core::str::FromStr for AllergyIntoleranceSeverity {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "mild" => Ok(Self::Mild),
            "moderate" => Ok(Self::Moderate),
            "severe" => Ok(Self::Severe),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for AllergyIntoleranceSeverity {
    fn as_ref(&self) -> &str {
        match self {
            Self::Mild => "mild",
            Self::Moderate => "moderate",
            Self::Severe => "severe",
        }
    }
}
impl ::std::fmt::Debug for AllergyIntoleranceSeverity {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for AllergyIntoleranceSeverity {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for AllergyIntoleranceSeverity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for AllergyIntoleranceSeverity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<AllergyIntoleranceSeverity> for Coding {
    fn from(code: AllergyIntoleranceSeverity) -> Self {
        CodingInner {
            system: Some(
                "http://hl7.org/fhir/ValueSet/reaction-event-severity".to_owned(),
            ),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<AllergyIntoleranceSeverity> for CodeableConcept {
    fn from(code: AllergyIntoleranceSeverity) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
#[doc = "**[AllergyIntoleranceVerificationStatus](http://hl7.org/fhir/ValueSet/allergy-verification-status)**. Assertion about certainty associated with a propensity, or potential risk, of a reaction to the identified substance.\n\nFHIR version: 3.0.2."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum AllergyIntoleranceVerificationStatus {
    /** **confirmed**

Confirmed. A high level of certainty about the propensity for a reaction to the identified substance, which may include clinical evidence by testing or rechallenge. */
    Confirmed,
    /** **entered-in-error**

Entered In Error. The statement was entered in error and is not valid. */
    EnteredInError,
    /** **refuted**

Refuted. A propensity for a reaction to the identified substance has been disproven with a high level of clinical certainty, which may include testing or rechallenge, and is refuted. */
    Refuted,
    /** **unconfirmed**

Unconfirmed. A low level of certainty about the propensity for a reaction to the identified substance. */
    Unconfirmed,
}
impl ::core::str::FromStr for AllergyIntoleranceVerificationStatus {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "confirmed" => Ok(Self::Confirmed),
            "entered-in-error" => Ok(Self::EnteredInError),
            "refuted" => Ok(Self::Refuted),
            "unconfirmed" => Ok(Self::Unconfirmed),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for AllergyIntoleranceVerificationStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Confirmed => "confirmed",
            Self::EnteredInError => "entered-in-error",
            Self::Refuted => "refuted",
            Self::Unconfirmed => "unconfirmed",
        }
    }
}
impl ::std::fmt::Debug for AllergyIntoleranceVerificationStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for AllergyIntoleranceVerificationStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for AllergyIntoleranceVerificationStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for AllergyIntoleranceVerificationStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<AllergyIntoleranceVerificationStatus> for Coding {
    fn from(code: AllergyIntoleranceVerificationStatus) -> Self {
        CodingInner {
            system: Some(
                "http://hl7.org/fhir/ValueSet/allergy-verification-status".to_owned(),
            ),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<AllergyIntoleranceVerificationStatus> for CodeableConcept {
    fn from(code: AllergyIntoleranceVerificationStatus) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
#[doc = "**[BundleType](http://hl7.org/fhir/ValueSet/bundle-type)**. Indicates the purpose of a bundle - how it was intended to be used.\n\nFHIR version: 3.0.2."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum BundleType {
    /** **batch**

Batch. The bundle is a transaction - intended to be processed by a server as a group of actions. */
    Batch,
    /** **batch-response**

Batch Response. The bundle is a batch response. Note that as a batch, some responses may indicate failure and others success. */
    BatchResponse,
    /** **collection**

Collection. The bundle is a set of resources collected into a single package for ease of distribution. */
    Collection,
    /** **document**

Document. The bundle is a document. The first resource is a Composition. */
    Document,
    /** **history**

History List. The bundle is a list of resources from a history interaction on a server. */
    History,
    /** **message**

Message. The bundle is a message. The first resource is a MessageHeader. */
    Message,
    /** **searchset**

Search Results. The bundle is a list of resources returned as a result of a search/query interaction, operation, or message. */
    Searchset,
    /** **transaction**

Transaction. The bundle is a transaction - intended to be processed by a server as an atomic commit. */
    Transaction,
    /** **transaction-response**

Transaction Response. The bundle is a transaction response. Because the response is a transaction response, the transaction has succeeded, and all responses are error free. */
    TransactionResponse,
}
impl ::core::str::FromStr for BundleType {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "batch" => Ok(Self::Batch),
            "batch-response" => Ok(Self::BatchResponse),
            "collection" => Ok(Self::Collection),
            "document" => Ok(Self::Document),
            "history" => Ok(Self::History),
            "message" => Ok(Self::Message),
            "searchset" => Ok(Self::Searchset),
            "transaction" => Ok(Self::Transaction),
            "transaction-response" => Ok(Self::TransactionResponse),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for BundleType {
    fn as_ref(&self) -> &str {
        match self {
            Self::Batch => "batch",
            Self::BatchResponse => "batch-response",
            Self::Collection => "collection",
            Self::Document => "document",
            Self::History => "history",
            Self::Message => "message",
            Self::Searchset => "searchset",
            Self::Transaction => "transaction",
            Self::TransactionResponse => "transaction-response",
        }
    }
}
impl ::std::fmt::Debug for BundleType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for BundleType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for BundleType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for BundleType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<BundleType> for Coding {
    fn from(code: BundleType) -> Self {
        CodingInner {
            system: Some("http://hl7.org/fhir/ValueSet/bundle-type".to_owned()),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<BundleType> for CodeableConcept {
    fn from(code: BundleType) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
#[doc = "**[CarePlanIntent](http://hl7.org/fhir/ValueSet/care-plan-intent)**. Codes indicating the degree of authority/intentionality associated with a care plan\n\nFHIR version: 3.0.2."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum CarePlanIntent {
    /** **option**

Option. The care plan represents a component or option for a RequestGroup that establishes timing, conditionality and/or other constraints among a set of requests.

Refer to [[[RequestGroup]]] for additional information on how this status is used */
    Option,
    /** **order**

Order. The care plan represents a request/demand and authorization for action */
    Order,
    /** **plan**

Plan. The care plan represents an intention to ensure something occurs without providing an authorization for others to act */
    Plan,
    /** **proposal**

Proposal. The care plan is a suggestion made by someone/something that doesn't have an intention to ensure it occurs and without providing an authorization to act */
    Proposal,
}
impl ::core::str::FromStr for CarePlanIntent {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "option" => Ok(Self::Option),
            "order" => Ok(Self::Order),
            "plan" => Ok(Self::Plan),
            "proposal" => Ok(Self::Proposal),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for CarePlanIntent {
    fn as_ref(&self) -> &str {
        match self {
            Self::Option => "option",
            Self::Order => "order",
            Self::Plan => "plan",
            Self::Proposal => "proposal",
        }
    }
}
impl ::std::fmt::Debug for CarePlanIntent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for CarePlanIntent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for CarePlanIntent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for CarePlanIntent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<CarePlanIntent> for Coding {
    fn from(code: CarePlanIntent) -> Self {
        CodingInner {
            system: Some("http://hl7.org/fhir/ValueSet/care-plan-intent".to_owned()),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<CarePlanIntent> for CodeableConcept {
    fn from(code: CarePlanIntent) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
#[doc = "**[CarePlanStatus](http://hl7.org/fhir/ValueSet/care-plan-status)**. Indicates whether the plan is currently being acted upon, represents future intentions or is now a historical record.\n\nFHIR version: 3.0.2."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum CarePlanStatus {
    /** **active**

Active. The plan is intended to be followed and used as part of patient care. */
    Active,
    /** **cancelled**

Cancelled. The plan has been terminated prior to reaching completion (though it may have been replaced by a new plan). */
    Cancelled,
    /** **completed**

Completed. The plan is no longer in use and is not expected to be followed or used in patient care. */
    Completed,
    /** **draft**

Pending. The plan is in development or awaiting use but is not yet intended to be acted upon. */
    Draft,
    /** **entered-in-error**

Entered In Error. The plan was entered in error and voided. */
    EnteredInError,
    /** **suspended**

Suspended. The plan has been temporarily stopped but is expected to resume in the future. */
    Suspended,
    /** **unknown**

Unknown. The authoring system doesn't know the current state of the care plan. */
    Unknown,
}
impl ::core::str::FromStr for CarePlanStatus {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "cancelled" => Ok(Self::Cancelled),
            "completed" => Ok(Self::Completed),
            "draft" => Ok(Self::Draft),
            "entered-in-error" => Ok(Self::EnteredInError),
            "suspended" => Ok(Self::Suspended),
            "unknown" => Ok(Self::Unknown),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for CarePlanStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Active => "active",
            Self::Cancelled => "cancelled",
            Self::Completed => "completed",
            Self::Draft => "draft",
            Self::EnteredInError => "entered-in-error",
            Self::Suspended => "suspended",
            Self::Unknown => "unknown",
        }
    }
}
impl ::std::fmt::Debug for CarePlanStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for CarePlanStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for CarePlanStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for CarePlanStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<CarePlanStatus> for Coding {
    fn from(code: CarePlanStatus) -> Self {
        CodingInner {
            system: Some("http://hl7.org/fhir/ValueSet/care-plan-status".to_owned()),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<CarePlanStatus> for CodeableConcept {
    fn from(code: CarePlanStatus) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
#[doc = "**[CareTeamStatus](http://hl7.org/fhir/ValueSet/care-team-status)**. Indicates the status of the care team.\n\nFHIR version: 3.0.2."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum CareTeamStatus {
    /** **active**

Active. The care team is currently participating in the coordination and delivery of care. */
    Active,
    /** **entered-in-error**

Entered In Error. The care team should have never existed. */
    EnteredInError,
    /** **inactive**

Inactive. The care team was, but is no longer, participating in the coordination and delivery of care. */
    Inactive,
    /** **proposed**

Proposed. The care team has been drafted and proposed, but not yet participating in the coordination and delivery of care. */
    Proposed,
    /** **suspended**

Suspended. The care team is temporarily on hold or suspended and not participating in the coordination and delivery of care. */
    Suspended,
}
impl ::core::str::FromStr for CareTeamStatus {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "entered-in-error" => Ok(Self::EnteredInError),
            "inactive" => Ok(Self::Inactive),
            "proposed" => Ok(Self::Proposed),
            "suspended" => Ok(Self::Suspended),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for CareTeamStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Active => "active",
            Self::EnteredInError => "entered-in-error",
            Self::Inactive => "inactive",
            Self::Proposed => "proposed",
            Self::Suspended => "suspended",
        }
    }
}
impl ::std::fmt::Debug for CareTeamStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for CareTeamStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for CareTeamStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for CareTeamStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<CareTeamStatus> for Coding {
    fn from(code: CareTeamStatus) -> Self {
        CodingInner {
            system: Some("http://hl7.org/fhir/ValueSet/care-team-status".to_owned()),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<CareTeamStatus> for CodeableConcept {
    fn from(code: CareTeamStatus) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
#[doc = "**[ConditionVerificationStatus](http://hl7.org/fhir/ValueSet/condition-ver-status)**. The verification status to support or decline the clinical status of the condition or diagnosis.\n\nFHIR version: 3.0.2."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum ConditionVerificationStatus {
    /** **confirmed**

Confirmed. There is sufficient diagnostic and/or clinical evidence to treat this as a confirmed condition. */
    Confirmed,
    /** **differential**

Differential. One of a set of potential (and typically mutually exclusive) diagnoses asserted to further guide the diagnostic process and preliminary treatment. */
    Differential,
    /** **entered-in-error**

Entered In Error. The statement was entered in error and is not valid. */
    EnteredInError,
    /** **provisional**

Provisional. This is a tentative diagnosis - still a candidate that is under consideration. */
    Provisional,
    /** **refuted**

Refuted. This condition has been ruled out by diagnostic and clinical evidence. */
    Refuted,
    /** **unknown**

Unknown. The condition status is unknown.  Note that "unknown" is a value of last resort and every attempt should be made to provide a meaningful value other than "unknown". */
    Unknown,
}
impl ::core::str::FromStr for ConditionVerificationStatus {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "confirmed" => Ok(Self::Confirmed),
            "differential" => Ok(Self::Differential),
            "entered-in-error" => Ok(Self::EnteredInError),
            "provisional" => Ok(Self::Provisional),
            "refuted" => Ok(Self::Refuted),
            "unknown" => Ok(Self::Unknown),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for ConditionVerificationStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Confirmed => "confirmed",
            Self::Differential => "differential",
            Self::EnteredInError => "entered-in-error",
            Self::Provisional => "provisional",
            Self::Refuted => "refuted",
            Self::Unknown => "unknown",
        }
    }
}
impl ::std::fmt::Debug for ConditionVerificationStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for ConditionVerificationStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for ConditionVerificationStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for ConditionVerificationStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<ConditionVerificationStatus> for Coding {
    fn from(code: ConditionVerificationStatus) -> Self {
        CodingInner {
            system: Some("http://hl7.org/fhir/ValueSet/condition-ver-status".to_owned()),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<ConditionVerificationStatus> for CodeableConcept {
    fn from(code: ConditionVerificationStatus) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
#[doc = "**[ContactPointSystem](http://hl7.org/fhir/ValueSet/contact-point-system)**. Telecommunications form for contact point\n\nFHIR version: 3.0.2."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum ContactPointSystem {
    /** **email**

Email. The value is an email address. */
    Email,
    /** **fax**

Fax. The value is a fax machine. Use of full international numbers starting with + is recommended to enable automatic dialing support but not required. */
    Fax,
    /** **other**

Other. A contact that is not a phone, fax, page or email address and is not expressible as a URL.  E.g. Internal mail address.  This SHOULD NOT be used for contacts that are expressible as a URL (e.g. Skype, Twitter, Facebook, etc.)  Extensions may be used to distinguish "other" contact types. */
    Other,
    /** **pager**

Pager. The value is a pager number. These may be local pager numbers that are only usable on a particular pager system. */
    Pager,
    /** **phone**

Phone. The value is a telephone number used for voice calls. Use of full international numbers starting with + is recommended to enable automatic dialing support but not required. */
    Phone,
    /** **sms**

SMS. A contact that can be used for sending an sms message (e.g. mobide phones, some landlines) */
    Sms,
    /** **url**

URL. A contact that is not a phone, fax, pager or email address and is expressed as a URL.  This is intended for various personal contacts including blogs, Skype, Twitter, Facebook, etc. Do not use for email addresses. */
    Url,
}
impl ::core::str::FromStr for ContactPointSystem {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "email" => Ok(Self::Email),
            "fax" => Ok(Self::Fax),
            "other" => Ok(Self::Other),
            "pager" => Ok(Self::Pager),
            "phone" => Ok(Self::Phone),
            "sms" => Ok(Self::Sms),
            "url" => Ok(Self::Url),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for ContactPointSystem {
    fn as_ref(&self) -> &str {
        match self {
            Self::Email => "email",
            Self::Fax => "fax",
            Self::Other => "other",
            Self::Pager => "pager",
            Self::Phone => "phone",
            Self::Sms => "sms",
            Self::Url => "url",
        }
    }
}
impl ::std::fmt::Debug for ContactPointSystem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for ContactPointSystem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for ContactPointSystem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for ContactPointSystem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<ContactPointSystem> for Coding {
    fn from(code: ContactPointSystem) -> Self {
        CodingInner {
            system: Some("http://hl7.org/fhir/ValueSet/contact-point-system".to_owned()),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<ContactPointSystem> for CodeableConcept {
    fn from(code: ContactPointSystem) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
#[doc = "**[ContactPointUse](http://hl7.org/fhir/ValueSet/contact-point-use)**. Use of contact point\n\nFHIR version: 3.0.2."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum ContactPointUse {
    /** **home**

Home. A communication contact point at a home; attempted contacts for business purposes might intrude privacy and chances are one will contact family or other household members instead of the person one wishes to call. Typically used with urgent cases, or if no other contacts are available. */
    Home,
    /** **mobile**

Mobile. A telecommunication device that moves and stays with its owner. May have characteristics of all other use codes, suitable for urgent matters, not the first choice for routine business. */
    Mobile,
    /** **old**

Old. This contact point is no longer in use (or was never correct, but retained for records). */
    Old,
    /** **temp**

Temp. A temporary contact point. The period can provide more detailed information. */
    Temp,
    /** **work**

Work. An office contact point. First choice for business related contacts during business hours. */
    Work,
}
impl ::core::str::FromStr for ContactPointUse {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "home" => Ok(Self::Home),
            "mobile" => Ok(Self::Mobile),
            "old" => Ok(Self::Old),
            "temp" => Ok(Self::Temp),
            "work" => Ok(Self::Work),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for ContactPointUse {
    fn as_ref(&self) -> &str {
        match self {
            Self::Home => "home",
            Self::Mobile => "mobile",
            Self::Old => "old",
            Self::Temp => "temp",
            Self::Work => "work",
        }
    }
}
impl ::std::fmt::Debug for ContactPointUse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for ContactPointUse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for ContactPointUse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for ContactPointUse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<ContactPointUse> for Coding {
    fn from(code: ContactPointUse) -> Self {
        CodingInner {
            system: Some("http://hl7.org/fhir/ValueSet/contact-point-use".to_owned()),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<ContactPointUse> for CodeableConcept {
    fn from(code: ContactPointUse) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
#[doc = "**[DaysOfWeek](http://hl7.org/fhir/ValueSet/days-of-week)**. The days of the week.\n\nFHIR version: 3.0.2."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum DaysOfWeek {
    /** **fri**

Friday. Friday */
    Fri,
    /** **mon**

Monday. Monday */
    Mon,
    /** **sat**

Saturday. Saturday */
    Sat,
    /** **sun**

Sunday. Sunday */
    Sun,
    /** **thu**

Thursday. Thursday */
    Thu,
    /** **tue**

Tuesday. Tuesday */
    Tue,
    /** **wed**

Wednesday. Wednesday */
    Wed,
}
impl ::core::str::FromStr for DaysOfWeek {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "fri" => Ok(Self::Fri),
            "mon" => Ok(Self::Mon),
            "sat" => Ok(Self::Sat),
            "sun" => Ok(Self::Sun),
            "thu" => Ok(Self::Thu),
            "tue" => Ok(Self::Tue),
            "wed" => Ok(Self::Wed),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for DaysOfWeek {
    fn as_ref(&self) -> &str {
        match self {
            Self::Fri => "fri",
            Self::Mon => "mon",
            Self::Sat => "sat",
            Self::Sun => "sun",
            Self::Thu => "thu",
            Self::Tue => "tue",
            Self::Wed => "wed",
        }
    }
}
impl ::std::fmt::Debug for DaysOfWeek {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for DaysOfWeek {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for DaysOfWeek {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for DaysOfWeek {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<DaysOfWeek> for Coding {
    fn from(code: DaysOfWeek) -> Self {
        CodingInner {
            system: Some("http://hl7.org/fhir/ValueSet/days-of-week".to_owned()),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<DaysOfWeek> for CodeableConcept {
    fn from(code: DaysOfWeek) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
#[doc = "**[DeviceUseStatementStatus](http://hl7.org/fhir/ValueSet/device-statement-status)**. A coded concept indicating the current status of a the Device Usage\n\nFHIR version: 3.0.2."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum DeviceUseStatementStatus {
    /** **active**

Active. The device is still being used. */
    Active,
    /** **completed**

Completed. The device is no longer being used. */
    Completed,
    /** **entered-in-error**

Entered in Error. The statement was recorded incorrectly. */
    EnteredInError,
    /** **intended**

Intended. The device may be used at some time in the future. */
    Intended,
    /** **on-hold**

On Hold. Actions implied by the statement have been temporarily halted, but are expected to continue later. May also be called "suspended". */
    OnHold,
    /** **stopped**

Stopped. Actions implied by the statement have been permanently halted, before all of them occurred. */
    Stopped,
}
impl ::core::str::FromStr for DeviceUseStatementStatus {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "completed" => Ok(Self::Completed),
            "entered-in-error" => Ok(Self::EnteredInError),
            "intended" => Ok(Self::Intended),
            "on-hold" => Ok(Self::OnHold),
            "stopped" => Ok(Self::Stopped),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for DeviceUseStatementStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Active => "active",
            Self::Completed => "completed",
            Self::EnteredInError => "entered-in-error",
            Self::Intended => "intended",
            Self::OnHold => "on-hold",
            Self::Stopped => "stopped",
        }
    }
}
impl ::std::fmt::Debug for DeviceUseStatementStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for DeviceUseStatementStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for DeviceUseStatementStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for DeviceUseStatementStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<DeviceUseStatementStatus> for Coding {
    fn from(code: DeviceUseStatementStatus) -> Self {
        CodingInner {
            system: Some(
                "http://hl7.org/fhir/ValueSet/device-statement-status".to_owned(),
            ),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<DeviceUseStatementStatus> for CodeableConcept {
    fn from(code: DeviceUseStatementStatus) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
#[doc = "**[DiagnosticReportStatus](http://hl7.org/fhir/ValueSet/diagnostic-report-status)**. The status of the diagnostic report as a whole.\n\nFHIR version: 3.0.2."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum DiagnosticReportStatus {
    /** **amended**

Amended. Subsequent to being final, the report has been modified.  This includes any change in the results, diagnosis, narrative text, or other content of a report that has been issued. */
    Amended,
    /** **appended**

Appended. Subsequent to being final, the report has been modified by adding new content. The existing content is unchanged. */
    Appended,
    /** **cancelled**

Cancelled. The report is unavailable because the measurement was not started or not completed (also sometimes called "aborted"). */
    Cancelled,
    /** **corrected**

Corrected. Subsequent to being final, the report has been modified  to correct an error in the report or referenced results. */
    Corrected,
    /** **entered-in-error**

Entered in Error. The report has been withdrawn following a previous final release.  This electronic record should never have existed, though it is possible that real-world decisions were based on it. (If real-world activity has occurred, the status should be "cancelled" rather than "entered-in-error".) */
    EnteredInError,
    /** **final**

Final. The report is complete and verified by an authorized person. */
    Final,
    /** **partial**

Partial. This is a partial (e.g. initial, interim or preliminary) report: data in the report may be incomplete or unverified. */
    Partial,
    /** **preliminary**

Preliminary. Verified early results are available, but not all  results are final. */
    Preliminary,
    /** **registered**

Registered. The existence of the report is registered, but there is nothing yet available. */
    Registered,
    /** **unknown**

Unknown. The authoring system does not know which of the status values currently applies for this request. Note: This concept is not to be used for "other" - one of the listed statuses is presumed to apply, it's just not known which one. */
    Unknown,
}
impl ::core::str::FromStr for DiagnosticReportStatus {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "amended" => Ok(Self::Amended),
            "appended" => Ok(Self::Appended),
            "cancelled" => Ok(Self::Cancelled),
            "corrected" => Ok(Self::Corrected),
            "entered-in-error" => Ok(Self::EnteredInError),
            "final" => Ok(Self::Final),
            "partial" => Ok(Self::Partial),
            "preliminary" => Ok(Self::Preliminary),
            "registered" => Ok(Self::Registered),
            "unknown" => Ok(Self::Unknown),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for DiagnosticReportStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Amended => "amended",
            Self::Appended => "appended",
            Self::Cancelled => "cancelled",
            Self::Corrected => "corrected",
            Self::EnteredInError => "entered-in-error",
            Self::Final => "final",
            Self::Partial => "partial",
            Self::Preliminary => "preliminary",
            Self::Registered => "registered",
            Self::Unknown => "unknown",
        }
    }
}
impl ::std::fmt::Debug for DiagnosticReportStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for DiagnosticReportStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for DiagnosticReportStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for DiagnosticReportStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<DiagnosticReportStatus> for Coding {
    fn from(code: DiagnosticReportStatus) -> Self {
        CodingInner {
            system: Some(
                "http://hl7.org/fhir/ValueSet/diagnostic-report-status".to_owned(),
            ),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<DiagnosticReportStatus> for CodeableConcept {
    fn from(code: DiagnosticReportStatus) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
#[doc = "**[EncounterStatus](http://hl7.org/fhir/ValueSet/encounter-status)**. Current state of the encounter\n\nFHIR version: 3.0.2."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum EncounterStatus {
    /** **arrived**

Arrived. The Patient is present for the encounter, however is not currently meeting with a practitioner. */
    Arrived,
    /** **cancelled**

Cancelled. The Encounter has ended before it has begun. */
    Cancelled,
    /** **entered-in-error**

Entered in Error. This instance should not have been part of this patient's medical record. */
    EnteredInError,
    /** **finished**

Finished. The Encounter has ended. */
    Finished,
    /** **in-progress**

In Progress. The Encounter has begun and the patient is present / the practitioner and the patient are meeting. */
    InProgress,
    /** **onleave**

On Leave. The Encounter has begun, but the patient is temporarily on leave. */
    Onleave,
    /** **planned**

Planned. The Encounter has not yet started. */
    Planned,
    /** **triaged**

Triaged. The patient has been assessed for the priority of their treatment based on the severity of their condition. */
    Triaged,
    /** **unknown**

Unknown. The encounter status is unknown. Note that "unknown" is a value of last resort and every attempt should be made to provide a meaningful value other than "unknown". */
    Unknown,
}
impl ::core::str::FromStr for EncounterStatus {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "arrived" => Ok(Self::Arrived),
            "cancelled" => Ok(Self::Cancelled),
            "entered-in-error" => Ok(Self::EnteredInError),
            "finished" => Ok(Self::Finished),
            "in-progress" => Ok(Self::InProgress),
            "onleave" => Ok(Self::Onleave),
            "planned" => Ok(Self::Planned),
            "triaged" => Ok(Self::Triaged),
            "unknown" => Ok(Self::Unknown),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for EncounterStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Arrived => "arrived",
            Self::Cancelled => "cancelled",
            Self::EnteredInError => "entered-in-error",
            Self::Finished => "finished",
            Self::InProgress => "in-progress",
            Self::Onleave => "onleave",
            Self::Planned => "planned",
            Self::Triaged => "triaged",
            Self::Unknown => "unknown",
        }
    }
}
impl ::std::fmt::Debug for EncounterStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for EncounterStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for EncounterStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for EncounterStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<EncounterStatus> for Coding {
    fn from(code: EncounterStatus) -> Self {
        CodingInner {
            system: Some("http://hl7.org/fhir/ValueSet/encounter-status".to_owned()),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<EncounterStatus> for CodeableConcept {
    fn from(code: EncounterStatus) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
#[doc = "**[EpisodeOfCareStatus](http://hl7.org/fhir/ValueSet/episode-of-care-status)**. The status of the episode of care.\n\nFHIR version: 3.0.2."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum EpisodeOfCareStatus {
    /** **active**

Active. This episode of care is current. */
    Active,
    /** **cancelled**

Cancelled. The episode of care was cancelled, or withdrawn from service, often selected during the planned stage as the patient may have gone elsewhere, or the circumstances have changed and the organization is unable to provide the care. It indicates that services terminated outside the planned/expected workflow. */
    Cancelled,
    /** **entered-in-error**

Entered in Error. This instance should not have been part of this patient's medical record. */
    EnteredInError,
    /** **finished**

Finished. This episode of care is finished and the organization is not expecting to be providing further care to the patient. Can also be known as "closed", "completed" or other similar terms. */
    Finished,
    /** **onhold**

On Hold. This episode of care is on hold, the organization has limited responsibility for the patient (such as while on respite). */
    Onhold,
    /** **planned**

Planned. This episode of care is planned to start at the date specified in the period.start. During this status, an organization may perform assessments to determine if the patient is eligible to receive services, or be organizing to make resources available to provide care services. */
    Planned,
    /** **waitlist**

Waitlist. This episode has been placed on a waitlist, pending the episode being made active (or cancelled). */
    Waitlist,
}
impl ::core::str::FromStr for EpisodeOfCareStatus {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "cancelled" => Ok(Self::Cancelled),
            "entered-in-error" => Ok(Self::EnteredInError),
            "finished" => Ok(Self::Finished),
            "onhold" => Ok(Self::Onhold),
            "planned" => Ok(Self::Planned),
            "waitlist" => Ok(Self::Waitlist),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for EpisodeOfCareStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Active => "active",
            Self::Cancelled => "cancelled",
            Self::EnteredInError => "entered-in-error",
            Self::Finished => "finished",
            Self::Onhold => "onhold",
            Self::Planned => "planned",
            Self::Waitlist => "waitlist",
        }
    }
}
impl ::std::fmt::Debug for EpisodeOfCareStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for EpisodeOfCareStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for EpisodeOfCareStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for EpisodeOfCareStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<EpisodeOfCareStatus> for Coding {
    fn from(code: EpisodeOfCareStatus) -> Self {
        CodingInner {
            system: Some(
                "http://hl7.org/fhir/ValueSet/episode-of-care-status".to_owned(),
            ),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<EpisodeOfCareStatus> for CodeableConcept {
    fn from(code: EpisodeOfCareStatus) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
#[doc = "**[EventStatus](http://hl7.org/fhir/ValueSet/event-status)**. Codes identifying the stage lifecycle stage of a event\n\nFHIR version: 3.0.2."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum EventStatus {
    /** **aborted**

Aborted. The event was  prior to the full completion of the intended actions */
    Aborted,
    /** **completed**

Completed. The event has now concluded */
    Completed,
    /** **entered-in-error**

Entered in Error. This electronic record should never have existed, though it is possible that real-world decisions were based on it.  (If real-world activity has occurred, the status should be "cancelled" rather than "entered-in-error".) */
    EnteredInError,
    /** **in-progress**

In Progress. The event is currently occurring */
    InProgress,
    /** **preparation**

Preparation. The core event has not started yet, but some staging activities have begun (e.g. surgical suite preparation).  Preparation stages may be tracked for billing purposes. */
    Preparation,
    /** **suspended**

Suspended. The event has been temporarily stopped but is expected to resume in the future */
    Suspended,
    /** **unknown**

Unknown. The authoring system does not know which of the status values currently applies for this request.  Note: This concept is not to be used for "other" - one of the listed statuses is presumed to apply, it's just not known which one. */
    Unknown,
}
impl ::core::str::FromStr for EventStatus {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "aborted" => Ok(Self::Aborted),
            "completed" => Ok(Self::Completed),
            "entered-in-error" => Ok(Self::EnteredInError),
            "in-progress" => Ok(Self::InProgress),
            "preparation" => Ok(Self::Preparation),
            "suspended" => Ok(Self::Suspended),
            "unknown" => Ok(Self::Unknown),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for EventStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Aborted => "aborted",
            Self::Completed => "completed",
            Self::EnteredInError => "entered-in-error",
            Self::InProgress => "in-progress",
            Self::Preparation => "preparation",
            Self::Suspended => "suspended",
            Self::Unknown => "unknown",
        }
    }
}
impl ::std::fmt::Debug for EventStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for EventStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for EventStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for EventStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<EventStatus> for Coding {
    fn from(code: EventStatus) -> Self {
        CodingInner {
            system: Some("http://hl7.org/fhir/ValueSet/event-status".to_owned()),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<EventStatus> for CodeableConcept {
    fn from(code: EventStatus) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
#[doc = "**[FlagStatus](http://hl7.org/fhir/ValueSet/flag-status)**. Indicates whether this flag is active and needs to be displayed to a user, or whether it is no longer needed or entered in error.\n\nFHIR version: 3.0.2."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum FlagStatus {
    /** **active**

Active. A current flag that should be displayed to a user. A system may use the category to determine which roles should view the flag. */
    Active,
    /** **entered-in-error**

Entered in Error. The flag was added in error, and should no longer be displayed. */
    EnteredInError,
    /** **inactive**

Inactive. The flag does not need to be displayed any more. */
    Inactive,
}
impl ::core::str::FromStr for FlagStatus {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "entered-in-error" => Ok(Self::EnteredInError),
            "inactive" => Ok(Self::Inactive),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for FlagStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Active => "active",
            Self::EnteredInError => "entered-in-error",
            Self::Inactive => "inactive",
        }
    }
}
impl ::std::fmt::Debug for FlagStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for FlagStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for FlagStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for FlagStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<FlagStatus> for Coding {
    fn from(code: FlagStatus) -> Self {
        CodingInner {
            system: Some("http://hl7.org/fhir/ValueSet/flag-status".to_owned()),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<FlagStatus> for CodeableConcept {
    fn from(code: FlagStatus) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
#[doc = "**[HTTPVerb](http://hl7.org/fhir/ValueSet/http-verb)**. HTTP verbs (in the HTTP command line).\n\nFHIR version: 3.0.2."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum HTTPVerb {
    /** **DELETE**

DELETE. HTTP DELETE */
    Delete,
    /** **GET**

GET. HTTP GET */
    Get,
    /** **POST**

POST. HTTP POST */
    Post,
    /** **PUT**

PUT. HTTP PUT */
    Put,
}
impl ::core::str::FromStr for HTTPVerb {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "DELETE" => Ok(Self::Delete),
            "GET" => Ok(Self::Get),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for HTTPVerb {
    fn as_ref(&self) -> &str {
        match self {
            Self::Delete => "DELETE",
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Put => "PUT",
        }
    }
}
impl ::std::fmt::Debug for HTTPVerb {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for HTTPVerb {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for HTTPVerb {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for HTTPVerb {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<HTTPVerb> for Coding {
    fn from(code: HTTPVerb) -> Self {
        CodingInner {
            system: Some("http://hl7.org/fhir/ValueSet/http-verb".to_owned()),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<HTTPVerb> for CodeableConcept {
    fn from(code: HTTPVerb) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
#[doc = "**[NameUse](http://hl7.org/fhir/ValueSet/name-use)**. The use of a human name\n\nFHIR version: 3.0.2."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum NameUse {
    /** **anonymous**

Anonymous. Anonymous assigned name, alias, or pseudonym (used to protect a person's identity for privacy reasons) */
    Anonymous,
    /** **maiden**

Name changed for Marriage. A name used prior to changing name because of marriage. This name use is for use by applications that collect and store names that were used prior to a marriage. Marriage naming customs vary greatly around the world, and are constantly changing. This term is not gender specific. The use of this term does not imply any particular history for a person's name */
    Maiden,
    /** **nickname**

Nickname. A name that is used to address the person in an informal manner, but is not part of their formal or usual name */
    Nickname,
    /** **official**

Official. The formal name as registered in an official (government) registry, but which name might not be commonly used. May be called "legal name". */
    Official,
    /** **old**

Old. This name is no longer in use (or was never correct, but retained for records) */
    Old,
    /** **temp**

Temp. A temporary name. Name.period can provide more detailed information. This may also be used for temporary names assigned at birth or in emergency situations. */
    Temp,
    /** **usual**

Usual. Known as/conventional/the one you normally use */
    Usual,
}
impl ::core::str::FromStr for NameUse {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "anonymous" => Ok(Self::Anonymous),
            "maiden" => Ok(Self::Maiden),
            "nickname" => Ok(Self::Nickname),
            "official" => Ok(Self::Official),
            "old" => Ok(Self::Old),
            "temp" => Ok(Self::Temp),
            "usual" => Ok(Self::Usual),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for NameUse {
    fn as_ref(&self) -> &str {
        match self {
            Self::Anonymous => "anonymous",
            Self::Maiden => "maiden",
            Self::Nickname => "nickname",
            Self::Official => "official",
            Self::Old => "old",
            Self::Temp => "temp",
            Self::Usual => "usual",
        }
    }
}
impl ::std::fmt::Debug for NameUse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for NameUse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for NameUse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for NameUse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<NameUse> for Coding {
    fn from(code: NameUse) -> Self {
        CodingInner {
            system: Some("http://hl7.org/fhir/ValueSet/name-use".to_owned()),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<NameUse> for CodeableConcept {
    fn from(code: NameUse) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
#[doc = "**[ObservationStatus](http://hl7.org/fhir/ValueSet/observation-status)**. Codes providing the status of an observation.\n\nFHIR version: 3.0.2."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum ObservationStatus {
    /** **amended**

Amended. Subsequent to being Final, the observation has been modified subsequent.  This includes updates/new information and corrections. */
    Amended,
    /** **cancelled**

Cancelled. The observation is unavailable because the measurement was not started or not completed (also sometimes called "aborted"). */
    Cancelled,
    /** **corrected**

Corrected. Subsequent to being Final, the observation has been modified to correct an error in the test result. */
    Corrected,
    /** **entered-in-error**

Entered in Error. The observation has been withdrawn following previous final release.  This electronic record should never have existed, though it is possible that real-world decisions were based on it. (If real-world activity has occurred, the status should be "cancelled" rather than "entered-in-error".) */
    EnteredInError,
    /** **final**

Final. The observation is complete. */
    Final,
    /** **preliminary**

Preliminary. This is an initial or interim observation: data may be incomplete or unverified. */
    Preliminary,
    /** **registered**

Registered. The existence of the observation is registered, but there is no result yet available. */
    Registered,
    /** **unknown**

Unknown. The authoring system does not know which of the status values currently applies for this request. Note: This concept is not to be used for "other" - one of the listed statuses is presumed to apply, but the authoring system does not know which. */
    Unknown,
}
impl ::core::str::FromStr for ObservationStatus {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "amended" => Ok(Self::Amended),
            "cancelled" => Ok(Self::Cancelled),
            "corrected" => Ok(Self::Corrected),
            "entered-in-error" => Ok(Self::EnteredInError),
            "final" => Ok(Self::Final),
            "preliminary" => Ok(Self::Preliminary),
            "registered" => Ok(Self::Registered),
            "unknown" => Ok(Self::Unknown),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for ObservationStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Amended => "amended",
            Self::Cancelled => "cancelled",
            Self::Corrected => "corrected",
            Self::EnteredInError => "entered-in-error",
            Self::Final => "final",
            Self::Preliminary => "preliminary",
            Self::Registered => "registered",
            Self::Unknown => "unknown",
        }
    }
}
impl ::std::fmt::Debug for ObservationStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for ObservationStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for ObservationStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for ObservationStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<ObservationStatus> for Coding {
    fn from(code: ObservationStatus) -> Self {
        CodingInner {
            system: Some("http://hl7.org/fhir/ValueSet/observation-status".to_owned()),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<ObservationStatus> for CodeableConcept {
    fn from(code: ObservationStatus) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
#[doc = "**[ParticipationStatus](http://hl7.org/fhir/ValueSet/participationstatus)**. The Participation status of an appointment.\n\nFHIR version: 3.0.2."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum ParticipationStatus {
    /** **accepted**

Accepted. The participant has accepted the appointment. */
    Accepted,
    /** **declined**

Declined. The participant has declined the appointment and will not participate in the appointment. */
    Declined,
    /** **needs-action**

Needs Action. The participant needs to indicate if they accept the appointment by changing this status to one of the other statuses. */
    NeedsAction,
    /** **tentative**

Tentative. The participant has  tentatively accepted the appointment. This could be automatically created by a system and requires further processing before it can be accepted. There is no commitment that attendance will occur. */
    Tentative,
}
impl ::core::str::FromStr for ParticipationStatus {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "accepted" => Ok(Self::Accepted),
            "declined" => Ok(Self::Declined),
            "needs-action" => Ok(Self::NeedsAction),
            "tentative" => Ok(Self::Tentative),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for ParticipationStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Accepted => "accepted",
            Self::Declined => "declined",
            Self::NeedsAction => "needs-action",
            Self::Tentative => "tentative",
        }
    }
}
impl ::std::fmt::Debug for ParticipationStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for ParticipationStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for ParticipationStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for ParticipationStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<ParticipationStatus> for Coding {
    fn from(code: ParticipationStatus) -> Self {
        CodingInner {
            system: Some("http://hl7.org/fhir/ValueSet/participationstatus".to_owned()),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<ParticipationStatus> for CodeableConcept {
    fn from(code: ParticipationStatus) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
#[doc = "**[QuestionnaireResponseStatus](http://hl7.org/fhir/ValueSet/questionnaire-answers-status)**. Lifecycle status of the questionnaire response.\n\nFHIR version: 3.0.2."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum QuestionnaireResponseStatus {
    /** **amended**

Amended. This QuestionnaireResponse has been filled out with answers, then marked as complete, yet changes or additions have been made to it afterwards. */
    Amended,
    /** **completed**

Completed. This QuestionnaireResponse has been filled out with answers, and the current content is regarded as definitive. */
    Completed,
    /** **entered-in-error**

Entered in Error. This QuestionnaireResponse was entered in error and voided. */
    EnteredInError,
    /** **in-progress**

In Progress. This QuestionnaireResponse has been partially filled out with answers, but changes or additions are still expected to be made to it. */
    InProgress,
    /** **stopped**

Stopped. This QuestionnaireResponse has been partially filled out with answers, but has been abandoned. It is unknown whether changes or additions are expected to be made to it. */
    Stopped,
}
impl ::core::str::FromStr for QuestionnaireResponseStatus {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "amended" => Ok(Self::Amended),
            "completed" => Ok(Self::Completed),
            "entered-in-error" => Ok(Self::EnteredInError),
            "in-progress" => Ok(Self::InProgress),
            "stopped" => Ok(Self::Stopped),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for QuestionnaireResponseStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Amended => "amended",
            Self::Completed => "completed",
            Self::EnteredInError => "entered-in-error",
            Self::InProgress => "in-progress",
            Self::Stopped => "stopped",
        }
    }
}
impl ::std::fmt::Debug for QuestionnaireResponseStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for QuestionnaireResponseStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for QuestionnaireResponseStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for QuestionnaireResponseStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<QuestionnaireResponseStatus> for Coding {
    fn from(code: QuestionnaireResponseStatus) -> Self {
        CodingInner {
            system: Some(
                "http://hl7.org/fhir/ValueSet/questionnaire-answers-status".to_owned(),
            ),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<QuestionnaireResponseStatus> for CodeableConcept {
    fn from(code: QuestionnaireResponseStatus) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
#[doc = "**[RequestPriority](http://hl7.org/fhir/ValueSet/request-priority)**. Identifies the level of importance to be assigned to actioning the request\n\nFHIR version: 3.0.2."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum RequestPriority {
    /** **asap**

ASAP. The request should be actioned as soon as possible - higher priority than urgent */
    Asap,
    /** **routine**

Routine. The request has normal priority */
    Routine,
    /** **stat**

STAT. The request should be actioned immediately - highest possible priority.  E.g. an emergency */
    Stat,
    /** **urgent**

Urgent. The request should be actioned promptly - higher priority than routine */
    Urgent,
}
impl ::core::str::FromStr for RequestPriority {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "asap" => Ok(Self::Asap),
            "routine" => Ok(Self::Routine),
            "stat" => Ok(Self::Stat),
            "urgent" => Ok(Self::Urgent),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for RequestPriority {
    fn as_ref(&self) -> &str {
        match self {
            Self::Asap => "asap",
            Self::Routine => "routine",
            Self::Stat => "stat",
            Self::Urgent => "urgent",
        }
    }
}
impl ::std::fmt::Debug for RequestPriority {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for RequestPriority {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for RequestPriority {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for RequestPriority {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<RequestPriority> for Coding {
    fn from(code: RequestPriority) -> Self {
        CodingInner {
            system: Some("http://hl7.org/fhir/ValueSet/request-priority".to_owned()),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<RequestPriority> for CodeableConcept {
    fn from(code: RequestPriority) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
#[doc = "**[SearchComparator](http://hl7.org/fhir/ValueSet/search-comparator)**. What Search Comparator Codes are supported in search\n\nFHIR version: 3.0.2."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum SearchComparator {
    /** **ap**

Approximately. the value for the parameter in the resource is approximately the same to the provided value. */
    Ap,
    /** **eb**

Ends Before. the value for the parameter in the resource ends before the provided value */
    Eb,
    /** **eq**

Equals. the value for the parameter in the resource is equal to the provided value */
    Eq,
    /** **ge**

Greater or Equals. the value for the parameter in the resource is greater or equal to the provided value */
    Ge,
    /** **gt**

Greater Than. the value for the parameter in the resource is greater than the provided value */
    Gt,
    /** **le**

Less of Equal. the value for the parameter in the resource is less or equal to the provided value */
    Le,
    /** **lt**

Less Then. the value for the parameter in the resource is less than the provided value */
    Lt,
    /** **ne**

Not Equals. the value for the parameter in the resource is not equal to the provided value */
    Ne,
    /** **sa**

Starts After. the value for the parameter in the resource starts after the provided value */
    Sa,
}
impl ::core::str::FromStr for SearchComparator {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "ap" => Ok(Self::Ap),
            "eb" => Ok(Self::Eb),
            "eq" => Ok(Self::Eq),
            "ge" => Ok(Self::Ge),
            "gt" => Ok(Self::Gt),
            "le" => Ok(Self::Le),
            "lt" => Ok(Self::Lt),
            "ne" => Ok(Self::Ne),
            "sa" => Ok(Self::Sa),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for SearchComparator {
    fn as_ref(&self) -> &str {
        match self {
            Self::Ap => "ap",
            Self::Eb => "eb",
            Self::Eq => "eq",
            Self::Ge => "ge",
            Self::Gt => "gt",
            Self::Le => "le",
            Self::Lt => "lt",
            Self::Ne => "ne",
            Self::Sa => "sa",
        }
    }
}
impl ::std::fmt::Debug for SearchComparator {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for SearchComparator {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for SearchComparator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for SearchComparator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<SearchComparator> for Coding {
    fn from(code: SearchComparator) -> Self {
        CodingInner {
            system: Some("http://hl7.org/fhir/ValueSet/search-comparator".to_owned()),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<SearchComparator> for CodeableConcept {
    fn from(code: SearchComparator) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
#[doc = "**[SearchEntryMode](http://hl7.org/fhir/ValueSet/search-entry-mode)**. Why an entry is in the result set - whether it's included as a match or because of an _include requirement.\n\nFHIR version: 3.0.2."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum SearchEntryMode {
    /** **include**

Include. This resource is returned because it is referred to from another resource in the search set. */
    Include,
    /** **match**

Match. This resource matched the search specification. */
    Match,
    /** **outcome**

Outcome. An OperationOutcome that provides additional information about the processing of a search. */
    Outcome,
}
impl ::core::str::FromStr for SearchEntryMode {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "include" => Ok(Self::Include),
            "match" => Ok(Self::Match),
            "outcome" => Ok(Self::Outcome),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for SearchEntryMode {
    fn as_ref(&self) -> &str {
        match self {
            Self::Include => "include",
            Self::Match => "match",
            Self::Outcome => "outcome",
        }
    }
}
impl ::std::fmt::Debug for SearchEntryMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for SearchEntryMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for SearchEntryMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for SearchEntryMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<SearchEntryMode> for Coding {
    fn from(code: SearchEntryMode) -> Self {
        CodingInner {
            system: Some("http://hl7.org/fhir/ValueSet/search-entry-mode".to_owned()),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<SearchEntryMode> for CodeableConcept {
    fn from(code: SearchEntryMode) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
#[doc = "**[SearchModifierCode](http://hl7.org/fhir/ValueSet/search-modifier-code)**. A supported modifier for a search parameter.\n\nFHIR version: 3.0.2."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum SearchModifierCode {
    /** **above**

Above. The search parameter tests whether the value in a resource subsumes the specified value (is-a, or hierarchical relationships). */
    Above,
    /** **below**

Below. The search parameter tests whether the value in a resource is subsumed by the specified value (is-a, or hierarchical relationships). */
    Below,
    /** **contains**

Contains. The search parameter returns resources that include the supplied parameter value anywhere within the field being searched. */
    Contains,
    /** **exact**

Exact. The search parameter returns resources that have a value that exactly matches the supplied parameter (the whole string, including casing and accents). */
    Exact,
    /** **in**

In. The search parameter is a URI (relative or absolute) that identifies a value set, and the search parameter tests whether the coding is in the specified value set. */
    In,
    /** **missing**

Missing. The search parameter returns resources that have a value or not. */
    Missing,
    /** **not**

Not. The search parameter returns resources that do not contain a match. */
    Not,
    /** **not-in**

Not In. The search parameter is a URI (relative or absolute) that identifies a value set, and the search parameter tests whether the coding is not in the specified value set. */
    NotIn,
    /** **text**

Text. The search parameter is processed as a string that searches text associated with the code/value - either CodeableConcept.text, Coding.display, or Identifier.type.text. */
    Text,
    /** **type**

Type. The search parameter only applies to the Resource Type specified as a modifier (e.g. the modifier is not actually :type, but :Patient etc.). */
    Type,
}
impl ::core::str::FromStr for SearchModifierCode {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "above" => Ok(Self::Above),
            "below" => Ok(Self::Below),
            "contains" => Ok(Self::Contains),
            "exact" => Ok(Self::Exact),
            "in" => Ok(Self::In),
            "missing" => Ok(Self::Missing),
            "not" => Ok(Self::Not),
            "not-in" => Ok(Self::NotIn),
            "text" => Ok(Self::Text),
            "type" => Ok(Self::Type),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for SearchModifierCode {
    fn as_ref(&self) -> &str {
        match self {
            Self::Above => "above",
            Self::Below => "below",
            Self::Contains => "contains",
            Self::Exact => "exact",
            Self::In => "in",
            Self::Missing => "missing",
            Self::Not => "not",
            Self::NotIn => "not-in",
            Self::Text => "text",
            Self::Type => "type",
        }
    }
}
impl ::std::fmt::Debug for SearchModifierCode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for SearchModifierCode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for SearchModifierCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for SearchModifierCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<SearchModifierCode> for Coding {
    fn from(code: SearchModifierCode) -> Self {
        CodingInner {
            system: Some("http://hl7.org/fhir/ValueSet/search-modifier-code".to_owned()),
            code: Some(code.as_ref().to_owned()),
            display: Some(format!("{code}")),
            id: None,
            extension: Vec::new(),
            system_ext: None,
            version: None,
            version_ext: None,
            code_ext: None,
            display_ext: None,
            user_selected: None,
            user_selected_ext: None,
        }
            .into()
    }
}
impl From<SearchModifierCode> for CodeableConcept {
    fn from(code: SearchModifierCode) -> Self {
        let text = format!("{code}");
        let coding = Coding::from(code);
        CodeableConceptInner {
            coding: vec![Some(coding)],
            text: Some(text),
            id: None,
            extension: Vec::new(),
            coding_ext: Vec::new(),
            text_ext: None,
        }
            .into()
    }
}
