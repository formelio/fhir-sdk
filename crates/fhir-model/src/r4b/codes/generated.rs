//! Generated code! Take a look at the generator-crate for changing this file!
#![allow(clippy::too_many_lines)]
use std::hash::Hash;
use serde::{Serialize, Deserialize};
use super::super::types::{Coding, CodingInner, CodeableConcept, CodeableConceptInner};
#[doc = "**[BundleType](http://hl7.org/fhir/ValueSet/bundle-type)**. Indicates the purpose of a bundle - how it is intended to be used.\n\nFHIR version: 4.3.0."]
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
#[doc = "**[HTTPVerb](http://hl7.org/fhir/ValueSet/http-verb)**. HTTP verbs (in the HTTP command line). See [HTTP rfc](https://tools.ietf.org/html/rfc7231) for details.\n\nFHIR version: 4.3.0."]
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
#[doc = "**[SearchComparator](http://hl7.org/fhir/ValueSet/search-comparator)**. What Search Comparator Codes are supported in search.\n\nFHIR version: 4.3.0."]
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
#[doc = "**[SearchEntryMode](http://hl7.org/fhir/ValueSet/search-entry-mode)**. Why an entry is in the result set - whether it's included as a match or because of an _include requirement, or to convey information or warning information about the search process.\n\nFHIR version: 4.3.0."]
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
#[doc = "**[SearchModifierCode](http://hl7.org/fhir/ValueSet/search-modifier-code)**. A supported modifier for a search parameter.\n\nFHIR version: 4.3.0."]
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
    /** **identifier**

Identifier. The search parameter applies to the identifier on the resource, not the reference. */
    Identifier,
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
    /** **ofType**

Of Type. The search parameter has the format system|code|value, where the system and code refer to an Identifier.type.coding.system and .code, and match if any of the type codes match. All 3 parts must be present. */
    OfType,
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
            "identifier" => Ok(Self::Identifier),
            "in" => Ok(Self::In),
            "missing" => Ok(Self::Missing),
            "not" => Ok(Self::Not),
            "not-in" => Ok(Self::NotIn),
            "ofType" => Ok(Self::OfType),
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
            Self::Identifier => "identifier",
            Self::In => "in",
            Self::Missing => "missing",
            Self::Not => "not",
            Self::NotIn => "not-in",
            Self::OfType => "ofType",
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
