//! Generated code! Take a look at the generator-crate for changing this file!
#![allow(clippy::too_many_lines)]
use std::hash::Hash;
use serde::{Serialize, Deserialize};
use super::super::types::{Coding, CodingInner, CodeableConcept, CodeableConceptInner};
#[doc = "**[AccountStatus](http://hl7.org/fhir/ValueSet/account-status)**. Indicates whether the account is available to be used.\n\nFHIR version: 5.0.0."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum AccountStatus {
    /** **active**

Active. This account is active and may be used. */
    Active,
    /** **entered-in-error**

Entered in error. This instance should not have been part of this patient's medical record. */
    EnteredInError,
    /** **inactive**

Inactive. This account is inactive and should not be used to track financial information. */
    Inactive,
    /** **on-hold**

On Hold. This account is on hold. */
    OnHold,
    /** **unknown**

Unknown. The account status is unknown. */
    Unknown,
}
impl ::core::str::FromStr for AccountStatus {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "entered-in-error" => Ok(Self::EnteredInError),
            "inactive" => Ok(Self::Inactive),
            "on-hold" => Ok(Self::OnHold),
            "unknown" => Ok(Self::Unknown),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for AccountStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Active => "active",
            Self::EnteredInError => "entered-in-error",
            Self::Inactive => "inactive",
            Self::OnHold => "on-hold",
            Self::Unknown => "unknown",
        }
    }
}
impl ::std::fmt::Debug for AccountStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for AccountStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for AccountStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for AccountStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<AccountStatus> for Coding {
    fn from(code: AccountStatus) -> Self {
        CodingInner {
            system: Some("http://hl7.org/fhir/ValueSet/account-status".to_owned()),
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
impl From<AccountStatus> for CodeableConcept {
    fn from(code: AccountStatus) -> Self {
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
#[doc = "**[AdministrativeGender](http://hl7.org/fhir/ValueSet/administrative-gender)**. The gender of a person used for administrative purposes.\n\nFHIR version: 5.0.0."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum AdministrativeGender {
    /** **female**

Female. Female. */
    Female,
    /** **male**

Male. Male. */
    Male,
    /** **other**

Other. Other. */
    Other,
    /** **unknown**

Unknown. Unknown. */
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
#[doc = "**[BundleType](http://hl7.org/fhir/ValueSet/bundle-type)**. Indicates the purpose of a bundle - how it is intended to be used.\n\nFHIR version: 5.0.0."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum BundleType {
    /** **batch**

Batch. The bundle is a set of actions - intended to be processed by a server as a group of independent actions. */
    Batch,
    /** **batch-response**

Batch Response. The bundle is a batch response. Note that as a batch, some responses may indicate failure and others success. */
    BatchResponse,
    /** **collection**

Collection. The bundle is a set of resources collected into a single package for ease of distribution that imposes no processing obligations or behavioral rules beyond persistence. */
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
    /** **subscription-notification**

Subscription Notification. The bundle has been generated by a Subscription to communicate information to a client. */
    SubscriptionNotification,
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
            "subscription-notification" => Ok(Self::SubscriptionNotification),
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
            Self::SubscriptionNotification => "subscription-notification",
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
#[doc = "**[CatalogType](http://hl7.org/fhir/catalogType)**. CatalogType\n\nFHIR version: 5.0.0."]
#[derive(PartialEq, Eq, Hash, Clone)]
pub enum CatalogType {
    /** **device**

Device Catalog. Device Catalog. */
    Device,
    /** **medication**

Medication Catalog. Medication Catalog. */
    Medication,
    /** **protocol**

Protocol List. Protocol List. */
    Protocol,
    /// Custom code value.
    _Custom(String),
}
impl ::core::str::FromStr for CatalogType {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "device" => Ok(Self::Device),
            "medication" => Ok(Self::Medication),
            "protocol" => Ok(Self::Protocol),
            _ => Ok(Self::_Custom(s.to_owned())),
        }
    }
}
impl AsRef<str> for CatalogType {
    fn as_ref(&self) -> &str {
        match self {
            Self::Device => "device",
            Self::Medication => "medication",
            Self::Protocol => "protocol",
            Self::_Custom(s) => s.as_str(),
        }
    }
}
impl ::std::fmt::Debug for CatalogType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for CatalogType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for CatalogType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for CatalogType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<CatalogType> for Coding {
    fn from(code: CatalogType) -> Self {
        CodingInner {
            system: Some("http://hl7.org/fhir/catalogType".to_owned()),
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
impl From<CatalogType> for CodeableConcept {
    fn from(code: CatalogType) -> Self {
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
#[doc = "**[EncounterStatus](http://hl7.org/fhir/ValueSet/encounter-status)**. Current state of the encounter.\n\nFHIR version: 5.0.0."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum EncounterStatus {
    /** **cancelled**

Cancelled. The Encounter has ended before it has begun. */
    Cancelled,
    /** **completed**

Completed. The Encounter has ended. */
    Completed,
    /** **discharged**

Discharged. The Encounter has been clinically completed, the patient has been discharged from the facility or the visit has ended, and the patient may have departed (refer to subjectStatus). While the encounter is in this status, administrative activities are usually performed, collating all required documentation and charge information before being released for billing, at which point the status will move to completed. */
    Discharged,
    /** **discontinued**

Discontinued. The Encounter has started, but was not able to be completed. Further action may need to be performed, such as rescheduling appointments related to this encounter. */
    Discontinued,
    /** **entered-in-error**

Entered in Error. This instance should not have been part of this patient's medical record. */
    EnteredInError,
    /** **in-progress**

In Progress. The Encounter has begun and the patient is present / the practitioner and the patient are meeting. */
    InProgress,
    /** **on-hold**

On Hold. The Encounter has begun, but is currently on hold, e.g. because the patient is temporarily on leave. */
    OnHold,
    /** **planned**

Planned. The Encounter has not yet started. */
    Planned,
    /** **unknown**

Unknown. The encounter status is unknown. Note that "unknown" is a value of last resort and every attempt should be made to provide a meaningful value other than "unknown". */
    Unknown,
}
impl ::core::str::FromStr for EncounterStatus {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "cancelled" => Ok(Self::Cancelled),
            "completed" => Ok(Self::Completed),
            "discharged" => Ok(Self::Discharged),
            "discontinued" => Ok(Self::Discontinued),
            "entered-in-error" => Ok(Self::EnteredInError),
            "in-progress" => Ok(Self::InProgress),
            "on-hold" => Ok(Self::OnHold),
            "planned" => Ok(Self::Planned),
            "unknown" => Ok(Self::Unknown),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for EncounterStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Cancelled => "cancelled",
            Self::Completed => "completed",
            Self::Discharged => "discharged",
            Self::Discontinued => "discontinued",
            Self::EnteredInError => "entered-in-error",
            Self::InProgress => "in-progress",
            Self::OnHold => "on-hold",
            Self::Planned => "planned",
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
#[doc = "**[HTTPVerb](http://hl7.org/fhir/ValueSet/http-verb)**. HTTP verbs (in the HTTP command line). See [HTTP rfc](https://tools.ietf.org/html/rfc7231) for details.\n\nFHIR version: 5.0.0."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum HTTPVerb {
    /** **DELETE**

DELETE. HTTP DELETE Command. */
    Delete,
    /** **GET**

GET. HTTP GET Command. */
    Get,
    /** **HEAD**

HEAD. HTTP HEAD Command. */
    Head,
    /** **PATCH**

PATCH. HTTP PATCH Command. */
    Patch,
    /** **POST**

POST. HTTP POST Command. */
    Post,
    /** **PUT**

PUT. HTTP PUT Command. */
    Put,
}
impl ::core::str::FromStr for HTTPVerb {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "DELETE" => Ok(Self::Delete),
            "GET" => Ok(Self::Get),
            "HEAD" => Ok(Self::Head),
            "PATCH" => Ok(Self::Patch),
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
            Self::Head => "HEAD",
            Self::Patch => "PATCH",
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
#[doc = "**[IssueSeverity](http://hl7.org/fhir/ValueSet/issue-severity)**. How the issue affects the success of the action.\n\nFHIR version: 5.0.0."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum IssueSeverity {
    /** **error**

Error. The issue is sufficiently important to cause the action to fail. */
    Error,
    /** **fatal**

Fatal. The issue caused the action to fail and no further checking could be performed. */
    Fatal,
    /** **information**

Information. The issue has no relation to the degree of success of the action. */
    Information,
    /** **success**

Operation Successful. The operation completed successfully. */
    Success,
    /** **warning**

Warning. The issue is not important enough to cause the action to fail but may cause it to be performed suboptimally or in a way that is not as desired. */
    Warning,
}
impl ::core::str::FromStr for IssueSeverity {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "error" => Ok(Self::Error),
            "fatal" => Ok(Self::Fatal),
            "information" => Ok(Self::Information),
            "success" => Ok(Self::Success),
            "warning" => Ok(Self::Warning),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for IssueSeverity {
    fn as_ref(&self) -> &str {
        match self {
            Self::Error => "error",
            Self::Fatal => "fatal",
            Self::Information => "information",
            Self::Success => "success",
            Self::Warning => "warning",
        }
    }
}
impl ::std::fmt::Debug for IssueSeverity {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for IssueSeverity {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for IssueSeverity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for IssueSeverity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<IssueSeverity> for Coding {
    fn from(code: IssueSeverity) -> Self {
        CodingInner {
            system: Some("http://hl7.org/fhir/ValueSet/issue-severity".to_owned()),
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
impl From<IssueSeverity> for CodeableConcept {
    fn from(code: IssueSeverity) -> Self {
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
#[doc = "**[Kind](http://hl7.org/fhir/ValueSet/coverage-kind)**. The nature of the Coverage details which convey who is paying potentially for health services.\n\nFHIR version: 5.0.0."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum Kind {
    /** **insurance**

Insurance. The Coverage provides the identifiers and card-level details of an insurance policy. */
    Insurance,
    /** **other**

Other. Some other organization is paying for the service. */
    Other,
    /** **self-pay**

Self-pay. One or more persons and/or organizations are paying for the services rendered. */
    SelfPay,
}
impl ::core::str::FromStr for Kind {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "insurance" => Ok(Self::Insurance),
            "other" => Ok(Self::Other),
            "self-pay" => Ok(Self::SelfPay),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for Kind {
    fn as_ref(&self) -> &str {
        match self {
            Self::Insurance => "insurance",
            Self::Other => "other",
            Self::SelfPay => "self-pay",
        }
    }
}
impl ::std::fmt::Debug for Kind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for Kind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for Kind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for Kind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<Kind> for Coding {
    fn from(code: Kind) -> Self {
        CodingInner {
            system: Some("http://hl7.org/fhir/ValueSet/coverage-kind".to_owned()),
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
impl From<Kind> for CodeableConcept {
    fn from(code: Kind) -> Self {
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
#[doc = "**[LinkRelationTypes](http://hl7.org/fhir/ValueSet/iana-link-relations)**. Link Relation Types defined at https://www.iana.org/assignments/link-relations/link-relations.xhtml#link-relations-1\n\nFHIR version: 5.0.0."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum LinkRelationTypes {
    /** **P3Pv1**

Refers to a P3P privacy policy for the context.. Refers to a P3P privacy policy for the context. */
    P3Pv1,
    /** **about**

Refers to a resource that is the subject of the link's context.. Refers to a resource that is the subject of the link's context. */
    About,
    /** **acl**

Asserts that the link target provides an access control description for the link context.. Asserts that the link target provides an access control description for the link context. */
    Acl,
    /** **alternate**

Refers to a substitute for this context. Refers to a substitute for this context */
    Alternate,
    /** **amphtml**

Used to reference alternative content that uses the AMP profile of the HTML format.. Used to reference alternative content that uses the AMP profile of the HTML format. */
    Amphtml,
    /** **appendix**

Refers to an appendix.. Refers to an appendix. */
    Appendix,
    /** **apple-touch-icon**

Refers to an icon for the context. Synonym for icon.. Refers to an icon for the context. Synonym for icon. */
    AppleTouchIcon,
    /** **apple-touch-startup-image**

Refers to a launch screen for the context.. Refers to a launch screen for the context. */
    AppleTouchStartupImage,
    /** **archives**

Refers to a collection of records, documents, or other
      materials of historical interest.. Refers to a collection of records, documents, or other
      materials of historical interest. */
    Archives,
    /** **author**

Refers to the context's author.. Refers to the context's author. */
    Author,
    /** **blocked-by**

Identifies the entity that blocks access to a resource
      following receipt of a legal demand.. Identifies the entity that blocks access to a resource
      following receipt of a legal demand. */
    BlockedBy,
    /** **bookmark**

Gives a permanent link to use for bookmarking purposes.. Gives a permanent link to use for bookmarking purposes. */
    Bookmark,
    /** **canonical**

Designates the preferred version of a resource (the IRI and its contents).. Designates the preferred version of a resource (the IRI and its contents). */
    Canonical,
    /** **chapter**

Refers to a chapter in a collection of resources.. Refers to a chapter in a collection of resources. */
    Chapter,
    /** **cite-as**

Indicates that the link target is preferred over the link context for the purpose of permanent citation.. Indicates that the link target is preferred over the link context for the purpose of permanent citation. */
    CiteAs,
    /** **collection**

The target IRI points to a resource which represents the collection resource for the context IRI.. The target IRI points to a resource which represents the collection resource for the context IRI. */
    Collection,
    /** **contents**

Refers to a table of contents.. Refers to a table of contents. */
    Contents,
    /** **convertedFrom**

The document linked to was later converted to the
      document that contains this link relation.  For example, an RFC can
      have a link to the Internet-Draft that became the RFC; in that case,
      the link relation would be "convertedFrom".. The document linked to was later converted to the
      document that contains this link relation.  For example, an RFC can
      have a link to the Internet-Draft that became the RFC; in that case,
      the link relation would be "convertedFrom". */
    ConvertedFrom,
    /** **copyright**

Refers to a copyright statement that applies to the
    link's context.. Refers to a copyright statement that applies to the
    link's context. */
    Copyright,
    /** **create-form**

The target IRI points to a resource where a submission form can be obtained.. The target IRI points to a resource where a submission form can be obtained. */
    CreateForm,
    /** **current**

Refers to a resource containing the most recent
      item(s) in a collection of resources.. Refers to a resource containing the most recent
      item(s) in a collection of resources. */
    Current,
    /** **describedby**

Refers to a resource providing information about the
      link's context.. Refers to a resource providing information about the
      link's context. */
    Describedby,
    /** **describes**

The relationship A 'describes' B asserts that
      resource A provides a description of resource B. There are no
      constraints on the format or representation of either A or B,
      neither are there any further constraints on either resource.. The relationship A 'describes' B asserts that
      resource A provides a description of resource B. There are no
      constraints on the format or representation of either A or B,
      neither are there any further constraints on either resource. */
    Describes,
    /** **disclosure**

Refers to a list of patent disclosures made with respect to
      material for which 'disclosure' relation is specified.. Refers to a list of patent disclosures made with respect to
      material for which 'disclosure' relation is specified. */
    Disclosure,
    /** **dns-prefetch**

Used to indicate an origin that will be used to fetch required
      resources for the link context, and that the user agent ought to resolve
      as early as possible.. Used to indicate an origin that will be used to fetch required
      resources for the link context, and that the user agent ought to resolve
      as early as possible. */
    DnsPrefetch,
    /** **duplicate**

Refers to a resource whose available representations
      are byte-for-byte identical with the corresponding representations of
      the context IRI.. Refers to a resource whose available representations
      are byte-for-byte identical with the corresponding representations of
      the context IRI. */
    Duplicate,
    /** **edit**

Refers to a resource that can be used to edit the
      link's context.. Refers to a resource that can be used to edit the
      link's context. */
    Edit,
    /** **edit-form**

The target IRI points to a resource where a submission form for
      editing associated resource can be obtained.. The target IRI points to a resource where a submission form for
      editing associated resource can be obtained. */
    EditForm,
    /** **edit-media**

Refers to a resource that can be used to edit media
      associated with the link's context.. Refers to a resource that can be used to edit media
      associated with the link's context. */
    EditMedia,
    /** **enclosure**

Identifies a related resource that is potentially
      large and might require special handling.. Identifies a related resource that is potentially
      large and might require special handling. */
    Enclosure,
    /** **external**

Refers to a resource that is not part of the same site as the current context.. Refers to a resource that is not part of the same site as the current context. */
    External,
    /** **first**

An IRI that refers to the furthest preceding resource
    in a series of resources.. An IRI that refers to the furthest preceding resource
    in a series of resources. */
    First,
    /** **glossary**

Refers to a glossary of terms.. Refers to a glossary of terms. */
    Glossary,
    /** **help**

Refers to context-sensitive help.. Refers to context-sensitive help. */
    Help,
    /** **hosts**

Refers to a resource hosted by the server indicated by
      the link context.. Refers to a resource hosted by the server indicated by
      the link context. */
    Hosts,
    /** **hub**

Refers to a hub that enables registration for
    notification of updates to the context.. Refers to a hub that enables registration for
    notification of updates to the context. */
    Hub,
    /** **icon**

Refers to an icon representing the link's context.. Refers to an icon representing the link's context. */
    Icon,
    /** **index**

Refers to an index.. Refers to an index. */
    Index,
    /** **intervalAfter**

refers to a resource associated with a time interval that ends before the beginning of the time interval associated with the context resource. refers to a resource associated with a time interval that ends before the beginning of the time interval associated with the context resource */
    IntervalAfter,
    /** **intervalBefore**

refers to a resource associated with a time interval that begins after the end of the time interval associated with the context resource. refers to a resource associated with a time interval that begins after the end of the time interval associated with the context resource */
    IntervalBefore,
    /** **intervalContains**

refers to a resource associated with a time interval that begins after the beginning of the time interval associated with the context resource, and ends before the end of the time interval associated with the context resource. refers to a resource associated with a time interval that begins after the beginning of the time interval associated with the context resource, and ends before the end of the time interval associated with the context resource */
    IntervalContains,
    /** **intervalDisjoint**

refers to a resource associated with a time interval that begins after the end of the time interval associated with the context resource, or ends before the beginning of the time interval associated with the context resource. refers to a resource associated with a time interval that begins after the end of the time interval associated with the context resource, or ends before the beginning of the time interval associated with the context resource */
    IntervalDisjoint,
    /** **intervalDuring**

refers to a resource associated with a time interval that begins before the beginning of the time interval associated with the context resource, and ends after the end of the time interval associated with the context resource. refers to a resource associated with a time interval that begins before the beginning of the time interval associated with the context resource, and ends after the end of the time interval associated with the context resource */
    IntervalDuring,
    /** **intervalEquals**

refers to a resource associated with a time interval whose beginning coincides with the beginning of the time interval associated with the context resource, and whose end coincides with the end of the time interval associated with the context resource. refers to a resource associated with a time interval whose beginning coincides with the beginning of the time interval associated with the context resource, and whose end coincides with the end of the time interval associated with the context resource */
    IntervalEquals,
    /** **intervalFinishedBy**

refers to a resource associated with a time interval that begins after the beginning of the time interval associated with the context resource, and whose end coincides with the end of the time interval associated with the context resource. refers to a resource associated with a time interval that begins after the beginning of the time interval associated with the context resource, and whose end coincides with the end of the time interval associated with the context resource */
    IntervalFinishedBy,
    /** **intervalFinishes**

refers to a resource associated with a time interval that begins before the beginning of the time interval associated with the context resource, and whose end coincides with the end of the time interval associated with the context resource. refers to a resource associated with a time interval that begins before the beginning of the time interval associated with the context resource, and whose end coincides with the end of the time interval associated with the context resource */
    IntervalFinishes,
    /** **intervalIn**

refers to a resource associated with a time interval that begins before or is coincident with the beginning of the time interval associated with the context resource, and ends after or is coincident with the end of the time interval associated with the context resource. refers to a resource associated with a time interval that begins before or is coincident with the beginning of the time interval associated with the context resource, and ends after or is coincident with the end of the time interval associated with the context resource */
    IntervalIn,
    /** **intervalMeets**

refers to a resource associated with a time interval whose beginning coincides with the end of the time interval associated with the context resource. refers to a resource associated with a time interval whose beginning coincides with the end of the time interval associated with the context resource */
    IntervalMeets,
    /** **intervalMetBy**

refers to a resource associated with a time interval whose end coincides with the beginning of the time interval associated with the context resource. refers to a resource associated with a time interval whose end coincides with the beginning of the time interval associated with the context resource */
    IntervalMetBy,
    /** **intervalOverlappedBy**

refers to a resource associated with a time interval that begins before the beginning of the time interval associated with the context resource, and ends after the beginning of the time interval associated with the context resource. refers to a resource associated with a time interval that begins before the beginning of the time interval associated with the context resource, and ends after the beginning of the time interval associated with the context resource */
    IntervalOverlappedBy,
    /** **intervalOverlaps**

refers to a resource associated with a time interval that begins before the end of the time interval associated with the context resource, and ends after the end of the time interval associated with the context resource. refers to a resource associated with a time interval that begins before the end of the time interval associated with the context resource, and ends after the end of the time interval associated with the context resource */
    IntervalOverlaps,
    /** **intervalStartedBy**

refers to a resource associated with a time interval whose beginning coincides with the beginning of the time interval associated with the context resource, and ends before the end of the time interval associated with the context resource. refers to a resource associated with a time interval whose beginning coincides with the beginning of the time interval associated with the context resource, and ends before the end of the time interval associated with the context resource */
    IntervalStartedBy,
    /** **intervalStarts**

refers to a resource associated with a time interval whose beginning coincides with the beginning of the time interval associated with the context resource, and ends after the end of the time interval associated with the context resource. refers to a resource associated with a time interval whose beginning coincides with the beginning of the time interval associated with the context resource, and ends after the end of the time interval associated with the context resource */
    IntervalStarts,
    /** **item**

The target IRI points to a resource that is a member of the collection represented by the context IRI.. The target IRI points to a resource that is a member of the collection represented by the context IRI. */
    Item,
    /** **last**

An IRI that refers to the furthest following resource
      in a series of resources.. An IRI that refers to the furthest following resource
      in a series of resources. */
    Last,
    /** **latest-version**

Points to a resource containing the latest (e.g.,
      current) version of the context.. Points to a resource containing the latest (e.g.,
      current) version of the context. */
    LatestVersion,
    /** **license**

Refers to a license associated with this context.. Refers to a license associated with this context. */
    License,
    /** **linkset**

The link target of a link with the "linkset" relation
      type provides a set of links, including links in which the link
      context of the link participates.
    . The link target of a link with the "linkset" relation
      type provides a set of links, including links in which the link
      context of the link participates.
     */
    Linkset,
    /** **lrdd**

Refers to further information about the link's context,
      expressed as a LRDD ("Link-based Resource Descriptor Document")
      resource.  See  for information about
      processing this relation type in host-meta documents. When used
      elsewhere, it refers to additional links and other metadata.
      Multiple instances indicate additional LRDD resources. LRDD
      resources MUST have an "application/xrd+xml" representation, and
      MAY have others.. Refers to further information about the link's context,
      expressed as a LRDD ("Link-based Resource Descriptor Document")
      resource.  See  for information about
      processing this relation type in host-meta documents. When used
      elsewhere, it refers to additional links and other metadata.
      Multiple instances indicate additional LRDD resources. LRDD
      resources MUST have an "application/xrd+xml" representation, and
      MAY have others. */
    Lrdd,
    /** **manifest**

Links to a manifest file for the context.. Links to a manifest file for the context. */
    Manifest,
    /** **mask-icon**

Refers to a mask that can be applied to the icon for the context.. Refers to a mask that can be applied to the icon for the context. */
    MaskIcon,
    /** **media-feed**

Refers to a feed of personalised media recommendations relevant to the link context.. Refers to a feed of personalised media recommendations relevant to the link context. */
    MediaFeed,
    /** **memento**

The Target IRI points to a Memento, a fixed resource that will not change state anymore.. The Target IRI points to a Memento, a fixed resource that will not change state anymore. */
    Memento,
    /** **micropub**

Links to the context's Micropub endpoint.. Links to the context's Micropub endpoint. */
    Micropub,
    /** **modulepreload**

Refers to a module that the user agent is to preemptively fetch and store for use in the current context.. Refers to a module that the user agent is to preemptively fetch and store for use in the current context. */
    Modulepreload,
    /** **monitor**

Refers to a resource that can be used to monitor changes in an HTTP resource.
    . Refers to a resource that can be used to monitor changes in an HTTP resource.
     */
    Monitor,
    /** **monitor-group**

Refers to a resource that can be used to monitor changes in a specified group of HTTP resources.
    . Refers to a resource that can be used to monitor changes in a specified group of HTTP resources.
     */
    MonitorGroup,
    /** **next**

Indicates that the link's context is a part of a series, and
      that the next in the series is the link target.
    . Indicates that the link's context is a part of a series, and
      that the next in the series is the link target.
     */
    Next,
    /** **next-archive**

Refers to the immediately following archive resource.. Refers to the immediately following archive resource. */
    NextArchive,
    /** **nofollow**

Indicates that the context’s original author or publisher does not endorse the link target.. Indicates that the context’s original author or publisher does not endorse the link target. */
    Nofollow,
    /** **noopener**

Indicates that any newly created top-level browsing context which results from following the link will not be an auxiliary browsing context.. Indicates that any newly created top-level browsing context which results from following the link will not be an auxiliary browsing context. */
    Noopener,
    /** **noreferrer**

Indicates that no referrer information is to be leaked when following the link.. Indicates that no referrer information is to be leaked when following the link. */
    Noreferrer,
    /** **opener**

Indicates that any newly created top-level browsing context which results from following the link will be an auxiliary browsing context.. Indicates that any newly created top-level browsing context which results from following the link will be an auxiliary browsing context. */
    Opener,
    /** **openid2.local_id**

Refers to an OpenID Authentication server on which the context relies for an assertion that the end user controls an Identifier.. Refers to an OpenID Authentication server on which the context relies for an assertion that the end user controls an Identifier. */
    Openid2LocalId,
    /** **openid2.provider**

Refers to a resource which accepts OpenID Authentication protocol messages for the context.. Refers to a resource which accepts OpenID Authentication protocol messages for the context. */
    Openid2Provider,
    /** **original**

The Target IRI points to an Original Resource.. The Target IRI points to an Original Resource. */
    Original,
    /** **payment**

Indicates a resource where payment is accepted.. Indicates a resource where payment is accepted. */
    Payment,
    /** **pingback**

Gives the address of the pingback resource for the link context.. Gives the address of the pingback resource for the link context. */
    Pingback,
    /** **preconnect**

Used to indicate an origin that will be used to fetch required
      resources for the link context. Initiating an early connection, which
      includes the DNS lookup, TCP handshake, and optional TLS negotiation,
      allows the user agent to mask the high latency costs of establishing a
      connection.. Used to indicate an origin that will be used to fetch required
      resources for the link context. Initiating an early connection, which
      includes the DNS lookup, TCP handshake, and optional TLS negotiation,
      allows the user agent to mask the high latency costs of establishing a
      connection. */
    Preconnect,
    /** **predecessor-version**

Points to a resource containing the predecessor
      version in the version history.
    . Points to a resource containing the predecessor
      version in the version history.
     */
    PredecessorVersion,
    /** **prefetch**

The prefetch link relation type is used to identify a resource
      that might be required by the next navigation from the link context, and
      that the user agent ought to fetch, such that the user agent can deliver a
      faster response once the resource is requested in the future.. The prefetch link relation type is used to identify a resource
      that might be required by the next navigation from the link context, and
      that the user agent ought to fetch, such that the user agent can deliver a
      faster response once the resource is requested in the future. */
    Prefetch,
    /** **preload**

Refers to a resource that should be loaded early in the
      processing of the link's context, without blocking rendering.. Refers to a resource that should be loaded early in the
      processing of the link's context, without blocking rendering. */
    Preload,
    /** **prerender**

Used to identify a resource that might be required by the next
      navigation from the link context, and that the user agent ought to fetch
      and execute, such that the user agent can deliver a faster response once
      the resource is requested in the future.. Used to identify a resource that might be required by the next
      navigation from the link context, and that the user agent ought to fetch
      and execute, such that the user agent can deliver a faster response once
      the resource is requested in the future. */
    Prerender,
    /** **prev**

Indicates that the link's context is a part of a series, and
      that the previous in the series is the link target.
    . Indicates that the link's context is a part of a series, and
      that the previous in the series is the link target.
     */
    Prev,
    /** **prev-archive**

Refers to the immediately preceding archive resource.. Refers to the immediately preceding archive resource. */
    PrevArchive,
    /** **preview**

Refers to a resource that provides a preview of the link's context.. Refers to a resource that provides a preview of the link's context. */
    Preview,
    /** **previous**

Refers to the previous resource in an ordered series
      of resources.  Synonym for "prev".. Refers to the previous resource in an ordered series
      of resources.  Synonym for "prev". */
    Previous,
    /** **privacy-policy**

Refers to a privacy policy associated with the link's context.. Refers to a privacy policy associated with the link's context. */
    PrivacyPolicy,
    /** **profile**

Identifying that a resource representation conforms
to a certain profile, without affecting the non-profile semantics
of the resource representation.. Identifying that a resource representation conforms
to a certain profile, without affecting the non-profile semantics
of the resource representation. */
    Profile,
    /** **publication**

Links to a publication manifest. A manifest represents
      structured information about a publication, such as informative metadata,
      a list of resources, and a default reading order.. Links to a publication manifest. A manifest represents
      structured information about a publication, such as informative metadata,
      a list of resources, and a default reading order. */
    Publication,
    /** **related**

Identifies a related resource.. Identifies a related resource. */
    Related,
    /** **replies**

Identifies a resource that is a reply to the context
      of the link.
    . Identifies a resource that is a reply to the context
      of the link.
     */
    Replies,
    /** **restconf**

Identifies the root of RESTCONF API as configured on this HTTP server.
      The "restconf" relation defines the root of the API defined in RFC8040.
      Subsequent revisions of RESTCONF will use alternate relation values to support
      protocol versioning.. Identifies the root of RESTCONF API as configured on this HTTP server.
      The "restconf" relation defines the root of the API defined in RFC8040.
      Subsequent revisions of RESTCONF will use alternate relation values to support
      protocol versioning. */
    Restconf,
    /** **ruleinput**

The resource identified by the link target provides an input value to an
    instance of a rule, where the resource which represents the rule instance is
    identified by the link context.
    . The resource identified by the link target provides an input value to an
    instance of a rule, where the resource which represents the rule instance is
    identified by the link context.
     */
    Ruleinput,
    /** **search**

Refers to a resource that can be used to search through
      the link's context and related resources.. Refers to a resource that can be used to search through
      the link's context and related resources. */
    Search,
    /** **section**

Refers to a section in a collection of resources.. Refers to a section in a collection of resources. */
    Section,
    /** **self**

Conveys an identifier for the link's context.
    . Conveys an identifier for the link's context.
     */
    _Self,
    /** **service**

Indicates a URI that can be used to retrieve a
      service document.. Indicates a URI that can be used to retrieve a
      service document. */
    Service,
    /** **service-desc**

Identifies service description for the context that
      is primarily intended for consumption by machines.. Identifies service description for the context that
      is primarily intended for consumption by machines. */
    ServiceDesc,
    /** **service-doc**

Identifies service documentation for the context that
      is primarily intended for human consumption.. Identifies service documentation for the context that
      is primarily intended for human consumption. */
    ServiceDoc,
    /** **service-meta**

Identifies general metadata for the context that is
      primarily intended for consumption by machines.. Identifies general metadata for the context that is
      primarily intended for consumption by machines. */
    ServiceMeta,
    /** **sponsored**

Refers to a resource that is within a context that is
		sponsored (such as advertising or another compensation agreement).. Refers to a resource that is within a context that is
		sponsored (such as advertising or another compensation agreement). */
    Sponsored,
    /** **start**

Refers to the first resource in a collection of
      resources.. Refers to the first resource in a collection of
      resources. */
    Start,
    /** **status**

Identifies a resource that represents the context's
      status.. Identifies a resource that represents the context's
      status. */
    Status,
    /** **stylesheet**

Refers to a stylesheet.. Refers to a stylesheet. */
    Stylesheet,
    /** **subsection**

Refers to a resource serving as a subsection in a
      collection of resources.. Refers to a resource serving as a subsection in a
      collection of resources. */
    Subsection,
    /** **successor-version**

Points to a resource containing the successor version
      in the version history.
    . Points to a resource containing the successor version
      in the version history.
     */
    SuccessorVersion,
    /** **sunset**

Identifies a resource that provides information about
      the context's retirement policy.
    . Identifies a resource that provides information about
      the context's retirement policy.
     */
    Sunset,
    /** **tag**

Gives a tag (identified by the given address) that applies to
      the current document.
    . Gives a tag (identified by the given address) that applies to
      the current document.
     */
    Tag,
    /** **terms-of-service**

Refers to the terms of service associated with the link's context.. Refers to the terms of service associated with the link's context. */
    TermsOfService,
    /** **timegate**

The Target IRI points to a TimeGate for an Original Resource.. The Target IRI points to a TimeGate for an Original Resource. */
    Timegate,
    /** **timemap**

The Target IRI points to a TimeMap for an Original Resource.. The Target IRI points to a TimeMap for an Original Resource. */
    Timemap,
    /** **type**

Refers to a resource identifying the abstract semantic type of which the link's context is considered to be an instance.. Refers to a resource identifying the abstract semantic type of which the link's context is considered to be an instance. */
    Type,
    /** **ugc**

Refers to a resource that is within a context that is User Generated Content.
    . Refers to a resource that is within a context that is User Generated Content.
     */
    Ugc,
    /** **up**

Refers to a parent document in a hierarchy of
      documents.
    . Refers to a parent document in a hierarchy of
      documents.
     */
    Up,
    /** **version-history**

Points to a resource containing the version history
      for the context.
    . Points to a resource containing the version history
      for the context.
     */
    VersionHistory,
    /** **via**

Identifies a resource that is the source of the
      information in the link's context.
    . Identifies a resource that is the source of the
      information in the link's context.
     */
    Via,
    /** **webmention**

Identifies a target URI that supports the Webmention protocol.
    This allows clients that mention a resource in some form of publishing process
    to contact that endpoint and inform it that this resource has been mentioned.. Identifies a target URI that supports the Webmention protocol.
    This allows clients that mention a resource in some form of publishing process
    to contact that endpoint and inform it that this resource has been mentioned. */
    Webmention,
    /** **working-copy**

Points to a working copy for this resource.. Points to a working copy for this resource. */
    WorkingCopy,
    /** **working-copy-of**

Points to the versioned resource from which this
      working copy was obtained.
    . Points to the versioned resource from which this
      working copy was obtained.
     */
    WorkingCopyOf,
}
impl ::core::str::FromStr for LinkRelationTypes {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "P3Pv1" => Ok(Self::P3Pv1),
            "about" => Ok(Self::About),
            "acl" => Ok(Self::Acl),
            "alternate" => Ok(Self::Alternate),
            "amphtml" => Ok(Self::Amphtml),
            "appendix" => Ok(Self::Appendix),
            "apple-touch-icon" => Ok(Self::AppleTouchIcon),
            "apple-touch-startup-image" => Ok(Self::AppleTouchStartupImage),
            "archives" => Ok(Self::Archives),
            "author" => Ok(Self::Author),
            "blocked-by" => Ok(Self::BlockedBy),
            "bookmark" => Ok(Self::Bookmark),
            "canonical" => Ok(Self::Canonical),
            "chapter" => Ok(Self::Chapter),
            "cite-as" => Ok(Self::CiteAs),
            "collection" => Ok(Self::Collection),
            "contents" => Ok(Self::Contents),
            "convertedFrom" => Ok(Self::ConvertedFrom),
            "copyright" => Ok(Self::Copyright),
            "create-form" => Ok(Self::CreateForm),
            "current" => Ok(Self::Current),
            "describedby" => Ok(Self::Describedby),
            "describes" => Ok(Self::Describes),
            "disclosure" => Ok(Self::Disclosure),
            "dns-prefetch" => Ok(Self::DnsPrefetch),
            "duplicate" => Ok(Self::Duplicate),
            "edit" => Ok(Self::Edit),
            "edit-form" => Ok(Self::EditForm),
            "edit-media" => Ok(Self::EditMedia),
            "enclosure" => Ok(Self::Enclosure),
            "external" => Ok(Self::External),
            "first" => Ok(Self::First),
            "glossary" => Ok(Self::Glossary),
            "help" => Ok(Self::Help),
            "hosts" => Ok(Self::Hosts),
            "hub" => Ok(Self::Hub),
            "icon" => Ok(Self::Icon),
            "index" => Ok(Self::Index),
            "intervalAfter" => Ok(Self::IntervalAfter),
            "intervalBefore" => Ok(Self::IntervalBefore),
            "intervalContains" => Ok(Self::IntervalContains),
            "intervalDisjoint" => Ok(Self::IntervalDisjoint),
            "intervalDuring" => Ok(Self::IntervalDuring),
            "intervalEquals" => Ok(Self::IntervalEquals),
            "intervalFinishedBy" => Ok(Self::IntervalFinishedBy),
            "intervalFinishes" => Ok(Self::IntervalFinishes),
            "intervalIn" => Ok(Self::IntervalIn),
            "intervalMeets" => Ok(Self::IntervalMeets),
            "intervalMetBy" => Ok(Self::IntervalMetBy),
            "intervalOverlappedBy" => Ok(Self::IntervalOverlappedBy),
            "intervalOverlaps" => Ok(Self::IntervalOverlaps),
            "intervalStartedBy" => Ok(Self::IntervalStartedBy),
            "intervalStarts" => Ok(Self::IntervalStarts),
            "item" => Ok(Self::Item),
            "last" => Ok(Self::Last),
            "latest-version" => Ok(Self::LatestVersion),
            "license" => Ok(Self::License),
            "linkset" => Ok(Self::Linkset),
            "lrdd" => Ok(Self::Lrdd),
            "manifest" => Ok(Self::Manifest),
            "mask-icon" => Ok(Self::MaskIcon),
            "media-feed" => Ok(Self::MediaFeed),
            "memento" => Ok(Self::Memento),
            "micropub" => Ok(Self::Micropub),
            "modulepreload" => Ok(Self::Modulepreload),
            "monitor" => Ok(Self::Monitor),
            "monitor-group" => Ok(Self::MonitorGroup),
            "next" => Ok(Self::Next),
            "next-archive" => Ok(Self::NextArchive),
            "nofollow" => Ok(Self::Nofollow),
            "noopener" => Ok(Self::Noopener),
            "noreferrer" => Ok(Self::Noreferrer),
            "opener" => Ok(Self::Opener),
            "openid2.local_id" => Ok(Self::Openid2LocalId),
            "openid2.provider" => Ok(Self::Openid2Provider),
            "original" => Ok(Self::Original),
            "payment" => Ok(Self::Payment),
            "pingback" => Ok(Self::Pingback),
            "preconnect" => Ok(Self::Preconnect),
            "predecessor-version" => Ok(Self::PredecessorVersion),
            "prefetch" => Ok(Self::Prefetch),
            "preload" => Ok(Self::Preload),
            "prerender" => Ok(Self::Prerender),
            "prev" => Ok(Self::Prev),
            "prev-archive" => Ok(Self::PrevArchive),
            "preview" => Ok(Self::Preview),
            "previous" => Ok(Self::Previous),
            "privacy-policy" => Ok(Self::PrivacyPolicy),
            "profile" => Ok(Self::Profile),
            "publication" => Ok(Self::Publication),
            "related" => Ok(Self::Related),
            "replies" => Ok(Self::Replies),
            "restconf" => Ok(Self::Restconf),
            "ruleinput" => Ok(Self::Ruleinput),
            "search" => Ok(Self::Search),
            "section" => Ok(Self::Section),
            "self" => Ok(Self::_Self),
            "service" => Ok(Self::Service),
            "service-desc" => Ok(Self::ServiceDesc),
            "service-doc" => Ok(Self::ServiceDoc),
            "service-meta" => Ok(Self::ServiceMeta),
            "sponsored" => Ok(Self::Sponsored),
            "start" => Ok(Self::Start),
            "status" => Ok(Self::Status),
            "stylesheet" => Ok(Self::Stylesheet),
            "subsection" => Ok(Self::Subsection),
            "successor-version" => Ok(Self::SuccessorVersion),
            "sunset" => Ok(Self::Sunset),
            "tag" => Ok(Self::Tag),
            "terms-of-service" => Ok(Self::TermsOfService),
            "timegate" => Ok(Self::Timegate),
            "timemap" => Ok(Self::Timemap),
            "type" => Ok(Self::Type),
            "ugc" => Ok(Self::Ugc),
            "up" => Ok(Self::Up),
            "version-history" => Ok(Self::VersionHistory),
            "via" => Ok(Self::Via),
            "webmention" => Ok(Self::Webmention),
            "working-copy" => Ok(Self::WorkingCopy),
            "working-copy-of" => Ok(Self::WorkingCopyOf),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for LinkRelationTypes {
    fn as_ref(&self) -> &str {
        match self {
            Self::P3Pv1 => "P3Pv1",
            Self::About => "about",
            Self::Acl => "acl",
            Self::Alternate => "alternate",
            Self::Amphtml => "amphtml",
            Self::Appendix => "appendix",
            Self::AppleTouchIcon => "apple-touch-icon",
            Self::AppleTouchStartupImage => "apple-touch-startup-image",
            Self::Archives => "archives",
            Self::Author => "author",
            Self::BlockedBy => "blocked-by",
            Self::Bookmark => "bookmark",
            Self::Canonical => "canonical",
            Self::Chapter => "chapter",
            Self::CiteAs => "cite-as",
            Self::Collection => "collection",
            Self::Contents => "contents",
            Self::ConvertedFrom => "convertedFrom",
            Self::Copyright => "copyright",
            Self::CreateForm => "create-form",
            Self::Current => "current",
            Self::Describedby => "describedby",
            Self::Describes => "describes",
            Self::Disclosure => "disclosure",
            Self::DnsPrefetch => "dns-prefetch",
            Self::Duplicate => "duplicate",
            Self::Edit => "edit",
            Self::EditForm => "edit-form",
            Self::EditMedia => "edit-media",
            Self::Enclosure => "enclosure",
            Self::External => "external",
            Self::First => "first",
            Self::Glossary => "glossary",
            Self::Help => "help",
            Self::Hosts => "hosts",
            Self::Hub => "hub",
            Self::Icon => "icon",
            Self::Index => "index",
            Self::IntervalAfter => "intervalAfter",
            Self::IntervalBefore => "intervalBefore",
            Self::IntervalContains => "intervalContains",
            Self::IntervalDisjoint => "intervalDisjoint",
            Self::IntervalDuring => "intervalDuring",
            Self::IntervalEquals => "intervalEquals",
            Self::IntervalFinishedBy => "intervalFinishedBy",
            Self::IntervalFinishes => "intervalFinishes",
            Self::IntervalIn => "intervalIn",
            Self::IntervalMeets => "intervalMeets",
            Self::IntervalMetBy => "intervalMetBy",
            Self::IntervalOverlappedBy => "intervalOverlappedBy",
            Self::IntervalOverlaps => "intervalOverlaps",
            Self::IntervalStartedBy => "intervalStartedBy",
            Self::IntervalStarts => "intervalStarts",
            Self::Item => "item",
            Self::Last => "last",
            Self::LatestVersion => "latest-version",
            Self::License => "license",
            Self::Linkset => "linkset",
            Self::Lrdd => "lrdd",
            Self::Manifest => "manifest",
            Self::MaskIcon => "mask-icon",
            Self::MediaFeed => "media-feed",
            Self::Memento => "memento",
            Self::Micropub => "micropub",
            Self::Modulepreload => "modulepreload",
            Self::Monitor => "monitor",
            Self::MonitorGroup => "monitor-group",
            Self::Next => "next",
            Self::NextArchive => "next-archive",
            Self::Nofollow => "nofollow",
            Self::Noopener => "noopener",
            Self::Noreferrer => "noreferrer",
            Self::Opener => "opener",
            Self::Openid2LocalId => "openid2.local_id",
            Self::Openid2Provider => "openid2.provider",
            Self::Original => "original",
            Self::Payment => "payment",
            Self::Pingback => "pingback",
            Self::Preconnect => "preconnect",
            Self::PredecessorVersion => "predecessor-version",
            Self::Prefetch => "prefetch",
            Self::Preload => "preload",
            Self::Prerender => "prerender",
            Self::Prev => "prev",
            Self::PrevArchive => "prev-archive",
            Self::Preview => "preview",
            Self::Previous => "previous",
            Self::PrivacyPolicy => "privacy-policy",
            Self::Profile => "profile",
            Self::Publication => "publication",
            Self::Related => "related",
            Self::Replies => "replies",
            Self::Restconf => "restconf",
            Self::Ruleinput => "ruleinput",
            Self::Search => "search",
            Self::Section => "section",
            Self::_Self => "self",
            Self::Service => "service",
            Self::ServiceDesc => "service-desc",
            Self::ServiceDoc => "service-doc",
            Self::ServiceMeta => "service-meta",
            Self::Sponsored => "sponsored",
            Self::Start => "start",
            Self::Status => "status",
            Self::Stylesheet => "stylesheet",
            Self::Subsection => "subsection",
            Self::SuccessorVersion => "successor-version",
            Self::Sunset => "sunset",
            Self::Tag => "tag",
            Self::TermsOfService => "terms-of-service",
            Self::Timegate => "timegate",
            Self::Timemap => "timemap",
            Self::Type => "type",
            Self::Ugc => "ugc",
            Self::Up => "up",
            Self::VersionHistory => "version-history",
            Self::Via => "via",
            Self::Webmention => "webmention",
            Self::WorkingCopy => "working-copy",
            Self::WorkingCopyOf => "working-copy-of",
        }
    }
}
impl ::std::fmt::Debug for LinkRelationTypes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for LinkRelationTypes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for LinkRelationTypes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for LinkRelationTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<LinkRelationTypes> for Coding {
    fn from(code: LinkRelationTypes) -> Self {
        CodingInner {
            system: Some("http://hl7.org/fhir/ValueSet/iana-link-relations".to_owned()),
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
impl From<LinkRelationTypes> for CodeableConcept {
    fn from(code: LinkRelationTypes) -> Self {
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
#[doc = "**[ObservationStatus](http://hl7.org/fhir/ValueSet/observation-status)**. Codes providing the status of an observation.\n\nFHIR version: 5.0.0."]
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

Entered in Error. The observation has been withdrawn following previous final release.  This electronic record should never have existed, though it is possible that real-world decisions were based on it. (If real-world activity has occurred, the status should be "cancelled" rather than "entered-in-error".). */
    EnteredInError,
    /** **final**

Final. The observation is complete and there are no further actions needed. Additional information such "released", "signed", etc. would be represented using [Provenance](provenance.html) which provides not only the act but also the actors and dates and other related data. These act states would be associated with an observation status of `preliminary` until they are all completed and then a status of `final` would be applied. */
    Final,
    /** **preliminary**

Preliminary. This is an initial or interim observation: data may be incomplete or unverified. */
    Preliminary,
    /** **registered**

Registered. The existence of the observation is registered, but there is no result yet available. */
    Registered,
    /** **unknown**

Unknown. The authoring/source system does not know which of the status values currently applies for this observation. Note: This concept is not to be used for "other" - one of the listed statuses is presumed to apply, but the authoring/source system does not know which. */
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
#[doc = "**[RequestIntent](http://hl7.org/fhir/ValueSet/request-intent)**. Codes indicating the degree of authority/intentionality associated with a request.\n\nFHIR version: 5.0.0."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum RequestIntent {
    /** **directive**

Directive. The request represents a legally binding instruction authored by a Patient or RelatedPerson. */
    Directive,
    /** **filler-order**

Filler Order. The request represents the view of an authorization instantiated by a fulfilling system representing the details of the fulfiller's intention to act upon a submitted order. */
    FillerOrder,
    /** **instance-order**

Instance Order. An order created in fulfillment of a broader order that represents the authorization for a single activity occurrence.  E.g. The administration of a single dose of a drug. */
    InstanceOrder,
    /** **option**

Option. The request represents a component or option for a RequestOrchestration that establishes timing, conditionality and/or other constraints among a set of requests.  Refer to [[[RequestOrchestration]]] for additional information on how this status is used. */
    Option,
    /** **order**

Order. The request represents a request/demand and authorization for action by the requestor. */
    Order,
    /** **original-order**

Original Order. The request represents an original authorization for action. */
    OriginalOrder,
    /** **plan**

Plan. The request represents an intention to ensure something occurs without providing an authorization for others to act. */
    Plan,
    /** **proposal**

Proposal. The request is a suggestion made by someone/something that does not have an intention to ensure it occurs and without providing an authorization to act. */
    Proposal,
    /** **reflex-order**

Reflex Order. The request represents an automatically generated supplemental authorization for action based on a parent authorization together with initial results of the action taken against that parent authorization. */
    ReflexOrder,
}
impl ::core::str::FromStr for RequestIntent {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "directive" => Ok(Self::Directive),
            "filler-order" => Ok(Self::FillerOrder),
            "instance-order" => Ok(Self::InstanceOrder),
            "option" => Ok(Self::Option),
            "order" => Ok(Self::Order),
            "original-order" => Ok(Self::OriginalOrder),
            "plan" => Ok(Self::Plan),
            "proposal" => Ok(Self::Proposal),
            "reflex-order" => Ok(Self::ReflexOrder),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for RequestIntent {
    fn as_ref(&self) -> &str {
        match self {
            Self::Directive => "directive",
            Self::FillerOrder => "filler-order",
            Self::InstanceOrder => "instance-order",
            Self::Option => "option",
            Self::Order => "order",
            Self::OriginalOrder => "original-order",
            Self::Plan => "plan",
            Self::Proposal => "proposal",
            Self::ReflexOrder => "reflex-order",
        }
    }
}
impl ::std::fmt::Debug for RequestIntent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for RequestIntent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for RequestIntent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for RequestIntent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<RequestIntent> for Coding {
    fn from(code: RequestIntent) -> Self {
        CodingInner {
            system: Some("http://hl7.org/fhir/ValueSet/request-intent".to_owned()),
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
impl From<RequestIntent> for CodeableConcept {
    fn from(code: RequestIntent) -> Self {
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
#[doc = "**[RequestStatus](http://hl7.org/fhir/ValueSet/request-status)**. Codes identifying the lifecycle stage of a request.\n\nFHIR version: 5.0.0."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum RequestStatus {
    /** **active**

Active. The request is in force and ready to be acted upon. */
    Active,
    /** **completed**

Completed. The activity described by the request has been fully performed.  No further activity will occur. */
    Completed,
    /** **draft**

Draft. The request has been created but is not yet complete or ready for action. */
    Draft,
    /** **entered-in-error**

Entered in Error. This request should never have existed and should be considered 'void'.  (It is possible that real-world decisions were based on it.  If real-world activity has occurred, the status should be "revoked" rather than "entered-in-error".). */
    EnteredInError,
    /** **on-hold**

On Hold. The request (and any implicit authorization to act) has been temporarily withdrawn but is expected to resume in the future. */
    OnHold,
    /** **revoked**

Revoked. The request (and any implicit authorization to act) has been terminated prior to the known full completion of the intended actions.  No further activity should occur. */
    Revoked,
    /** **unknown**

Unknown. The authoring/source system does not know which of the status values currently applies for this request.  Note: This concept is not to be used for "other" - one of the listed statuses is presumed to apply,  but the authoring/source system does not know which. */
    Unknown,
}
impl ::core::str::FromStr for RequestStatus {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "completed" => Ok(Self::Completed),
            "draft" => Ok(Self::Draft),
            "entered-in-error" => Ok(Self::EnteredInError),
            "on-hold" => Ok(Self::OnHold),
            "revoked" => Ok(Self::Revoked),
            "unknown" => Ok(Self::Unknown),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for RequestStatus {
    fn as_ref(&self) -> &str {
        match self {
            Self::Active => "active",
            Self::Completed => "completed",
            Self::Draft => "draft",
            Self::EnteredInError => "entered-in-error",
            Self::OnHold => "on-hold",
            Self::Revoked => "revoked",
            Self::Unknown => "unknown",
        }
    }
}
impl ::std::fmt::Debug for RequestStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for RequestStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for RequestStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for RequestStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<RequestStatus> for Coding {
    fn from(code: RequestStatus) -> Self {
        CodingInner {
            system: Some("http://hl7.org/fhir/ValueSet/request-status".to_owned()),
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
impl From<RequestStatus> for CodeableConcept {
    fn from(code: RequestStatus) -> Self {
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
#[doc = "**[SearchComparator](http://hl7.org/fhir/ValueSet/search-comparator)**. What Search Comparator Codes are supported in search.\n\nFHIR version: 5.0.0."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum SearchComparator {
    /** **ap**

Approximately. the value for the parameter in the resource is approximately the same to the provided value. */
    Ap,
    /** **eb**

Ends Before. the value for the parameter in the resource ends before the provided value. */
    Eb,
    /** **eq**

Equals. the value for the parameter in the resource is equal to the provided value. */
    Eq,
    /** **ge**

Greater or Equals. the value for the parameter in the resource is greater or equal to the provided value. */
    Ge,
    /** **gt**

Greater Than. the value for the parameter in the resource is greater than the provided value. */
    Gt,
    /** **le**

Less of Equal. the value for the parameter in the resource is less or equal to the provided value. */
    Le,
    /** **lt**

Less Than. the value for the parameter in the resource is less than the provided value. */
    Lt,
    /** **ne**

Not Equals. the value for the parameter in the resource is not equal to the provided value. */
    Ne,
    /** **sa**

Starts After. the value for the parameter in the resource starts after the provided value. */
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
#[doc = "**[SearchEntryMode](http://hl7.org/fhir/ValueSet/search-entry-mode)**. Why an entry is in the result set - whether it's included as a match or because of an _include requirement, or to convey information or warning information about the search process.\n\nFHIR version: 5.0.0."]
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
#[doc = "**[SearchModifierCode](http://hl7.org/fhir/ValueSet/search-modifier-code)**. A supported modifier for a search parameter.\n\nFHIR version: 5.0.0."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum SearchModifierCode {
    /** **above**

Above. The search parameter tests whether the value in a resource subsumes the specified value (is-a, or hierarchical relationships). */
    Above,
    /** **below**

Below. The search parameter tests whether the value in a resource is subsumed by the specified value (is-a, or hierarchical relationships). */
    Below,
    /** **code-text**

Code Text. Tests whether the textual display value in a resource (e.g., CodeableConcept.text, Coding.display, or Reference.display) matches the supplied parameter value. */
    CodeText,
    /** **contains**

Contains. The search parameter returns resources that include the supplied parameter value anywhere within the field being searched. */
    Contains,
    /** **exact**

Exact. The search parameter returns resources that have a value that exactly matches the supplied parameter (the whole string, including casing and accents). */
    Exact,
    /** **identifier**

Identifier. The search parameter applies to the identifier on the resource, not the reference. */
    Identifier,
    /** **in**

In. The search parameter is a URI (relative or absolute) that identifies a value set, and the search parameter tests whether the coding is in the specified value set. */
    In,
    /** **iterate**

Iterate. The search parameter indicates an inclusion directive (_include, _revinclude) that is applied to an included resource instead of the matching resource. */
    Iterate,
    /** **missing**

Missing. The search parameter returns resources that have a value or not. */
    Missing,
    /** **not**

Not. The search parameter returns resources that do not contain a match. */
    Not,
    /** **not-in**

Not In. The search parameter is a URI (relative or absolute) that identifies a value set, and the search parameter tests whether the coding is not in the specified value set. */
    NotIn,
    /** **of-type**

Of Type. The search parameter has the format system|code|value, where the system and code refer to an Identifier.type.coding.system and .code, and match if any of the type codes match. All 3 parts must be present. */
    OfType,
    /** **text**

Text. The search parameter is processed as a string that searches text associated with the code/value - either CodeableConcept.text, Coding.display, Identifier.type.text, or Reference.display. */
    Text,
    /** **text-advanced**

Text Advanced. Tests whether the value in a resource matches the supplied parameter value using advanced text handling that searches text associated with the code/value - e.g., CodeableConcept.text, Coding.display, or Identifier.type.text. */
    TextAdvanced,
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
            "code-text" => Ok(Self::CodeText),
            "contains" => Ok(Self::Contains),
            "exact" => Ok(Self::Exact),
            "identifier" => Ok(Self::Identifier),
            "in" => Ok(Self::In),
            "iterate" => Ok(Self::Iterate),
            "missing" => Ok(Self::Missing),
            "not" => Ok(Self::Not),
            "not-in" => Ok(Self::NotIn),
            "of-type" => Ok(Self::OfType),
            "text" => Ok(Self::Text),
            "text-advanced" => Ok(Self::TextAdvanced),
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
            Self::CodeText => "code-text",
            Self::Contains => "contains",
            Self::Exact => "exact",
            Self::Identifier => "identifier",
            Self::In => "in",
            Self::Iterate => "iterate",
            Self::Missing => "missing",
            Self::Not => "not",
            Self::NotIn => "not-in",
            Self::OfType => "of-type",
            Self::Text => "text",
            Self::TextAdvanced => "text-advanced",
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
#[doc = "**[SubscriptionPayloadContent](http://hl7.org/fhir/ValueSet/subscription-payload-content)**. Codes to represent how much resource content to send in the notification payload.\n\nFHIR version: 5.0.0."]
#[derive(PartialEq, Eq, Hash, Clone)]
#[derive(Copy)]
pub enum SubscriptionPayloadContent {
    /** **empty**

Empty. No resource content is transacted in the notification payload. */
    Empty,
    /** **full-resource**

Full-resource. The entire resource is transacted in the notification payload. */
    FullResource,
    /** **id-only**

Id-only. Only the resource id is transacted in the notification payload. */
    IdOnly,
}
impl ::core::str::FromStr for SubscriptionPayloadContent {
    type Err = String;
    #[allow(clippy::match_single_binding)]
    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s {
            "empty" => Ok(Self::Empty),
            "full-resource" => Ok(Self::FullResource),
            "id-only" => Ok(Self::IdOnly),
            _ => Err(format!("Unknown value: {s}")),
        }
    }
}
impl AsRef<str> for SubscriptionPayloadContent {
    fn as_ref(&self) -> &str {
        match self {
            Self::Empty => "empty",
            Self::FullResource => "full-resource",
            Self::IdOnly => "id-only",
        }
    }
}
impl ::std::fmt::Debug for SubscriptionPayloadContent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl ::std::fmt::Display for SubscriptionPayloadContent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_ref())
    }
}
impl<'de> Deserialize<'de> for SubscriptionPayloadContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        string.parse().map_err(D::Error::custom)
    }
}
impl Serialize for SubscriptionPayloadContent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}
impl From<SubscriptionPayloadContent> for Coding {
    fn from(code: SubscriptionPayloadContent) -> Self {
        CodingInner {
            system: Some(
                "http://hl7.org/fhir/ValueSet/subscription-payload-content".to_owned(),
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
impl From<SubscriptionPayloadContent> for CodeableConcept {
    fn from(code: SubscriptionPayloadContent) -> Self {
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
