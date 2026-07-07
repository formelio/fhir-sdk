//! Generated code! Take a look at the generator-crate for changing this file!
#![allow(clippy::too_many_lines)]
use crate::params::*;
use super::super::resources;
/// Search parameters for the ActivityDefinition resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ActivityDefinitionSearchParameter {
    /// What resource is being referenced
    ComposedOf,
    /// The activity definition publication date
    Date,
    /// What resource is being referenced
    DependsOn,
    /// What resource is being referenced
    DerivedFrom,
    /// The description of the activity definition
    Description,
    /// The time during which the activity definition is intended to be in use
    Effective,
    /// External identifier for the activity definition
    Identifier,
    /// Intended jurisdiction for the activity definition
    Jurisdiction,
    /// Computationally friendly name of the activity definition
    Name,
    /// What resource is being referenced
    Predecessor,
    /// Name of the publisher of the activity definition
    Publisher,
    /// The current status of the activity definition
    Status,
    /// What resource is being referenced
    Successor,
    /// The human-friendly name of the activity definition
    Title,
    /// Topics associated with the module
    Topic,
    /// The uri that identifies the activity definition
    Url,
    /// The business version of the activity definition
    Version,
}
impl ResourceSearchParameterDefinition for ActivityDefinitionSearchParameter {
    fn resource_type(&self) -> &'static str {
        "ActivityDefinition"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::ComposedOf => "composed-of",
            Self::Date => "date",
            Self::DependsOn => "depends-on",
            Self::DerivedFrom => "derived-from",
            Self::Description => "description",
            Self::Effective => "effective",
            Self::Identifier => "identifier",
            Self::Jurisdiction => "jurisdiction",
            Self::Name => "name",
            Self::Predecessor => "predecessor",
            Self::Publisher => "publisher",
            Self::Status => "status",
            Self::Successor => "successor",
            Self::Title => "title",
            Self::Topic => "topic",
            Self::Url => "url",
            Self::Version => "version",
        }
    }
}
/// Search parameters for the AdverseEvent resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AdverseEventSearchParameter {
    /** AE | PAE
An adverse event is an event that caused harm to a patient,  an adverse reaction is a something that is a subject-specific event that is a result of an exposure to a medication, food, device or environmental substance, a potential adverse event is something that occurred and that could have caused harm to a patient but did not*/
    Category,
    /// When the event occurred
    Date,
    /// Location where adverse event occurred
    Location,
    /// Adverse Reaction Events linked to exposure to substance
    Reaction,
    /// Who recorded the adverse event
    Recorder,
    /// Mild | Moderate | Severe
    Seriousness,
    /// AdverseEvent.study
    Study,
    /// Subject or group impacted by event
    Subject,
    /// Refers to the specific entity that caused the adverse event
    Substance,
    /// actual | potential
    Type,
}
impl ResourceSearchParameterDefinition for AdverseEventSearchParameter {
    fn resource_type(&self) -> &'static str {
        "AdverseEvent"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Category => "category",
            Self::Date => "date",
            Self::Location => "location",
            Self::Reaction => "reaction",
            Self::Recorder => "recorder",
            Self::Seriousness => "seriousness",
            Self::Study => "study",
            Self::Subject => "subject",
            Self::Substance => "substance",
            Self::Type => "type",
        }
    }
}
/// Search parameters for the AllergyIntolerance resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AllergyIntoleranceSearchParameter {
    /// Source of the information about the allergy
    Asserter,
    /// food | medication | environment | biologic
    Category,
    /// active | inactive | resolved
    ClinicalStatus,
    /// Code that identifies the allergy or intolerance
    Code,
    /// low | high | unable-to-assess
    Criticality,
    /// Date record was believed accurate
    Date,
    /// External ids for this item
    Identifier,
    /// Date(/time) of last known occurrence of a reaction
    LastDate,
    /// Clinical symptoms/signs associated with the Event
    Manifestation,
    /// Date(/time) when manifestations showed
    Onset,
    /// Who the sensitivity is for
    Patient,
    /// Who recorded the sensitivity
    Recorder,
    /// How the subject was exposed to the substance
    Route,
    /// mild | moderate | severe (of event as a whole)
    Severity,
    /// allergy | intolerance - Underlying mechanism (if known)
    Type,
    /// unconfirmed | confirmed | refuted | entered-in-error
    VerificationStatus,
}
impl ResourceSearchParameterDefinition for AllergyIntoleranceSearchParameter {
    fn resource_type(&self) -> &'static str {
        "AllergyIntolerance"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Asserter => "asserter",
            Self::Category => "category",
            Self::ClinicalStatus => "clinical-status",
            Self::Code => "code",
            Self::Criticality => "criticality",
            Self::Date => "date",
            Self::Identifier => "identifier",
            Self::LastDate => "last-date",
            Self::Manifestation => "manifestation",
            Self::Onset => "onset",
            Self::Patient => "patient",
            Self::Recorder => "recorder",
            Self::Route => "route",
            Self::Severity => "severity",
            Self::Type => "type",
            Self::VerificationStatus => "verification-status",
        }
    }
}
/// Search parameters for the Appointment resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AppointmentSearchParameter {
    /// Any one of the individuals participating in the appointment
    Actor,
    /// The style of appointment or patient that has been booked in the slot (not service type)
    AppointmentType,
    /// Appointment date/time.
    Date,
    /// An Identifier of the Appointment
    Identifier,
    /// The ReferralRequest provided as information to allocate to the Encounter
    Incomingreferral,
    /// This location is listed in the participants of the appointment
    Location,
    /// The Participation status of the subject, or other participant on the appointment. Can be used to locate participants that have not responded to meeting requests.
    PartStatus,
    /// One of the individuals of the appointment is this patient
    Patient,
    /// One of the individuals of the appointment is this practitioner
    Practitioner,
    /// The specific service that is to be performed during this appointment
    ServiceType,
    /// The overall status of the appointment
    Status,
}
impl ResourceSearchParameterDefinition for AppointmentSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Appointment"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Actor => "actor",
            Self::AppointmentType => "appointment-type",
            Self::Date => "date",
            Self::Identifier => "identifier",
            Self::Incomingreferral => "incomingreferral",
            Self::Location => "location",
            Self::PartStatus => "part-status",
            Self::Patient => "patient",
            Self::Practitioner => "practitioner",
            Self::ServiceType => "service-type",
            Self::Status => "status",
        }
    }
}
/// Search parameters for the AuditEvent resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AuditEventSearchParameter {
    /// Type of action performed during the event
    Action,
    /// Identifier for the network access point of the user device
    Address,
    /// Direct reference to resource
    Agent,
    /// Human-meaningful name for the agent
    AgentName,
    /// Agent role in the event
    AgentRole,
    /// Alternative User id e.g. authentication
    Altid,
    /// Time when the event occurred on source
    Date,
    /// Specific instance of resource
    Entity,
    /// Specific instance of object
    EntityId,
    /// Descriptor for entity
    EntityName,
    /// What role the entity played
    EntityRole,
    /// Type of entity involved
    EntityType,
    /// Whether the event succeeded or failed
    Outcome,
    /// Direct reference to resource
    Patient,
    /// Policy that authorized event
    Policy,
    /// Logical source location within the enterprise
    Site,
    /// The identity of source detecting the event
    Source,
    /// More specific type/id for the event
    Subtype,
    /// Type/identifier of event
    Type,
    /// Unique identifier for the user
    User,
}
impl ResourceSearchParameterDefinition for AuditEventSearchParameter {
    fn resource_type(&self) -> &'static str {
        "AuditEvent"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Action => "action",
            Self::Address => "address",
            Self::Agent => "agent",
            Self::AgentName => "agent-name",
            Self::AgentRole => "agent-role",
            Self::Altid => "altid",
            Self::Date => "date",
            Self::Entity => "entity",
            Self::EntityId => "entity-id",
            Self::EntityName => "entity-name",
            Self::EntityRole => "entity-role",
            Self::EntityType => "entity-type",
            Self::Outcome => "outcome",
            Self::Patient => "patient",
            Self::Policy => "policy",
            Self::Site => "site",
            Self::Source => "source",
            Self::Subtype => "subtype",
            Self::Type => "type",
            Self::User => "user",
        }
    }
}
/// Search parameters for the Binary resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BinarySearchParameter {
    /// MimeType of the binary content
    Contenttype,
}
impl ResourceSearchParameterDefinition for BinarySearchParameter {
    fn resource_type(&self) -> &'static str {
        "Binary"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Contenttype => "contenttype",
        }
    }
}
/// Search parameters for the BodySite resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BodySiteSearchParameter {
    /// Named anatomical location
    Code,
    /// Identifier for this instance of the anatomical location
    Identifier,
    /// Patient to whom bodysite belongs
    Patient,
}
impl ResourceSearchParameterDefinition for BodySiteSearchParameter {
    fn resource_type(&self) -> &'static str {
        "BodySite"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Code => "code",
            Self::Identifier => "identifier",
            Self::Patient => "patient",
        }
    }
}
/// Search parameters for the Bundle resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BundleSearchParameter {
    /// The first resource in the bundle, if the bundle type is "document" - this is a composition, and this parameter provides access to searches its contents
    Composition,
    /// Persistent identifier for the bundle
    Identifier,
    /// The first resource in the bundle, if the bundle type is "message" - this is a message header, and this parameter provides access to search its contents
    Message,
    /// document | message | transaction | transaction-response | batch | batch-response | history | searchset | collection
    Type,
}
impl ResourceSearchParameterDefinition for BundleSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Bundle"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Composition => "composition",
            Self::Identifier => "identifier",
            Self::Message => "message",
            Self::Type => "type",
        }
    }
}
/// Search parameters for the CapabilityStatement resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CapabilityStatementSearchParameter {
    /// The capability statement publication date
    Date,
    /// The description of the capability statement
    Description,
    /// Event code in a capability statement
    Event,
    /// The version of FHIR
    Fhirversion,
    /// formats supported (xml | json | ttl | mime type)
    Format,
    /// Implementation guides supported
    Guide,
    /// Intended jurisdiction for the capability statement
    Jurisdiction,
    /// Mode - restful (server/client) or messaging (sender/receiver)
    Mode,
    /// Computationally friendly name of the capability statement
    Name,
    /// Name of the publisher of the capability statement
    Publisher,
    /// Name of a resource mentioned in a capability statement
    Resource,
    /// A profile id invoked in a capability statement
    ResourceProfile,
    /// OAuth | SMART-on-FHIR | NTLM | Basic | Kerberos | Certificates
    SecurityService,
    /// Part of a the name of a software application
    Software,
    /// The current status of the capability statement
    Status,
    /// Profiles for use cases supported
    SupportedProfile,
    /// The human-friendly name of the capability statement
    Title,
    /// The uri that identifies the capability statement
    Url,
    /// The business version of the capability statement
    Version,
}
impl ResourceSearchParameterDefinition for CapabilityStatementSearchParameter {
    fn resource_type(&self) -> &'static str {
        "CapabilityStatement"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Date => "date",
            Self::Description => "description",
            Self::Event => "event",
            Self::Fhirversion => "fhirversion",
            Self::Format => "format",
            Self::Guide => "guide",
            Self::Jurisdiction => "jurisdiction",
            Self::Mode => "mode",
            Self::Name => "name",
            Self::Publisher => "publisher",
            Self::Resource => "resource",
            Self::ResourceProfile => "resource-profile",
            Self::SecurityService => "security-service",
            Self::Software => "software",
            Self::Status => "status",
            Self::SupportedProfile => "supported-profile",
            Self::Title => "title",
            Self::Url => "url",
            Self::Version => "version",
        }
    }
}
/// Search parameters for the CarePlan resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CarePlanSearchParameter {
    /// Time period plan covers
    Date,
    /// External Ids for this plan
    Identifier,
    /// Who care plan is for
    Patient,
    /// Detail type of activity
    ActivityCode,
    /// Specified date occurs within period specified by CarePlan.activity.timingSchedule
    ActivityDate,
    /// Activity details defined in specific resource
    ActivityReference,
    /// Fulfills care plan
    BasedOn,
    /// Who's involved in plan?
    CareTeam,
    /// Type of plan
    Category,
    /// Health issues this plan addresses
    Condition,
    /// Created in context of
    Context,
    /// Protocol or definition
    Definition,
    /// Created in context of
    Encounter,
    /// Desired outcome of plan
    Goal,
    /// proposal | plan | order | option
    Intent,
    /// Part of referenced CarePlan
    PartOf,
    /// Matches if the practitioner is listed as a performer in any of the "simple" activities.  (For performers of the detailed activities, chain through the activitydetail search parameter.)
    Performer,
    /// CarePlan replaced by this CarePlan
    Replaces,
    /// draft | active | suspended | completed | entered-in-error | cancelled | unknown
    Status,
    /// Who care plan is for
    Subject,
}
impl ResourceSearchParameterDefinition for CarePlanSearchParameter {
    fn resource_type(&self) -> &'static str {
        "CarePlan"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Date => "date",
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::ActivityCode => "activity-code",
            Self::ActivityDate => "activity-date",
            Self::ActivityReference => "activity-reference",
            Self::BasedOn => "based-on",
            Self::CareTeam => "care-team",
            Self::Category => "category",
            Self::Condition => "condition",
            Self::Context => "context",
            Self::Definition => "definition",
            Self::Encounter => "encounter",
            Self::Goal => "goal",
            Self::Intent => "intent",
            Self::PartOf => "part-of",
            Self::Performer => "performer",
            Self::Replaces => "replaces",
            Self::Status => "status",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the CareTeam resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CareTeamSearchParameter {
    /// Time period team covers
    Date,
    /// External Ids for this team
    Identifier,
    /// Who care team is for
    Patient,
    /// Type of team
    Category,
    /// Encounter or episode associated with CareTeam
    Context,
    /// Encounter or episode associated with CareTeam
    Encounter,
    /// Who is involved
    Participant,
    /// proposed | active | suspended | inactive | entered-in-error
    Status,
    /// Who care team is for
    Subject,
}
impl ResourceSearchParameterDefinition for CareTeamSearchParameter {
    fn resource_type(&self) -> &'static str {
        "CareTeam"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Date => "date",
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Category => "category",
            Self::Context => "context",
            Self::Encounter => "encounter",
            Self::Participant => "participant",
            Self::Status => "status",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the ChargeItem resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ChargeItemSearchParameter {
    /// Account to place this charge
    Account,
    /// A code that identifies the charge, like a billing code
    Code,
    /// Encounter / Episode associated with event
    Context,
    /// Date the charge item was entered
    EnteredDate,
    /// Individual who was entering
    Enterer,
    /// Factor overriding the associated rules
    FactorOverride,
    /// Business Identifier for item
    Identifier,
    /// When the charged service was applied
    Occurrence,
    /// Individual who was performing
    ParticipantActor,
    /// What type of performance was done
    ParticipantRole,
    /// Individual service was done for/to
    Patient,
    /// Organization providing the charged sevice
    PerformingOrganization,
    /// Price overriding the associated rules
    PriceOverride,
    /// Quantity of which the charge item has been serviced
    Quantity,
    /// Organization requesting the charged service
    RequestingOrganization,
    /// Which rendered service is being charged?
    Service,
    /// Individual service was done for/to
    Subject,
}
impl ResourceSearchParameterDefinition for ChargeItemSearchParameter {
    fn resource_type(&self) -> &'static str {
        "ChargeItem"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Account => "account",
            Self::Code => "code",
            Self::Context => "context",
            Self::EnteredDate => "entered-date",
            Self::Enterer => "enterer",
            Self::FactorOverride => "factor-override",
            Self::Identifier => "identifier",
            Self::Occurrence => "occurrence",
            Self::ParticipantActor => "participant-actor",
            Self::ParticipantRole => "participant-role",
            Self::Patient => "patient",
            Self::PerformingOrganization => "performing-organization",
            Self::PriceOverride => "price-override",
            Self::Quantity => "quantity",
            Self::RequestingOrganization => "requesting-organization",
            Self::Service => "service",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the CodeSystem resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CodeSystemSearchParameter {
    /// A code defined in the code system
    Code,
    /// not-present | example | fragment | complete
    ContentMode,
    /// The code system publication date
    Date,
    /// The description of the code system
    Description,
    /// External identifier for the code system
    Identifier,
    /// Intended jurisdiction for the code system
    Jurisdiction,
    /// A language in which a designation is provided
    Language,
    /// Computationally friendly name of the code system
    Name,
    /// Name of the publisher of the code system
    Publisher,
    /// The current status of the code system
    Status,
    /// The system for any codes defined by this code system (same as 'url')
    System,
    /// The human-friendly name of the code system
    Title,
    /// The uri that identifies the code system
    Url,
    /// The business version of the code system
    Version,
}
impl ResourceSearchParameterDefinition for CodeSystemSearchParameter {
    fn resource_type(&self) -> &'static str {
        "CodeSystem"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Code => "code",
            Self::ContentMode => "content-mode",
            Self::Date => "date",
            Self::Description => "description",
            Self::Identifier => "identifier",
            Self::Jurisdiction => "jurisdiction",
            Self::Language => "language",
            Self::Name => "name",
            Self::Publisher => "publisher",
            Self::Status => "status",
            Self::System => "system",
            Self::Title => "title",
            Self::Url => "url",
            Self::Version => "version",
        }
    }
}
/// Search parameters for the CompartmentDefinition resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CompartmentDefinitionSearchParameter {
    /// Patient | Encounter | RelatedPerson | Practitioner | Device
    Code,
    /// The compartment definition publication date
    Date,
    /// The description of the compartment definition
    Description,
    /// Intended jurisdiction for the compartment definition
    Jurisdiction,
    /// Computationally friendly name of the compartment definition
    Name,
    /// Name of the publisher of the compartment definition
    Publisher,
    /// Name of resource type
    Resource,
    /// The current status of the compartment definition
    Status,
    /// The human-friendly name of the compartment definition
    Title,
    /// The uri that identifies the compartment definition
    Url,
}
impl ResourceSearchParameterDefinition for CompartmentDefinitionSearchParameter {
    fn resource_type(&self) -> &'static str {
        "CompartmentDefinition"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Code => "code",
            Self::Date => "date",
            Self::Description => "description",
            Self::Jurisdiction => "jurisdiction",
            Self::Name => "name",
            Self::Publisher => "publisher",
            Self::Resource => "resource",
            Self::Status => "status",
            Self::Title => "title",
            Self::Url => "url",
        }
    }
}
/// Search parameters for the Composition resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CompositionSearchParameter {
    /// Composition editing time
    Date,
    /// Logical identifier of composition (version-independent)
    Identifier,
    /// Who and/or what the composition is about
    Patient,
    /// Kind of composition (LOINC if possible)
    Type,
    /// Who attested the composition
    Attester,
    /// Who and/or what authored the composition
    Author,
    /// Categorization of Composition
    Class,
    /// As defined by affinity domain
    Confidentiality,
    /// Code(s) that apply to the event being documented
    Context,
    /// Context of the Composition
    Encounter,
    /// A reference to data that supports this section
    Entry,
    /// The period covered by the documentation
    Period,
    /// Target of the relationship
    RelatedId,
    /// Target of the relationship
    RelatedRef,
    /// Classification of section (recommended)
    Section,
    /// preliminary | final | amended | entered-in-error
    Status,
    /// Who and/or what the composition is about
    Subject,
    /// Human Readable name/title
    Title,
}
impl ResourceSearchParameterDefinition for CompositionSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Composition"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Date => "date",
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Type => "type",
            Self::Attester => "attester",
            Self::Author => "author",
            Self::Class => "class",
            Self::Confidentiality => "confidentiality",
            Self::Context => "context",
            Self::Encounter => "encounter",
            Self::Entry => "entry",
            Self::Period => "period",
            Self::RelatedId => "related-id",
            Self::RelatedRef => "related-ref",
            Self::Section => "section",
            Self::Status => "status",
            Self::Subject => "subject",
            Self::Title => "title",
        }
    }
}
/// Search parameters for the ConceptMap resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ConceptMapSearchParameter {
    /// The concept map publication date
    Date,
    /// Reference to property mapping depends on
    Dependson,
    /// The description of the concept map
    Description,
    /// External identifier for the concept map
    Identifier,
    /// Intended jurisdiction for the concept map
    Jurisdiction,
    /// Computationally friendly name of the concept map
    Name,
    /// Canonical URL for other concept map
    Other,
    /// Reference to property mapping depends on
    Product,
    /// Name of the publisher of the concept map
    Publisher,
    /// Identifies the source of the concepts which are being mapped
    Source,
    /// Identifies element being mapped
    SourceCode,
    /// Code System (if value set crosses code systems)
    SourceSystem,
    /// Identifies the source of the concepts which are being mapped
    SourceUri,
    /// The current status of the concept map
    Status,
    /// Provides context to the mappings
    Target,
    /// Code that identifies the target element
    TargetCode,
    /// System of the target (if necessary)
    TargetSystem,
    /// Provides context to the mappings
    TargetUri,
    /// The human-friendly name of the concept map
    Title,
    /// The uri that identifies the concept map
    Url,
    /// The business version of the concept map
    Version,
}
impl ResourceSearchParameterDefinition for ConceptMapSearchParameter {
    fn resource_type(&self) -> &'static str {
        "ConceptMap"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Date => "date",
            Self::Dependson => "dependson",
            Self::Description => "description",
            Self::Identifier => "identifier",
            Self::Jurisdiction => "jurisdiction",
            Self::Name => "name",
            Self::Other => "other",
            Self::Product => "product",
            Self::Publisher => "publisher",
            Self::Source => "source",
            Self::SourceCode => "source-code",
            Self::SourceSystem => "source-system",
            Self::SourceUri => "source-uri",
            Self::Status => "status",
            Self::Target => "target",
            Self::TargetCode => "target-code",
            Self::TargetSystem => "target-system",
            Self::TargetUri => "target-uri",
            Self::Title => "title",
            Self::Url => "url",
            Self::Version => "version",
        }
    }
}
/// Search parameters for the Condition resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ConditionSearchParameter {
    /// Code for the condition
    Code,
    /// A unique identifier of the condition record
    Identifier,
    /// Who has the condition?
    Patient,
    /// Abatement as age or age range
    AbatementAge,
    /// Abatement boolean (boolean is true or non-boolean values are present)
    AbatementBoolean,
    /// Date-related abatements (dateTime and period)
    AbatementDate,
    /// Abatement as a string
    AbatementString,
    /// Date record was believed accurate
    AssertedDate,
    /// Person who asserts this condition
    Asserter,
    /// Anatomical location, if relevant
    BodySite,
    /// The category of the condition
    Category,
    /// The clinical status of the condition
    ClinicalStatus,
    /// Encounter or episode when condition first asserted
    Context,
    /// Encounter when condition first asserted
    Encounter,
    /// Manifestation/symptom
    Evidence,
    /// Supporting information found elsewhere
    EvidenceDetail,
    /// Onsets as age or age range
    OnsetAge,
    /// Date related onsets (dateTime and Period)
    OnsetDate,
    /// Onsets as a string
    OnsetInfo,
    /// The severity of the condition
    Severity,
    /// Simple summary (disease specific)
    Stage,
    /// Who has the condition?
    Subject,
    /// provisional | differential | confirmed | refuted | entered-in-error | unknown
    VerificationStatus,
}
impl ResourceSearchParameterDefinition for ConditionSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Condition"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Code => "code",
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::AbatementAge => "abatement-age",
            Self::AbatementBoolean => "abatement-boolean",
            Self::AbatementDate => "abatement-date",
            Self::AbatementString => "abatement-string",
            Self::AssertedDate => "asserted-date",
            Self::Asserter => "asserter",
            Self::BodySite => "body-site",
            Self::Category => "category",
            Self::ClinicalStatus => "clinical-status",
            Self::Context => "context",
            Self::Encounter => "encounter",
            Self::Evidence => "evidence",
            Self::EvidenceDetail => "evidence-detail",
            Self::OnsetAge => "onset-age",
            Self::OnsetDate => "onset-date",
            Self::OnsetInfo => "onset-info",
            Self::Severity => "severity",
            Self::Stage => "stage",
            Self::Subject => "subject",
            Self::VerificationStatus => "verification-status",
        }
    }
}
/// Search parameters for the Consent resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ConsentSearchParameter {
    /// When this Consent was created or indexed
    Date,
    /// Identifier for this record (external references)
    Identifier,
    /// Who the consent applies to
    Patient,
    /// Actions controlled by this consent
    Action,
    /// Resource for the actor (or group, by role)
    Actor,
    /// Classification of the consent statement - for indexing/retrieval
    Category,
    /// Who is agreeing to the policy and exceptions
    Consentor,
    /// The actual data reference
    Data,
    /// Custodian of the consent
    Organization,
    /// Period that this consent applies
    Period,
    /// Context of activities for which the agreement is made
    Purpose,
    /// Security Labels that define affected resources
    Securitylabel,
    /// Source from which this consent is taken
    Source,
    /// draft | proposed | active | rejected | inactive | entered-in-error
    Status,
}
impl ResourceSearchParameterDefinition for ConsentSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Consent"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Date => "date",
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Action => "action",
            Self::Actor => "actor",
            Self::Category => "category",
            Self::Consentor => "consentor",
            Self::Data => "data",
            Self::Organization => "organization",
            Self::Period => "period",
            Self::Purpose => "purpose",
            Self::Securitylabel => "securitylabel",
            Self::Source => "source",
            Self::Status => "status",
        }
    }
}
/// Search parameters for the Contract resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ContractSearchParameter {
    /// Agent to the Contact
    Agent,
    /// The authority of the contract
    Authority,
    /// The domain of the contract
    Domain,
    /// The identity of the contract
    Identifier,
    /// The date/time the contract was issued
    Issued,
    /// The identity of the subject of the contract (if a patient)
    Patient,
    /// Contract Signatory Party
    Signer,
    /// The identity of the subject of the contract
    Subject,
    /// The identity of the topic of the contract terms
    TermTopic,
}
impl ResourceSearchParameterDefinition for ContractSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Contract"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Agent => "agent",
            Self::Authority => "authority",
            Self::Domain => "domain",
            Self::Identifier => "identifier",
            Self::Issued => "issued",
            Self::Patient => "patient",
            Self::Signer => "signer",
            Self::Subject => "subject",
            Self::TermTopic => "term-topic",
        }
    }
}
/// Search parameters for the Coverage resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CoverageSearchParameter {
    /// Covered party
    Beneficiary,
    /// Class identifier
    Class,
    /// Dependent number
    Dependent,
    /// Group identifier
    Group,
    /// The primary identifier of the insured and the coverage
    Identifier,
    /// The identity of the insurer or party paying for services
    Payor,
    /// A plan or policy identifier
    Plan,
    /// Reference to the policyholder
    PolicyHolder,
    /// Sequence number
    Sequence,
    /// Sub-class identifier
    Subclass,
    /// Sub-group identifier
    Subgroup,
    /// Sub-plan identifier
    Subplan,
    /// Reference to the subscriber
    Subscriber,
    /// The kind of coverage (health plan, auto, Workers Compensation)
    Type,
}
impl ResourceSearchParameterDefinition for CoverageSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Coverage"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Beneficiary => "beneficiary",
            Self::Class => "class",
            Self::Dependent => "dependent",
            Self::Group => "group",
            Self::Identifier => "identifier",
            Self::Payor => "payor",
            Self::Plan => "plan",
            Self::PolicyHolder => "policy-holder",
            Self::Sequence => "sequence",
            Self::Subclass => "subclass",
            Self::Subgroup => "subgroup",
            Self::Subplan => "subplan",
            Self::Subscriber => "subscriber",
            Self::Type => "type",
        }
    }
}
/// Search parameters for the DetectedIssue resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DetectedIssueSearchParameter {
    /// When identified
    Date,
    /// Unique id for the detected issue
    Identifier,
    /// Associated patient
    Patient,
    /// The provider or device that identified the issue
    Author,
    /// Issue Category, e.g. drug-drug, duplicate therapy, etc.
    Category,
    /// Problem resource
    Implicated,
}
impl ResourceSearchParameterDefinition for DetectedIssueSearchParameter {
    fn resource_type(&self) -> &'static str {
        "DetectedIssue"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Date => "date",
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Author => "author",
            Self::Category => "category",
            Self::Implicated => "implicated",
        }
    }
}
/// Search parameters for the Device resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DeviceSearchParameter {
    /// A server defined search that may match any of the string fields in the Device.udi.name  or Device.type.coding.display or  Device.type.text
    DeviceName,
    /// Instance id from manufacturer, owner, and others
    Identifier,
    /// A location, where the resource is found
    Location,
    /// The manufacturer of the device
    Manufacturer,
    /// The model of the device
    Model,
    /// The organization responsible for the device
    Organization,
    /// Patient information, if the resource is affixed to a person
    Patient,
    /// active | inactive | entered-in-error | unknown
    Status,
    /// The type of the device
    Type,
    /// UDI Barcode (RFID or other technology) string either in HRF format or AIDC format converted to base64 string.
    UdiCarrier,
    /// The udi Device Identifier (DI)
    UdiDi,
    /// Network address to contact device
    Url,
}
impl ResourceSearchParameterDefinition for DeviceSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Device"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::DeviceName => "device-name",
            Self::Identifier => "identifier",
            Self::Location => "location",
            Self::Manufacturer => "manufacturer",
            Self::Model => "model",
            Self::Organization => "organization",
            Self::Patient => "patient",
            Self::Status => "status",
            Self::Type => "type",
            Self::UdiCarrier => "udi-carrier",
            Self::UdiDi => "udi-di",
            Self::Url => "url",
        }
    }
}
/// Search parameters for the DeviceComponent resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DeviceComponentSearchParameter {
    /// The identifier of the component
    Identifier,
    /// The parent DeviceComponent resource
    Parent,
    /// The device source
    Source,
    /// The device component type
    Type,
}
impl ResourceSearchParameterDefinition for DeviceComponentSearchParameter {
    fn resource_type(&self) -> &'static str {
        "DeviceComponent"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Parent => "parent",
            Self::Source => "source",
            Self::Type => "type",
        }
    }
}
/// Search parameters for the DeviceMetric resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DeviceMetricSearchParameter {
    /// The category of the metric
    Category,
    /// The identifier of the metric
    Identifier,
    /// The parent DeviceMetric resource
    Parent,
    /// The device resource
    Source,
    /// The component type
    Type,
}
impl ResourceSearchParameterDefinition for DeviceMetricSearchParameter {
    fn resource_type(&self) -> &'static str {
        "DeviceMetric"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Category => "category",
            Self::Identifier => "identifier",
            Self::Parent => "parent",
            Self::Source => "source",
            Self::Type => "type",
        }
    }
}
/// Search parameters for the DeviceRequest resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DeviceRequestSearchParameter {
    /// Code for what is being requested/ordered
    Code,
    /// Business identifier for request/order
    Identifier,
    /// Individual the service is ordered for
    Patient,
    /// Encounter or Episode during which request was created
    Encounter,
    /// When the request transitioned to being actionable
    AuthoredOn,
    /// Plan/proposal/order fulfilled by this request
    BasedOn,
    /// Protocol or definition followed by this request
    Definition,
    /// Reference to resource that is being requested/ordered
    Device,
    /// When service should occur
    EventDate,
    /// Composite request this is part of
    GroupIdentifier,
    /// proposal | plan | original-order |reflex-order
    Intent,
    /// Desired performer for service
    Performer,
    /// Request takes the place of referenced completed or terminated requests
    Priorrequest,
    /// Who/what is requesting service 
    Requester,
    /// entered-in-error | draft | active |suspended | completed 
    Status,
    /// Individual the service is ordered for
    Subject,
}
impl ResourceSearchParameterDefinition for DeviceRequestSearchParameter {
    fn resource_type(&self) -> &'static str {
        "DeviceRequest"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Code => "code",
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Encounter => "encounter",
            Self::AuthoredOn => "authored-on",
            Self::BasedOn => "based-on",
            Self::Definition => "definition",
            Self::Device => "device",
            Self::EventDate => "event-date",
            Self::GroupIdentifier => "group-identifier",
            Self::Intent => "intent",
            Self::Performer => "performer",
            Self::Priorrequest => "priorrequest",
            Self::Requester => "requester",
            Self::Status => "status",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the DeviceUseStatement resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DeviceUseStatementSearchParameter {
    /// Search by subject - a patient
    Patient,
    /// Search by device
    Device,
    /// Search by identifier
    Identifier,
    /// Search by subject
    Subject,
}
impl ResourceSearchParameterDefinition for DeviceUseStatementSearchParameter {
    fn resource_type(&self) -> &'static str {
        "DeviceUseStatement"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Patient => "patient",
            Self::Device => "device",
            Self::Identifier => "identifier",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the DiagnosticReport resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DiagnosticReportSearchParameter {
    /// The code for the report as a whole, as opposed to codes for the atomic results, which are the names on the observation resource referred to from the result
    Code,
    /// The clinically relevant time of the report
    Date,
    /// An identifier for the report
    Identifier,
    /// The subject of the report if a patient
    Patient,
    /// The Encounter when the order was made
    Encounter,
    /// Reference to the procedure request.
    BasedOn,
    /// Which diagnostic discipline/department created the report
    Category,
    /// Healthcare event (Episode of Care or Encounter) related to the report
    Context,
    /// A coded diagnosis on the report
    Diagnosis,
    /// A reference to the image source.
    Image,
    /// When the report was issued
    Issued,
    /// Who was the source of the report (organization)
    Performer,
    /// Link to an atomic result (observation resource)
    Result,
    /// The specimen details
    Specimen,
    /// The status of the report
    Status,
    /// The subject of the report
    Subject,
}
impl ResourceSearchParameterDefinition for DiagnosticReportSearchParameter {
    fn resource_type(&self) -> &'static str {
        "DiagnosticReport"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Code => "code",
            Self::Date => "date",
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Encounter => "encounter",
            Self::BasedOn => "based-on",
            Self::Category => "category",
            Self::Context => "context",
            Self::Diagnosis => "diagnosis",
            Self::Image => "image",
            Self::Issued => "issued",
            Self::Performer => "performer",
            Self::Result => "result",
            Self::Specimen => "specimen",
            Self::Status => "status",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the DocumentReference resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DocumentReferenceSearchParameter {
    /// Master Version Specific Identifier
    Identifier,
    /// Who/what is the subject of the document
    Patient,
    /// Kind of document (LOINC if possible)
    Type,
    /// Context of the document  content
    Encounter,
    /// Who/what authenticated the document
    Authenticator,
    /// Who and/or what authored the document
    Author,
    /// Categorization of document
    Class,
    /// Document creation time
    Created,
    /// Organization which maintains the document
    Custodian,
    /// Human-readable description (title)
    Description,
    /// Main clinical acts documented
    Event,
    /// Kind of facility where patient was seen
    Facility,
    /// Format/content rules for the document
    Format,
    /// When this document reference was created
    Indexed,
    /// Human language of the content (BCP-47)
    Language,
    /// Uri where the data can be found
    Location,
    /// Time of service that is being documented
    Period,
    /// Identifier of related objects or events
    RelatedId,
    /// Related Resource
    RelatedRef,
    /// Target of the relationship
    Relatesto,
    /// replaces | transforms | signs | appends
    Relation,
    /// Combination of relation and relatesTo
    Relationship,
    /// Document security-tags
    Securitylabel,
    /// Additional details about where the content was created (e.g. clinical specialty)
    Setting,
    /// current | superseded | entered-in-error
    Status,
    /// Who/what is the subject of the document
    Subject,
}
impl ResourceSearchParameterDefinition for DocumentReferenceSearchParameter {
    fn resource_type(&self) -> &'static str {
        "DocumentReference"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Type => "type",
            Self::Encounter => "encounter",
            Self::Authenticator => "authenticator",
            Self::Author => "author",
            Self::Class => "class",
            Self::Created => "created",
            Self::Custodian => "custodian",
            Self::Description => "description",
            Self::Event => "event",
            Self::Facility => "facility",
            Self::Format => "format",
            Self::Indexed => "indexed",
            Self::Language => "language",
            Self::Location => "location",
            Self::Period => "period",
            Self::RelatedId => "related-id",
            Self::RelatedRef => "related-ref",
            Self::Relatesto => "relatesto",
            Self::Relation => "relation",
            Self::Relationship => "relationship",
            Self::Securitylabel => "securitylabel",
            Self::Setting => "setting",
            Self::Status => "status",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the Encounter resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum EncounterSearchParameter {
    /// A date within the period the Encounter lasted
    Date,
    /// Identifier(s) by which this encounter is known
    Identifier,
    /// The patient ro group present at the encounter
    Patient,
    /// Specific type of encounter
    Type,
    /// The appointment that scheduled this encounter
    Appointment,
    /// inpatient | outpatient | ambulatory | emergency +
    Class,
    /// Reason the encounter takes place (resource)
    Diagnosis,
    /// Episode(s) of care that this encounter should be recorded against
    Episodeofcare,
    /// The ReferralRequest that initiated this encounter
    Incomingreferral,
    /// Length of encounter in days
    Length,
    /// Location the encounter takes place
    Location,
    /// Time period during which the patient was present at the location
    LocationPeriod,
    /// Another Encounter this encounter is part of
    PartOf,
    /// Persons involved in the encounter other than the patient
    Participant,
    /// Role of participant in encounter
    ParticipantType,
    /// Persons involved in the encounter other than the patient
    Practitioner,
    /// Reason the encounter takes place (code)
    Reason,
    /// The custodian organization of this Encounter record
    ServiceProvider,
    /// Wheelchair, translator, stretcher, etc.
    SpecialArrangement,
    /// planned | arrived | triaged | in-progress | onleave | finished | cancelled +
    Status,
    /// The patient ro group present at the encounter
    Subject,
}
impl ResourceSearchParameterDefinition for EncounterSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Encounter"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Date => "date",
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Type => "type",
            Self::Appointment => "appointment",
            Self::Class => "class",
            Self::Diagnosis => "diagnosis",
            Self::Episodeofcare => "episodeofcare",
            Self::Incomingreferral => "incomingreferral",
            Self::Length => "length",
            Self::Location => "location",
            Self::LocationPeriod => "location-period",
            Self::PartOf => "part-of",
            Self::Participant => "participant",
            Self::ParticipantType => "participant-type",
            Self::Practitioner => "practitioner",
            Self::Reason => "reason",
            Self::ServiceProvider => "service-provider",
            Self::SpecialArrangement => "special-arrangement",
            Self::Status => "status",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the EpisodeOfCare resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum EpisodeOfCareSearchParameter {
    /// The provided date search value falls within the episode of care's period
    Date,
    /// Business Identifier(s) relevant for this EpisodeOfCare
    Identifier,
    /// The patient who is the focus of this episode of care
    Patient,
    /// Type/class  - e.g. specialist referral, disease management
    Type,
    /// Care manager/care co-ordinator for the patient
    CareManager,
    /// Conditions/problems/diagnoses this episode of care is for
    Condition,
    /// Incoming Referral Request
    Incomingreferral,
    /// The organization that has assumed the specific responsibilities of this EpisodeOfCare
    Organization,
    /// The current status of the Episode of Care as provided (does not check the status history collection)
    Status,
}
impl ResourceSearchParameterDefinition for EpisodeOfCareSearchParameter {
    fn resource_type(&self) -> &'static str {
        "EpisodeOfCare"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Date => "date",
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Type => "type",
            Self::CareManager => "care-manager",
            Self::Condition => "condition",
            Self::Incomingreferral => "incomingreferral",
            Self::Organization => "organization",
            Self::Status => "status",
        }
    }
}
/// Search parameters for the ExpansionProfile resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ExpansionProfileSearchParameter {
    /// The expansion profile publication date
    Date,
    /// The description of the expansion profile
    Description,
    /// External identifier for the expansion profile
    Identifier,
    /// Intended jurisdiction for the expansion profile
    Jurisdiction,
    /// Computationally friendly name of the expansion profile
    Name,
    /// Name of the publisher of the expansion profile
    Publisher,
    /// The current status of the expansion profile
    Status,
    /// The uri that identifies the expansion profile
    Url,
    /// The business version of the expansion profile
    Version,
}
impl ResourceSearchParameterDefinition for ExpansionProfileSearchParameter {
    fn resource_type(&self) -> &'static str {
        "ExpansionProfile"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Date => "date",
            Self::Description => "description",
            Self::Identifier => "identifier",
            Self::Jurisdiction => "jurisdiction",
            Self::Name => "name",
            Self::Publisher => "publisher",
            Self::Status => "status",
            Self::Url => "url",
            Self::Version => "version",
        }
    }
}
/// Search parameters for the Flag resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FlagSearchParameter {
    /// Time period when flag is active
    Date,
    /// The identity of a subject to list flags for
    Patient,
    /// Alert relevant during encounter
    Encounter,
    /// Flag creator
    Author,
    /// Business identifier
    Identifier,
    /// The identity of a subject to list flags for
    Subject,
}
impl ResourceSearchParameterDefinition for FlagSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Flag"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Date => "date",
            Self::Patient => "patient",
            Self::Encounter => "encounter",
            Self::Author => "author",
            Self::Identifier => "identifier",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the GraphDefinition resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum GraphDefinitionSearchParameter {
    /// The graph definition publication date
    Date,
    /// The description of the graph definition
    Description,
    /// Intended jurisdiction for the graph definition
    Jurisdiction,
    /// Computationally friendly name of the graph definition
    Name,
    /// Name of the publisher of the graph definition
    Publisher,
    /// Type of resource at which the graph starts
    Start,
    /// The current status of the graph definition
    Status,
    /// The uri that identifies the graph definition
    Url,
    /// The business version of the graph definition
    Version,
}
impl ResourceSearchParameterDefinition for GraphDefinitionSearchParameter {
    fn resource_type(&self) -> &'static str {
        "GraphDefinition"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Date => "date",
            Self::Description => "description",
            Self::Jurisdiction => "jurisdiction",
            Self::Name => "name",
            Self::Publisher => "publisher",
            Self::Start => "start",
            Self::Status => "status",
            Self::Url => "url",
            Self::Version => "version",
        }
    }
}
/// Search parameters for the GuidanceResponse resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum GuidanceResponseSearchParameter {
    /// The identifier of the guidance response
    Identifier,
    /// The identity of a patient to search for guidance response results
    Patient,
    /// The identifier of the request associated with the response
    Request,
    /// The subject that the guidance response is about
    Subject,
}
impl ResourceSearchParameterDefinition for GuidanceResponseSearchParameter {
    fn resource_type(&self) -> &'static str {
        "GuidanceResponse"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Request => "request",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the ImagingManifest resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ImagingManifestSearchParameter {
    /// Subject of the ImagingManifest (or a DICOM Key Object Selection which it represents)
    Patient,
    /// Author of the ImagingManifest (or a DICOM Key Object Selection which it represents)
    Author,
    /// Time of the ImagingManifest (or a DICOM Key Object Selection which it represents) authoring
    AuthoringTime,
    /// The endpoint for the study or series
    Endpoint,
    /// UID of the ImagingManifest (or a DICOM Key Object Selection which it represents)
    Identifier,
    /// ImagingStudy resource selected in the ImagingManifest (or a DICOM Key Object Selection which it represents)
    ImagingStudy,
    /// Study selected in the ImagingManifest (or a DICOM Key Object Selection which it represents)
    SelectedStudy,
}
impl ResourceSearchParameterDefinition for ImagingManifestSearchParameter {
    fn resource_type(&self) -> &'static str {
        "ImagingManifest"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Patient => "patient",
            Self::Author => "author",
            Self::AuthoringTime => "authoring-time",
            Self::Endpoint => "endpoint",
            Self::Identifier => "identifier",
            Self::ImagingStudy => "imaging-study",
            Self::SelectedStudy => "selected-study",
        }
    }
}
/// Search parameters for the Immunization resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ImmunizationSearchParameter {
    /// Vaccination  (non)-Administration Date
    Date,
    /// Business identifier
    Identifier,
    /// The patient for the vaccination record
    Patient,
    /// Dose number within series
    DoseSequence,
    /// The service delivery location or facility in which the vaccine was / was to be administered
    Location,
    /// Vaccine Lot Number
    LotNumber,
    /// Vaccine Manufacturer
    Manufacturer,
    /// Administrations which were not given
    Notgiven,
    /// The practitioner who played a role in the vaccination
    Practitioner,
    /// Additional information on reaction
    Reaction,
    /// When reaction started
    ReactionDate,
    /// Why immunization occurred
    Reason,
    /// Explanation of reason vaccination was not administered
    ReasonNotGiven,
    /// Immunization event status
    Status,
    /// Vaccine Product Administered
    VaccineCode,
}
impl ResourceSearchParameterDefinition for ImmunizationSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Immunization"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Date => "date",
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::DoseSequence => "dose-sequence",
            Self::Location => "location",
            Self::LotNumber => "lot-number",
            Self::Manufacturer => "manufacturer",
            Self::Notgiven => "notgiven",
            Self::Practitioner => "practitioner",
            Self::Reaction => "reaction",
            Self::ReactionDate => "reaction-date",
            Self::Reason => "reason",
            Self::ReasonNotGiven => "reason-not-given",
            Self::Status => "status",
            Self::VaccineCode => "vaccine-code",
        }
    }
}
/// Search parameters for the ImmunizationRecommendation resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ImmunizationRecommendationSearchParameter {
    /// Date recommendation created
    Date,
    /// Recommended dose number
    DoseNumber,
    /// Dose number within sequence
    DoseSequence,
    /// Business identifier
    Identifier,
    /// Patient observations supporting recommendation
    Information,
    /// Who this profile is for
    Patient,
    /// Vaccine administration status
    Status,
    /// Past immunizations supporting recommendation
    Support,
    /// Disease to be immunized against
    TargetDisease,
    /// Vaccine recommendation applies to
    VaccineType,
}
impl ResourceSearchParameterDefinition for ImmunizationRecommendationSearchParameter {
    fn resource_type(&self) -> &'static str {
        "ImmunizationRecommendation"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Date => "date",
            Self::DoseNumber => "dose-number",
            Self::DoseSequence => "dose-sequence",
            Self::Identifier => "identifier",
            Self::Information => "information",
            Self::Patient => "patient",
            Self::Status => "status",
            Self::Support => "support",
            Self::TargetDisease => "target-disease",
            Self::VaccineType => "vaccine-type",
        }
    }
}
/// Search parameters for the ImplementationGuide resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ImplementationGuideSearchParameter {
    /// The implementation guide publication date
    Date,
    /// Where to find dependency
    Dependency,
    /// The description of the implementation guide
    Description,
    /// For testing purposes, not real usage
    Experimental,
    /// Intended jurisdiction for the implementation guide
    Jurisdiction,
    /// Computationally friendly name of the implementation guide
    Name,
    /// Name of the publisher of the implementation guide
    Publisher,
    /// Location of the resource
    Resource,
    /// The current status of the implementation guide
    Status,
    /// The uri that identifies the implementation guide
    Url,
    /// The business version of the implementation guide
    Version,
}
impl ResourceSearchParameterDefinition for ImplementationGuideSearchParameter {
    fn resource_type(&self) -> &'static str {
        "ImplementationGuide"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Date => "date",
            Self::Dependency => "dependency",
            Self::Description => "description",
            Self::Experimental => "experimental",
            Self::Jurisdiction => "jurisdiction",
            Self::Name => "name",
            Self::Publisher => "publisher",
            Self::Resource => "resource",
            Self::Status => "status",
            Self::Url => "url",
            Self::Version => "version",
        }
    }
}
/// Search parameters for the Linkage resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LinkageSearchParameter {
    /// Author of the Linkage
    Author,
    /// Matches on any item in the Linkage
    Item,
    /// Matches on any item in the Linkage with a type of 'source'
    Source,
}
impl ResourceSearchParameterDefinition for LinkageSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Linkage"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Author => "author",
            Self::Item => "item",
            Self::Source => "source",
        }
    }
}
/// Search parameters for the Location resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LocationSearchParameter {
    /// A (part of the) address of the location
    Address,
    /// A city specified in an address
    AddressCity,
    /// A country specified in an address
    AddressCountry,
    /// A postal code specified in an address
    AddressPostalcode,
    /// A state specified in an address
    AddressState,
    /// A use code specified in an address
    AddressUse,
    /// Technical endpoints providing access to services operated for the location
    Endpoint,
    /// An identifier for the location
    Identifier,
    /// A portion of the location's name or alias
    Name,
    /** The coordinates expressed as [latitude]:[longitude] (using the WGS84 datum, see notes) to find locations near to (servers may search using a square rather than a circle for efficiency)

Requires the near-distance parameter to be provided also*/
    Near,
    /** A distance quantity to limit the near search to locations within a specific distance

Requires the near parameter to also be included*/
    NearDistance,
    /// Searches for locations (typically bed/room) that have an operational status (e.g. contaminated, housekeeping)
    OperationalStatus,
    /// Searches for locations that are managed by the provided organization
    Organization,
    /// A location of which this location is a part
    Partof,
    /// Searches for locations with a specific kind of status
    Status,
    /// A code for the type of location
    Type,
}
impl ResourceSearchParameterDefinition for LocationSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Location"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Address => "address",
            Self::AddressCity => "address-city",
            Self::AddressCountry => "address-country",
            Self::AddressPostalcode => "address-postalcode",
            Self::AddressState => "address-state",
            Self::AddressUse => "address-use",
            Self::Endpoint => "endpoint",
            Self::Identifier => "identifier",
            Self::Name => "name",
            Self::Near => "near",
            Self::NearDistance => "near-distance",
            Self::OperationalStatus => "operational-status",
            Self::Organization => "organization",
            Self::Partof => "partof",
            Self::Status => "status",
            Self::Type => "type",
        }
    }
}
/// Search parameters for the MeasureReport resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MeasureReportSearchParameter {
    /// External identifier of the measure report to be returned
    Identifier,
    /// The identity of a patient to search for individual measure report results for
    Patient,
    /// The status of the measure report
    Status,
}
impl ResourceSearchParameterDefinition for MeasureReportSearchParameter {
    fn resource_type(&self) -> &'static str {
        "MeasureReport"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Status => "status",
        }
    }
}
/// Search parameters for the Medication resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MedicationSearchParameter {
    /// Codes that identify this medication
    Code,
    /// E.g. box, vial, blister-pack
    Container,
    /// powder | tablets | capsule +
    Form,
    /// The product contained
    Ingredient,
    /// The product contained
    IngredientCode,
    /// Manufacturer of the item
    Manufacturer,
    /// True if medication does not require a prescription
    OverTheCounter,
    /// The item in the package
    PackageItem,
    /// The item in the package
    PackageItemCode,
    /// active | inactive | entered-in-error
    Status,
}
impl ResourceSearchParameterDefinition for MedicationSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Medication"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Code => "code",
            Self::Container => "container",
            Self::Form => "form",
            Self::Ingredient => "ingredient",
            Self::IngredientCode => "ingredient-code",
            Self::Manufacturer => "manufacturer",
            Self::OverTheCounter => "over-the-counter",
            Self::PackageItem => "package-item",
            Self::PackageItemCode => "package-item-code",
            Self::Status => "status",
        }
    }
}
/// Search parameters for the MedicationAdministration resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MedicationAdministrationSearchParameter {
    /// Return administrations of this medication code
    Code,
    /// Return administrations that share this encounter or episode of care
    Context,
    /// Return administrations with this administration device identity
    Device,
    /// Date administration happened (or did not happen)
    EffectiveTime,
    /// Return administrations with this external identifier
    Identifier,
    /// Return administrations of this medication resource
    Medication,
    /// Administrations that were not made
    NotGiven,
    /// The identity of a patient to list administrations  for
    Patient,
    /// The identify of the individual who administered the medication
    Performer,
    /// The identity of a prescription to list administrations from
    Prescription,
    /// Reasons for administering the medication
    ReasonGiven,
    /// Reasons for not administering the medication
    ReasonNotGiven,
    /// MedicationAdministration event status (for example one of active/paused/completed/nullified)
    Status,
    /// The identify of the individual or group to list administrations for
    Subject,
}
impl ResourceSearchParameterDefinition for MedicationAdministrationSearchParameter {
    fn resource_type(&self) -> &'static str {
        "MedicationAdministration"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Code => "code",
            Self::Context => "context",
            Self::Device => "device",
            Self::EffectiveTime => "effective-time",
            Self::Identifier => "identifier",
            Self::Medication => "medication",
            Self::NotGiven => "not-given",
            Self::Patient => "patient",
            Self::Performer => "performer",
            Self::Prescription => "prescription",
            Self::ReasonGiven => "reason-given",
            Self::ReasonNotGiven => "reason-not-given",
            Self::Status => "status",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the MedicationDispense resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MedicationDispenseSearchParameter {
    /// Return dispenses of this medicine code
    Code,
    /// Return dispenses with this external identifier
    Identifier,
    /// Return dispenses of this medicine resource
    Medication,
    /// The identity of a patient to list dispenses  for
    Patient,
    /// The identity of a prescription to list dispenses from
    Prescription,
    /// Return dispenses with a specified dispense status
    Status,
    /// Returns dispenses with a specific context (episode or episode of care)
    Context,
    /// Return dispenses that should be sent to a specific destination
    Destination,
    /// Return dispenses performed by a specific individual
    Performer,
    /// The identity of a receiver to list dispenses for
    Receiver,
    /// Return dispenses with the specified responsible party
    Responsibleparty,
    /// The identity of a patient to list dispenses  for
    Subject,
    /// Return dispenses of a specific type
    Type,
    /// Returns dispenses handed over on this date
    Whenhandedover,
    /// Returns dispenses prepared on this date
    Whenprepared,
}
impl ResourceSearchParameterDefinition for MedicationDispenseSearchParameter {
    fn resource_type(&self) -> &'static str {
        "MedicationDispense"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Code => "code",
            Self::Identifier => "identifier",
            Self::Medication => "medication",
            Self::Patient => "patient",
            Self::Prescription => "prescription",
            Self::Status => "status",
            Self::Context => "context",
            Self::Destination => "destination",
            Self::Performer => "performer",
            Self::Receiver => "receiver",
            Self::Responsibleparty => "responsibleparty",
            Self::Subject => "subject",
            Self::Type => "type",
            Self::Whenhandedover => "whenhandedover",
            Self::Whenprepared => "whenprepared",
        }
    }
}
/// Search parameters for the MedicationRequest resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MedicationRequestSearchParameter {
    /// Return prescriptions of this medication code
    Code,
    /// Return prescriptions with this external identifier
    Identifier,
    /// Return prescriptions of this medication reference
    Medication,
    /// Returns prescriptions for a specific patient
    Patient,
    /// Status of the prescription
    Status,
    /// Return prescriptions written on this date
    Authoredon,
    /// Returns prescriptions with different categories
    Category,
    /// Return prescriptions with this encounter or episode of care identifier
    Context,
    /// Returns medication request to be administered on a specific date
    Date,
    /// Returns prescriptions intended to be dispensed by this Organization
    IntendedDispenser,
    /// Returns prescriptions with different intents
    Intent,
    /// Returns prescriptions with different priorities
    Priority,
    /// Returns prescriptions prescribed by this prescriber
    Requester,
    /// The identity of a patient to list orders  for
    Subject,
}
impl ResourceSearchParameterDefinition for MedicationRequestSearchParameter {
    fn resource_type(&self) -> &'static str {
        "MedicationRequest"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Code => "code",
            Self::Identifier => "identifier",
            Self::Medication => "medication",
            Self::Patient => "patient",
            Self::Status => "status",
            Self::Authoredon => "authoredon",
            Self::Category => "category",
            Self::Context => "context",
            Self::Date => "date",
            Self::IntendedDispenser => "intended-dispenser",
            Self::Intent => "intent",
            Self::Priority => "priority",
            Self::Requester => "requester",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the MedicationStatement resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MedicationStatementSearchParameter {
    /// Return statements of this medication code
    Code,
    /// Return statements with this external identifier
    Identifier,
    /// Return statements of this medication reference
    Medication,
    /// Returns statements for a specific patient.
    Patient,
    /// Return statements that match the given status
    Status,
    /// Returns statements of this category of medicationstatement
    Category,
    /// Returns statements for a specific context (episode or episode of Care).
    Context,
    /// Date when patient was taking (or not taking) the medication
    Effective,
    /// Returns statements that are part of another event.
    PartOf,
    /// Who or where the information in the statement came from
    Source,
    /// The identity of a patient, animal or group to list statements for
    Subject,
}
impl ResourceSearchParameterDefinition for MedicationStatementSearchParameter {
    fn resource_type(&self) -> &'static str {
        "MedicationStatement"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Code => "code",
            Self::Identifier => "identifier",
            Self::Medication => "medication",
            Self::Patient => "patient",
            Self::Status => "status",
            Self::Category => "category",
            Self::Context => "context",
            Self::Effective => "effective",
            Self::PartOf => "part-of",
            Self::Source => "source",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the MessageDefinition resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MessageDefinitionSearchParameter {
    /// The behavior associated with the message
    Category,
    /// The message definition publication date
    Date,
    /// The description of the message definition
    Description,
    /// The event that triggers the message
    Event,
    /// A resource that is a permitted focus of the message
    Focus,
    /// External identifier for the message definition
    Identifier,
    /// Intended jurisdiction for the message definition
    Jurisdiction,
    /// Computationally friendly name of the message definition
    Name,
    /// Name of the publisher of the message definition
    Publisher,
    /// The current status of the message definition
    Status,
    /// The human-friendly name of the message definition
    Title,
    /// The uri that identifies the message definition
    Url,
    /// The business version of the message definition
    Version,
}
impl ResourceSearchParameterDefinition for MessageDefinitionSearchParameter {
    fn resource_type(&self) -> &'static str {
        "MessageDefinition"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Category => "category",
            Self::Date => "date",
            Self::Description => "description",
            Self::Event => "event",
            Self::Focus => "focus",
            Self::Identifier => "identifier",
            Self::Jurisdiction => "jurisdiction",
            Self::Name => "name",
            Self::Publisher => "publisher",
            Self::Status => "status",
            Self::Title => "title",
            Self::Url => "url",
            Self::Version => "version",
        }
    }
}
/// Search parameters for the MessageHeader resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MessageHeaderSearchParameter {
    /// The source of the decision
    Author,
    /// ok | transient-error | fatal-error
    Code,
    /// Name of system
    Destination,
    /// Actual destination address or id
    DestinationUri,
    /// The source of the data entry
    Enterer,
    /// Code for the event this message represents
    Event,
    /// The actual content of the message
    Focus,
    /// Intended "real-world" recipient for the data
    Receiver,
    /// Id of original message
    ResponseId,
    /// Final responsibility for event
    Responsible,
    /// Real world sender of the message
    Sender,
    /// Name of system
    Source,
    /// Actual message source address or id
    SourceUri,
    /// Particular delivery destination within the destination
    Target,
    /// Time that the message was sent
    Timestamp,
}
impl ResourceSearchParameterDefinition for MessageHeaderSearchParameter {
    fn resource_type(&self) -> &'static str {
        "MessageHeader"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Author => "author",
            Self::Code => "code",
            Self::Destination => "destination",
            Self::DestinationUri => "destination-uri",
            Self::Enterer => "enterer",
            Self::Event => "event",
            Self::Focus => "focus",
            Self::Receiver => "receiver",
            Self::ResponseId => "response-id",
            Self::Responsible => "responsible",
            Self::Sender => "sender",
            Self::Source => "source",
            Self::SourceUri => "source-uri",
            Self::Target => "target",
            Self::Timestamp => "timestamp",
        }
    }
}
/// Search parameters for the NamingSystem resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum NamingSystemSearchParameter {
    /// Name of an individual to contact
    Contact,
    /// The naming system publication date
    Date,
    /// The description of the naming system
    Description,
    /// oid | uuid | uri | other
    IdType,
    /// Intended jurisdiction for the naming system
    Jurisdiction,
    /// codesystem | identifier | root
    Kind,
    /// Computationally friendly name of the naming system
    Name,
    /// When is identifier valid?
    Period,
    /// Name of the publisher of the naming system
    Publisher,
    /// Use this instead
    ReplacedBy,
    /// Who maintains system namespace?
    Responsible,
    /// The current status of the naming system
    Status,
    /// Contact details for individual or organization
    Telecom,
    /// e.g. driver,  provider,  patient, bank etc.
    Type,
    /// The unique identifier
    Value,
}
impl ResourceSearchParameterDefinition for NamingSystemSearchParameter {
    fn resource_type(&self) -> &'static str {
        "NamingSystem"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Contact => "contact",
            Self::Date => "date",
            Self::Description => "description",
            Self::IdType => "id-type",
            Self::Jurisdiction => "jurisdiction",
            Self::Kind => "kind",
            Self::Name => "name",
            Self::Period => "period",
            Self::Publisher => "publisher",
            Self::ReplacedBy => "replaced-by",
            Self::Responsible => "responsible",
            Self::Status => "status",
            Self::Telecom => "telecom",
            Self::Type => "type",
            Self::Value => "value",
        }
    }
}
/// Search parameters for the NutritionOrder resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum NutritionOrderSearchParameter {
    /// Return nutrition orders with this external identifier
    Identifier,
    /// The identity of the person who requires the diet, formula or nutritional supplement
    Patient,
    /// Return nutrition orders with this encounter identifier
    Encounter,
    /// Type of module component to add to the feeding
    Additive,
    /// Return nutrition orders requested on this date
    Datetime,
    /// Type of enteral or infant formula
    Formula,
    /// Type of diet that can be consumed orally (i.e., take via the mouth).
    Oraldiet,
    /// The identify of the provider who placed the nutrition order
    Provider,
    /// Status of the nutrition order.
    Status,
    /// Type of supplement product requested
    Supplement,
}
impl ResourceSearchParameterDefinition for NutritionOrderSearchParameter {
    fn resource_type(&self) -> &'static str {
        "NutritionOrder"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Encounter => "encounter",
            Self::Additive => "additive",
            Self::Datetime => "datetime",
            Self::Formula => "formula",
            Self::Oraldiet => "oraldiet",
            Self::Provider => "provider",
            Self::Status => "status",
            Self::Supplement => "supplement",
        }
    }
}
/// Search parameters for the Observation resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ObservationSearchParameter {
    /// The code of the observation type
    Code,
    /// Obtained date/time. If the obtained element is a period, a date that falls in the period
    Date,
    /// The unique id for a particular observation
    Identifier,
    /// The subject that the observation is about (if patient)
    Patient,
    /// Encounter related to the observation
    Encounter,
    /// Reference to the test or procedure request.
    BasedOn,
    /// The classification of the type of observation
    Category,
    /// Code and coded value parameter pair
    CodeValueConcept,
    /// Code and date/time value parameter pair
    CodeValueDate,
    /// Code and quantity value parameter pair
    CodeValueQuantity,
    /// Code and string value parameter pair
    CodeValueString,
    /// The code of the observation type or component type
    ComboCode,
    /// Code and coded value parameter pair, including in components
    ComboCodeValueConcept,
    /// Code and quantity value parameter pair, including in components
    ComboCodeValueQuantity,
    /// The reason why the expected value in the element Observation.value[x] or Observation.component.value[x] is missing.
    ComboDataAbsentReason,
    /// The value or component value of the observation, if the value is a CodeableConcept
    ComboValueConcept,
    /// The value or component value of the observation, if the value is a Quantity, or a SampledData (just search on the bounds of the values in sampled data)
    ComboValueQuantity,
    /// The component code of the observation type
    ComponentCode,
    /// Component code and component coded value parameter pair
    ComponentCodeValueConcept,
    /// Component code and component quantity value parameter pair
    ComponentCodeValueQuantity,
    /// The reason why the expected value in the element Observation.component.value[x] is missing.
    ComponentDataAbsentReason,
    /// The value of the component observation, if the value is a CodeableConcept
    ComponentValueConcept,
    /// The value of the component observation, if the value is a Quantity, or a SampledData (just search on the bounds of the values in sampled data)
    ComponentValueQuantity,
    /// Healthcare event  (Episode-of-care or Encounter) related to the observation
    Context,
    /// The reason why the expected value in the element Observation.value[x] is missing.
    DataAbsentReason,
    /// The Device that generated the observation data.
    Device,
    /// The method used for the observation
    Method,
    /// Who performed the observation
    Performer,
    /// Related Observations - search on related-type and related-target together
    Related,
    /// Resource that is related to this one
    RelatedTarget,
    /// has-member | derived-from | sequel-to | replaces | qualified-by | interfered-by
    RelatedType,
    /// Specimen used for this observation
    Specimen,
    /// The status of the observation
    Status,
    /// The subject that the observation is about
    Subject,
    /// The value of the observation, if the value is a CodeableConcept
    ValueConcept,
    /// The value of the observation, if the value is a date or period of time
    ValueDate,
    /// The value of the observation, if the value is a Quantity, or a SampledData (just search on the bounds of the values in sampled data)
    ValueQuantity,
    /// The value of the observation, if the value is a string, and also searches in CodeableConcept.text
    ValueString,
}
impl ResourceSearchParameterDefinition for ObservationSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Observation"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Code => "code",
            Self::Date => "date",
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Encounter => "encounter",
            Self::BasedOn => "based-on",
            Self::Category => "category",
            Self::CodeValueConcept => "code-value-concept",
            Self::CodeValueDate => "code-value-date",
            Self::CodeValueQuantity => "code-value-quantity",
            Self::CodeValueString => "code-value-string",
            Self::ComboCode => "combo-code",
            Self::ComboCodeValueConcept => "combo-code-value-concept",
            Self::ComboCodeValueQuantity => "combo-code-value-quantity",
            Self::ComboDataAbsentReason => "combo-data-absent-reason",
            Self::ComboValueConcept => "combo-value-concept",
            Self::ComboValueQuantity => "combo-value-quantity",
            Self::ComponentCode => "component-code",
            Self::ComponentCodeValueConcept => "component-code-value-concept",
            Self::ComponentCodeValueQuantity => "component-code-value-quantity",
            Self::ComponentDataAbsentReason => "component-data-absent-reason",
            Self::ComponentValueConcept => "component-value-concept",
            Self::ComponentValueQuantity => "component-value-quantity",
            Self::Context => "context",
            Self::DataAbsentReason => "data-absent-reason",
            Self::Device => "device",
            Self::Method => "method",
            Self::Performer => "performer",
            Self::Related => "related",
            Self::RelatedTarget => "related-target",
            Self::RelatedType => "related-type",
            Self::Specimen => "specimen",
            Self::Status => "status",
            Self::Subject => "subject",
            Self::ValueConcept => "value-concept",
            Self::ValueDate => "value-date",
            Self::ValueQuantity => "value-quantity",
            Self::ValueString => "value-string",
        }
    }
}
/// Search parameters for the OperationDefinition resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum OperationDefinitionSearchParameter {
    /// Marks this as a profile of the base
    Base,
    /// Name used to invoke the operation
    Code,
    /// The operation definition publication date
    Date,
    /// The description of the operation definition
    Description,
    /// Invoke on an instance?
    Instance,
    /// Intended jurisdiction for the operation definition
    Jurisdiction,
    /// operation | query
    Kind,
    /// Computationally friendly name of the operation definition
    Name,
    /// Profile on the type
    ParamProfile,
    /// Name of the publisher of the operation definition
    Publisher,
    /// The current status of the operation definition
    Status,
    /// Invoke at the system level?
    System,
    /// Invole at the type level?
    Type,
    /// The uri that identifies the operation definition
    Url,
    /// The business version of the operation definition
    Version,
}
impl ResourceSearchParameterDefinition for OperationDefinitionSearchParameter {
    fn resource_type(&self) -> &'static str {
        "OperationDefinition"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Base => "base",
            Self::Code => "code",
            Self::Date => "date",
            Self::Description => "description",
            Self::Instance => "instance",
            Self::Jurisdiction => "jurisdiction",
            Self::Kind => "kind",
            Self::Name => "name",
            Self::ParamProfile => "param-profile",
            Self::Publisher => "publisher",
            Self::Status => "status",
            Self::System => "system",
            Self::Type => "type",
            Self::Url => "url",
            Self::Version => "version",
        }
    }
}
/// Search parameters for the Organization resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum OrganizationSearchParameter {
    /// A server defined search that may match any of the string fields in the Address, including line, city, state, country, postalCode, and/or text
    Active,
    /// A (part of the) address of the organization
    Address,
    /// A city specified in an address
    AddressCity,
    /// A country specified in an address
    AddressCountry,
    /// A postal code specified in an address
    AddressPostalcode,
    /// A state specified in an address
    AddressState,
    /// A use code specified in an address
    AddressUse,
    /// Technical endpoints providing access to services operated for the organization
    Endpoint,
    /// Any identifier for the organization (not the accreditation issuer's identifier)
    Identifier,
    /// A portion of the organization's name or alias
    Name,
    /// An organization of which this organization forms a part
    Partof,
    /// A portion of the organization's name using some kind of phonetic matching algorithm
    Phonetic,
    /// A code for the type of organization
    Type,
}
impl ResourceSearchParameterDefinition for OrganizationSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Organization"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Address => "address",
            Self::AddressCity => "address-city",
            Self::AddressCountry => "address-country",
            Self::AddressPostalcode => "address-postalcode",
            Self::AddressState => "address-state",
            Self::AddressUse => "address-use",
            Self::Endpoint => "endpoint",
            Self::Identifier => "identifier",
            Self::Name => "name",
            Self::Partof => "partof",
            Self::Phonetic => "phonetic",
            Self::Type => "type",
        }
    }
}
/// Search parameters for the Patient resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PatientSearchParameter {
    /// Whether the patient record is active
    Active,
    /// A server defined search that may match any of the string fields in the Address, including line, city, state, country, postalCode, and/or text
    Address,
    /// A city specified in an address
    AddressCity,
    /// A country specified in an address
    AddressCountry,
    /// A postalCode specified in an address
    AddressPostalcode,
    /// A state specified in an address
    AddressState,
    /// A use code specified in an address
    AddressUse,
    /// The breed for animal patients
    AnimalBreed,
    /// The species for animal patients
    AnimalSpecies,
    /// The patient's date of birth
    Birthdate,
    /// The date of death has been provided and satisfies this search value
    DeathDate,
    /// This patient has been marked as deceased, or as a death date entered
    Deceased,
    /// A value in an email contact
    Email,
    /// A portion of the family name of the patient
    Family,
    /// Gender of the patient
    Gender,
    /// Patient's nominated general practitioner, not the organization that manages the record
    GeneralPractitioner,
    /// A portion of the given name of the patient
    Given,
    /// A patient identifier
    Identifier,
    /// Language code (irrespective of use value)
    Language,
    /// All patients linked to the given patient
    Link,
    /// A server defined search that may match any of the string fields in the HumanName, including family, give, prefix, suffix, suffix, and/or text
    Name,
    /// The organization at which this person is a patient
    Organization,
    /// A value in a phone contact
    Phone,
    /// A portion of either family or given name using some kind of phonetic matching algorithm
    Phonetic,
    /// The value in any kind of telecom details of the patient
    Telecom,
}
impl ResourceSearchParameterDefinition for PatientSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Patient"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Address => "address",
            Self::AddressCity => "address-city",
            Self::AddressCountry => "address-country",
            Self::AddressPostalcode => "address-postalcode",
            Self::AddressState => "address-state",
            Self::AddressUse => "address-use",
            Self::AnimalBreed => "animal-breed",
            Self::AnimalSpecies => "animal-species",
            Self::Birthdate => "birthdate",
            Self::DeathDate => "death-date",
            Self::Deceased => "deceased",
            Self::Email => "email",
            Self::Family => "family",
            Self::Gender => "gender",
            Self::GeneralPractitioner => "general-practitioner",
            Self::Given => "given",
            Self::Identifier => "identifier",
            Self::Language => "language",
            Self::Link => "link",
            Self::Name => "name",
            Self::Organization => "organization",
            Self::Phone => "phone",
            Self::Phonetic => "phonetic",
            Self::Telecom => "telecom",
        }
    }
}
/// Search parameters for the Person resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PersonSearchParameter {
    /// A server defined search that may match any of the string fields in the Address, including line, city, state, country, postalCode, and/or text
    Address,
    /// A city specified in an address
    AddressCity,
    /// A country specified in an address
    AddressCountry,
    /// A postal code specified in an address
    AddressPostalcode,
    /// A state specified in an address
    AddressState,
    /// A use code specified in an address
    AddressUse,
    /// The person's date of birth
    Birthdate,
    /// A value in an email contact
    Email,
    /// The gender of the person
    Gender,
    /// A value in a phone contact
    Phone,
    /// A portion of name using some kind of phonetic matching algorithm
    Phonetic,
    /// The value in any kind of contact
    Telecom,
    /// A person Identifier
    Identifier,
    /// Any link has this Patient, Person, RelatedPerson or Practitioner reference
    Link,
    /// A server defined search that may match any of the string fields in the HumanName, including family, give, prefix, suffix, suffix, and/or text
    Name,
    /// The organization at which this person record is being managed
    Organization,
    /// The Person links to this Patient
    Patient,
    /// The Person links to this Practitioner
    Practitioner,
    /// The Person links to this RelatedPerson
    Relatedperson,
}
impl ResourceSearchParameterDefinition for PersonSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Person"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Address => "address",
            Self::AddressCity => "address-city",
            Self::AddressCountry => "address-country",
            Self::AddressPostalcode => "address-postalcode",
            Self::AddressState => "address-state",
            Self::AddressUse => "address-use",
            Self::Birthdate => "birthdate",
            Self::Email => "email",
            Self::Gender => "gender",
            Self::Phone => "phone",
            Self::Phonetic => "phonetic",
            Self::Telecom => "telecom",
            Self::Identifier => "identifier",
            Self::Link => "link",
            Self::Name => "name",
            Self::Organization => "organization",
            Self::Patient => "patient",
            Self::Practitioner => "practitioner",
            Self::Relatedperson => "relatedperson",
        }
    }
}
/// Search parameters for the PlanDefinition resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PlanDefinitionSearchParameter {
    /// What resource is being referenced
    ComposedOf,
    /// The plan definition publication date
    Date,
    /// What resource is being referenced
    DependsOn,
    /// What resource is being referenced
    DerivedFrom,
    /// The description of the plan definition
    Description,
    /// The time during which the plan definition is intended to be in use
    Effective,
    /// External identifier for the plan definition
    Identifier,
    /// Intended jurisdiction for the plan definition
    Jurisdiction,
    /// Computationally friendly name of the plan definition
    Name,
    /// What resource is being referenced
    Predecessor,
    /// Name of the publisher of the plan definition
    Publisher,
    /// The current status of the plan definition
    Status,
    /// What resource is being referenced
    Successor,
    /// The human-friendly name of the plan definition
    Title,
    /// Topics associated with the module
    Topic,
    /// The uri that identifies the plan definition
    Url,
    /// The business version of the plan definition
    Version,
}
impl ResourceSearchParameterDefinition for PlanDefinitionSearchParameter {
    fn resource_type(&self) -> &'static str {
        "PlanDefinition"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::ComposedOf => "composed-of",
            Self::Date => "date",
            Self::DependsOn => "depends-on",
            Self::DerivedFrom => "derived-from",
            Self::Description => "description",
            Self::Effective => "effective",
            Self::Identifier => "identifier",
            Self::Jurisdiction => "jurisdiction",
            Self::Name => "name",
            Self::Predecessor => "predecessor",
            Self::Publisher => "publisher",
            Self::Status => "status",
            Self::Successor => "successor",
            Self::Title => "title",
            Self::Topic => "topic",
            Self::Url => "url",
            Self::Version => "version",
        }
    }
}
/// Search parameters for the Practitioner resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PractitionerSearchParameter {
    /// A server defined search that may match any of the string fields in the Address, including line, city, state, country, postalCode, and/or text
    Address,
    /// A city specified in an address
    AddressCity,
    /// A country specified in an address
    AddressCountry,
    /// A postalCode specified in an address
    AddressPostalcode,
    /// A state specified in an address
    AddressState,
    /// A use code specified in an address
    AddressUse,
    /// A value in an email contact
    Email,
    /// A portion of the family name
    Family,
    /// Gender of the practitioner
    Gender,
    /// A portion of the given name
    Given,
    /// A value in a phone contact
    Phone,
    /// A portion of either family or given name using some kind of phonetic matching algorithm
    Phonetic,
    /// The value in any kind of contact
    Telecom,
    /// Whether the practitioner record is active
    Active,
    /// One of the languages that the practitioner can communicate with
    Communication,
    /// A practitioner's Identifier
    Identifier,
    /// A server defined search that may match any of the string fields in the HumanName, including family, give, prefix, suffix, suffix, and/or text
    Name,
}
impl ResourceSearchParameterDefinition for PractitionerSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Practitioner"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Address => "address",
            Self::AddressCity => "address-city",
            Self::AddressCountry => "address-country",
            Self::AddressPostalcode => "address-postalcode",
            Self::AddressState => "address-state",
            Self::AddressUse => "address-use",
            Self::Email => "email",
            Self::Family => "family",
            Self::Gender => "gender",
            Self::Given => "given",
            Self::Phone => "phone",
            Self::Phonetic => "phonetic",
            Self::Telecom => "telecom",
            Self::Active => "active",
            Self::Communication => "communication",
            Self::Identifier => "identifier",
            Self::Name => "name",
        }
    }
}
/// Search parameters for the PractitionerRole resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PractitionerRoleSearchParameter {
    /// A value in an email contact
    Email,
    /// A value in a phone contact
    Phone,
    /// The value in any kind of contact
    Telecom,
    /// Whether this practitioner's record is in active use
    Active,
    /// The period during which the practitioner is authorized to perform in these role(s)
    Date,
    /// Technical endpoints providing access to services operated for the practitioner with this role
    Endpoint,
    /// A practitioner's Identifier
    Identifier,
    /// One of the locations at which this practitioner provides care
    Location,
    /// The identity of the organization the practitioner represents / acts on behalf of
    Organization,
    /// Practitioner that is able to provide the defined services for the organation
    Practitioner,
    /// The practitioner can perform this role at for the organization
    Role,
    /// The list of healthcare services that this worker provides for this role's Organization/Location(s)
    Service,
    /// The practitioner has this specialty at an organization
    Specialty,
}
impl ResourceSearchParameterDefinition for PractitionerRoleSearchParameter {
    fn resource_type(&self) -> &'static str {
        "PractitionerRole"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Email => "email",
            Self::Phone => "phone",
            Self::Telecom => "telecom",
            Self::Active => "active",
            Self::Date => "date",
            Self::Endpoint => "endpoint",
            Self::Identifier => "identifier",
            Self::Location => "location",
            Self::Organization => "organization",
            Self::Practitioner => "practitioner",
            Self::Role => "role",
            Self::Service => "service",
            Self::Specialty => "specialty",
        }
    }
}
/// Search parameters for the Procedure resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ProcedureSearchParameter {
    /// A code to identify a  procedure
    Code,
    /// Date/Period the procedure was performed
    Date,
    /// A unique identifier for a procedure
    Identifier,
    /// Search by subject - a patient
    Patient,
    /// Search by encounter
    Encounter,
    /// A request for this procedure
    BasedOn,
    /// Classification of the procedure
    Category,
    /// Encounter or episode associated with the procedure
    Context,
    /// Instantiates protocol or definition
    Definition,
    /// Where the procedure happened
    Location,
    /// Part of referenced event
    PartOf,
    /// The reference to the practitioner
    Performer,
    /// preparation | in-progress | suspended | aborted | completed | entered-in-error | unknown
    Status,
    /// Search by subject
    Subject,
}
impl ResourceSearchParameterDefinition for ProcedureSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Procedure"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Code => "code",
            Self::Date => "date",
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Encounter => "encounter",
            Self::BasedOn => "based-on",
            Self::Category => "category",
            Self::Context => "context",
            Self::Definition => "definition",
            Self::Location => "location",
            Self::PartOf => "part-of",
            Self::Performer => "performer",
            Self::Status => "status",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the ProcedureRequest resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ProcedureRequestSearchParameter {
    /// What is being requested/ordered
    Code,
    /// Identifiers assigned to this order
    Identifier,
    /// Search by subject - a patient
    Patient,
    /// An encounter in which this request is made
    Encounter,
    /// Date request signed
    Authored,
    /// What request fulfills
    BasedOn,
    /// Where procedure is going to be done
    BodySite,
    /// Encounter or Episode during which request was created
    Context,
    /// Protocol or definition
    Definition,
    /// proposal | plan | order +
    Intent,
    /// When procedure should occur
    Occurrence,
    /// Requested perfomer
    Performer,
    /// Performer role
    PerformerType,
    /// routine | urgent | asap | stat
    Priority,
    /// What request replaces
    Replaces,
    /// Individual making the request
    Requester,
    /// Composite Request ID
    Requisition,
    /// Specimen to be tested
    Specimen,
    /// draft | active | suspended | completed | entered-in-error | cancelled
    Status,
    /// Search by subject
    Subject,
}
impl ResourceSearchParameterDefinition for ProcedureRequestSearchParameter {
    fn resource_type(&self) -> &'static str {
        "ProcedureRequest"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Code => "code",
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Encounter => "encounter",
            Self::Authored => "authored",
            Self::BasedOn => "based-on",
            Self::BodySite => "body-site",
            Self::Context => "context",
            Self::Definition => "definition",
            Self::Intent => "intent",
            Self::Occurrence => "occurrence",
            Self::Performer => "performer",
            Self::PerformerType => "performer-type",
            Self::Priority => "priority",
            Self::Replaces => "replaces",
            Self::Requester => "requester",
            Self::Requisition => "requisition",
            Self::Specimen => "specimen",
            Self::Status => "status",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the Provenance resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ProvenanceSearchParameter {
    /// Who participated
    Agent,
    /// What the agents role was
    AgentRole,
    /// End time with inclusive boundary, if not ongoing
    End,
    /// Identity of entity
    EntityId,
    /// Identity of entity
    EntityRef,
    /// Where the activity occurred, if relevant
    Location,
    /// Target Reference(s) (usually version specific)
    Patient,
    /// When the activity was recorded / updated
    Recorded,
    /// Indication of the reason the entity signed the object(s)
    SignatureType,
    /// Starting time with inclusive boundary
    Start,
    /// Target Reference(s) (usually version specific)
    Target,
}
impl ResourceSearchParameterDefinition for ProvenanceSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Provenance"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Agent => "agent",
            Self::AgentRole => "agent-role",
            Self::End => "end",
            Self::EntityId => "entity-id",
            Self::EntityRef => "entity-ref",
            Self::Location => "location",
            Self::Patient => "patient",
            Self::Recorded => "recorded",
            Self::SignatureType => "signature-type",
            Self::Start => "start",
            Self::Target => "target",
        }
    }
}
/// Search parameters for the Questionnaire resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum QuestionnaireSearchParameter {
    /// A code that corresponds to one of its items in the questionnaire
    Code,
    /// The questionnaire publication date
    Date,
    /// The description of the questionnaire
    Description,
    /// The time during which the questionnaire is intended to be in use
    Effective,
    /// External identifier for the questionnaire
    Identifier,
    /// Intended jurisdiction for the questionnaire
    Jurisdiction,
    /// Computationally friendly name of the questionnaire
    Name,
    /// Name of the publisher of the questionnaire
    Publisher,
    /// The current status of the questionnaire
    Status,
    /// The human-friendly name of the questionnaire
    Title,
    /// The uri that identifies the questionnaire
    Url,
    /// The business version of the questionnaire
    Version,
}
impl ResourceSearchParameterDefinition for QuestionnaireSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Questionnaire"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Code => "code",
            Self::Date => "date",
            Self::Description => "description",
            Self::Effective => "effective",
            Self::Identifier => "identifier",
            Self::Jurisdiction => "jurisdiction",
            Self::Name => "name",
            Self::Publisher => "publisher",
            Self::Status => "status",
            Self::Title => "title",
            Self::Url => "url",
            Self::Version => "version",
        }
    }
}
/// Search parameters for the QuestionnaireResponse resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum QuestionnaireResponseSearchParameter {
    /// The author of the questionnaire response
    Author,
    /// When the questionnaire response was last changed
    Authored,
    /// Plan/proposal/order fulfilled by this questionnaire response
    BasedOn,
    /// Encounter or episode associated with the questionnaire response
    Context,
    /// The unique identifier for the questionnaire response
    Identifier,
    /// Procedure or observation this questionnaire response was performed as a part of
    Parent,
    /// The patient that is the subject of the questionnaire response
    Patient,
    /// The questionnaire the answers are provided for
    Questionnaire,
    /// The individual providing the information reflected in the questionnaire respose
    Source,
    /// The status of the questionnaire response
    Status,
    /// The subject of the questionnaire response
    Subject,
}
impl ResourceSearchParameterDefinition for QuestionnaireResponseSearchParameter {
    fn resource_type(&self) -> &'static str {
        "QuestionnaireResponse"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Author => "author",
            Self::Authored => "authored",
            Self::BasedOn => "based-on",
            Self::Context => "context",
            Self::Identifier => "identifier",
            Self::Parent => "parent",
            Self::Patient => "patient",
            Self::Questionnaire => "questionnaire",
            Self::Source => "source",
            Self::Status => "status",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the RelatedPerson resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum RelatedPersonSearchParameter {
    /// A server defined search that may match any of the string fields in the Address, including line, city, state, country, postalCode, and/or text
    Address,
    /// A city specified in an address
    AddressCity,
    /// A country specified in an address
    AddressCountry,
    /// A postal code specified in an address
    AddressPostalcode,
    /// A state specified in an address
    AddressState,
    /// A use code specified in an address
    AddressUse,
    /// The Related Person's date of birth
    Birthdate,
    /// A value in an email contact
    Email,
    /// Gender of the related person
    Gender,
    /// A value in a phone contact
    Phone,
    /// A portion of name using some kind of phonetic matching algorithm
    Phonetic,
    /// The value in any kind of contact
    Telecom,
    /// Indicates if the related person record is active
    Active,
    /// An Identifier of the RelatedPerson
    Identifier,
    /// A server defined search that may match any of the string fields in the HumanName, including family, give, prefix, suffix, suffix, and/or text
    Name,
    /// The patient this related person is related to
    Patient,
}
impl ResourceSearchParameterDefinition for RelatedPersonSearchParameter {
    fn resource_type(&self) -> &'static str {
        "RelatedPerson"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Address => "address",
            Self::AddressCity => "address-city",
            Self::AddressCountry => "address-country",
            Self::AddressPostalcode => "address-postalcode",
            Self::AddressState => "address-state",
            Self::AddressUse => "address-use",
            Self::Birthdate => "birthdate",
            Self::Email => "email",
            Self::Gender => "gender",
            Self::Phone => "phone",
            Self::Phonetic => "phonetic",
            Self::Telecom => "telecom",
            Self::Active => "active",
            Self::Identifier => "identifier",
            Self::Name => "name",
            Self::Patient => "patient",
        }
    }
}
/// Search parameters for the ResearchSubject resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ResearchSubjectSearchParameter {
    /// Start and end of participation
    Date,
    /// Business Identifier for research subject
    Identifier,
    /// Who is part of study
    Individual,
    /// Who is part of study
    Patient,
    /// candidate | enrolled | active | suspended | withdrawn | completed
    Status,
}
impl ResourceSearchParameterDefinition for ResearchSubjectSearchParameter {
    fn resource_type(&self) -> &'static str {
        "ResearchSubject"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Date => "date",
            Self::Identifier => "identifier",
            Self::Individual => "individual",
            Self::Patient => "patient",
            Self::Status => "status",
        }
    }
}
/// Search parameters for the RiskAssessment resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum RiskAssessmentSearchParameter {
    /// When was assessment made?
    Date,
    /// Unique identifier for the assessment
    Identifier,
    /// Who/what does assessment apply to?
    Patient,
    /// Where was assessment performed?
    Encounter,
    /// Condition assessed
    Condition,
    /// Evaluation mechanism
    Method,
    /// Who did assessment?
    Performer,
    /// Likelihood of specified outcome
    Probability,
    /// Likelihood of specified outcome as a qualitative value
    Risk,
    /// Who/what does assessment apply to?
    Subject,
}
impl ResourceSearchParameterDefinition for RiskAssessmentSearchParameter {
    fn resource_type(&self) -> &'static str {
        "RiskAssessment"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Date => "date",
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Encounter => "encounter",
            Self::Condition => "condition",
            Self::Method => "method",
            Self::Performer => "performer",
            Self::Probability => "probability",
            Self::Risk => "risk",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the SearchParameter resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SearchParameterSearchParameter {
    /// The resource type(s) this search parameter applies to
    Base,
    /// Code used in URL
    Code,
    /// Defines how the part works
    Component,
    /// The search parameter publication date
    Date,
    /// Original Definition for the search parameter
    DerivedFrom,
    /// The description of the search parameter
    Description,
    /// Intended jurisdiction for the search parameter
    Jurisdiction,
    /// Computationally friendly name of the search parameter
    Name,
    /// Name of the publisher of the search parameter
    Publisher,
    /// The current status of the search parameter
    Status,
    /// Types of resource (if a resource reference)
    Target,
    /// number | date | string | token | reference | composite | quantity | uri
    Type,
    /// The uri that identifies the search parameter
    Url,
    /// The business version of the search parameter
    Version,
}
impl ResourceSearchParameterDefinition for SearchParameterSearchParameter {
    fn resource_type(&self) -> &'static str {
        "SearchParameter"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Base => "base",
            Self::Code => "code",
            Self::Component => "component",
            Self::Date => "date",
            Self::DerivedFrom => "derived-from",
            Self::Description => "description",
            Self::Jurisdiction => "jurisdiction",
            Self::Name => "name",
            Self::Publisher => "publisher",
            Self::Status => "status",
            Self::Target => "target",
            Self::Type => "type",
            Self::Url => "url",
            Self::Version => "version",
        }
    }
}
/// Search parameters for the Specimen resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SpecimenSearchParameter {
    /// The accession number associated with the specimen
    Accession,
    /// The code for the body site from where the specimen originated
    Bodysite,
    /// The date the specimen was collected
    Collected,
    /// Who collected the specimen
    Collector,
    /// The kind of specimen container
    Container,
    /// The unique identifier associated with the specimen container
    ContainerId,
    /// The unique identifier associated with the specimen
    Identifier,
    /// The parent of the specimen
    Parent,
    /// The patient the specimen comes from
    Patient,
    /// available | unavailable | unsatisfactory | entered-in-error
    Status,
    /// The subject of the specimen
    Subject,
    /// The specimen type
    Type,
}
impl ResourceSearchParameterDefinition for SpecimenSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Specimen"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Accession => "accession",
            Self::Bodysite => "bodysite",
            Self::Collected => "collected",
            Self::Collector => "collector",
            Self::Container => "container",
            Self::ContainerId => "container-id",
            Self::Identifier => "identifier",
            Self::Parent => "parent",
            Self::Patient => "patient",
            Self::Status => "status",
            Self::Subject => "subject",
            Self::Type => "type",
        }
    }
}
/// Search parameters for the StructureDefinition resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum StructureDefinitionSearchParameter {
    /// Whether the structure is abstract
    Abstract,
    /// Definition that this type is constrained/specialized from
    Base,
    /// Path that identifies the base element
    BasePath,
    /// resource | datatype | extension
    ContextType,
    /// The structure definition publication date
    Date,
    /// specialization | constraint - How relates to base definition
    Derivation,
    /// The description of the structure definition
    Description,
    /// For testing purposes, not real usage
    Experimental,
    /// Where the extension can be used in instances
    ExtContext,
    /// External identifier for the structure definition
    Identifier,
    /// Intended jurisdiction for the structure definition
    Jurisdiction,
    /// A code for the profile
    Keyword,
    /// primitive-type | complex-type | resource | logical
    Kind,
    /// Computationally friendly name of the structure definition
    Name,
    /// A path that is constrained in the profile
    Path,
    /// Name of the publisher of the structure definition
    Publisher,
    /// The current status of the structure definition
    Status,
    /// The human-friendly name of the structure definition
    Title,
    /// Type defined or constrained by this structure
    Type,
    /// The uri that identifies the structure definition
    Url,
    /// A vocabulary binding reference
    Valueset,
    /// The business version of the structure definition
    Version,
}
impl ResourceSearchParameterDefinition for StructureDefinitionSearchParameter {
    fn resource_type(&self) -> &'static str {
        "StructureDefinition"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Abstract => "abstract",
            Self::Base => "base",
            Self::BasePath => "base-path",
            Self::ContextType => "context-type",
            Self::Date => "date",
            Self::Derivation => "derivation",
            Self::Description => "description",
            Self::Experimental => "experimental",
            Self::ExtContext => "ext-context",
            Self::Identifier => "identifier",
            Self::Jurisdiction => "jurisdiction",
            Self::Keyword => "keyword",
            Self::Kind => "kind",
            Self::Name => "name",
            Self::Path => "path",
            Self::Publisher => "publisher",
            Self::Status => "status",
            Self::Title => "title",
            Self::Type => "type",
            Self::Url => "url",
            Self::Valueset => "valueset",
            Self::Version => "version",
        }
    }
}
/// Search parameters for the StructureMap resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum StructureMapSearchParameter {
    /// The structure map publication date
    Date,
    /// The description of the structure map
    Description,
    /// External identifier for the structure map
    Identifier,
    /// Intended jurisdiction for the structure map
    Jurisdiction,
    /// Computationally friendly name of the structure map
    Name,
    /// Name of the publisher of the structure map
    Publisher,
    /// The current status of the structure map
    Status,
    /// The human-friendly name of the structure map
    Title,
    /// The uri that identifies the structure map
    Url,
    /// The business version of the structure map
    Version,
}
impl ResourceSearchParameterDefinition for StructureMapSearchParameter {
    fn resource_type(&self) -> &'static str {
        "StructureMap"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Date => "date",
            Self::Description => "description",
            Self::Identifier => "identifier",
            Self::Jurisdiction => "jurisdiction",
            Self::Name => "name",
            Self::Publisher => "publisher",
            Self::Status => "status",
            Self::Title => "title",
            Self::Url => "url",
            Self::Version => "version",
        }
    }
}
/// Search parameters for the Subscription resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SubscriptionSearchParameter {
    /// A tag to be added to the resource matching the criteria
    AddTag,
    /// Contact details for the subscription
    Contact,
    /// The search rules used to determine when to send a notification
    Criteria,
    /// The mime-type of the notification payload
    Payload,
    /// The current state of the subscription
    Status,
    /// The type of channel for the sent notifications
    Type,
    /// The uri that will receive the notifications
    Url,
}
impl ResourceSearchParameterDefinition for SubscriptionSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Subscription"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::AddTag => "add-tag",
            Self::Contact => "contact",
            Self::Criteria => "criteria",
            Self::Payload => "payload",
            Self::Status => "status",
            Self::Type => "type",
            Self::Url => "url",
        }
    }
}
/// Search parameters for the Substance resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SubstanceSearchParameter {
    /// The category of the substance
    Category,
    /// The code of the substance or ingredient
    Code,
    /// Identifier of the package/container
    ContainerIdentifier,
    /// Expiry date of package or container of substance
    Expiry,
    /// Unique identifier for the substance
    Identifier,
    /// Amount of substance in the package
    Quantity,
    /// active | inactive | entered-in-error
    Status,
    /// A component of the substance
    SubstanceReference,
}
impl ResourceSearchParameterDefinition for SubstanceSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Substance"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Category => "category",
            Self::Code => "code",
            Self::ContainerIdentifier => "container-identifier",
            Self::Expiry => "expiry",
            Self::Identifier => "identifier",
            Self::Quantity => "quantity",
            Self::Status => "status",
            Self::SubstanceReference => "substance-reference",
        }
    }
}
/// Search parameters for the SupplyDelivery resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SupplyDeliverySearchParameter {
    /// External identifier
    Identifier,
    /// Patient for whom the item is supplied
    Patient,
    /// Who collected the Supply
    Receiver,
    /// in-progress | completed | abandoned | entered-in-error
    Status,
    /// Dispenser
    Supplier,
}
impl ResourceSearchParameterDefinition for SupplyDeliverySearchParameter {
    fn resource_type(&self) -> &'static str {
        "SupplyDelivery"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Receiver => "receiver",
            Self::Status => "status",
            Self::Supplier => "supplier",
        }
    }
}
/// Search parameters for the SupplyRequest resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SupplyRequestSearchParameter {
    /// When the request was made
    Date,
    /// Unique identifier
    Identifier,
    /// The kind of supply (central, non-stock, etc.)
    Category,
    /// Individual making the request
    Requester,
    /// draft | active | suspended +
    Status,
    /// Who is intended to fulfill the request
    Supplier,
}
impl ResourceSearchParameterDefinition for SupplyRequestSearchParameter {
    fn resource_type(&self) -> &'static str {
        "SupplyRequest"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Date => "date",
            Self::Identifier => "identifier",
            Self::Category => "category",
            Self::Requester => "requester",
            Self::Status => "status",
            Self::Supplier => "supplier",
        }
    }
}
/// Search parameters for the TestReport resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TestReportSearchParameter {
    /// An external identifier for the test report
    Identifier,
    /// The test report generation date
    Issued,
    /// The reference to a participant in the test execution
    Participant,
    /// The result disposition of the test execution
    Result,
    /// The name of the testing organization
    Tester,
    /// The test script executed to produce this report
    Testscript,
}
impl ResourceSearchParameterDefinition for TestReportSearchParameter {
    fn resource_type(&self) -> &'static str {
        "TestReport"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Issued => "issued",
            Self::Participant => "participant",
            Self::Result => "result",
            Self::Tester => "tester",
            Self::Testscript => "testscript",
        }
    }
}
/// Search parameters for the TestScript resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TestScriptSearchParameter {
    /// The test script publication date
    Date,
    /// The description of the test script
    Description,
    /// External identifier for the test script
    Identifier,
    /// Intended jurisdiction for the test script
    Jurisdiction,
    /// Computationally friendly name of the test script
    Name,
    /// Name of the publisher of the test script
    Publisher,
    /// The current status of the test script
    Status,
    /// TestScript required and validated capability
    TestscriptCapability,
    /// The human-friendly name of the test script
    Title,
    /// The uri that identifies the test script
    Url,
    /// The business version of the test script
    Version,
}
impl ResourceSearchParameterDefinition for TestScriptSearchParameter {
    fn resource_type(&self) -> &'static str {
        "TestScript"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Date => "date",
            Self::Description => "description",
            Self::Identifier => "identifier",
            Self::Jurisdiction => "jurisdiction",
            Self::Name => "name",
            Self::Publisher => "publisher",
            Self::Status => "status",
            Self::TestscriptCapability => "testscript-capability",
            Self::Title => "title",
            Self::Url => "url",
            Self::Version => "version",
        }
    }
}
/// Search parameters for the ValueSet resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ValueSetSearchParameter {
    /// The value set publication date
    Date,
    /// The description of the value set
    Description,
    /// Uniquely identifies this expansion
    Expansion,
    /// External identifier for the value set
    Identifier,
    /// Intended jurisdiction for the value set
    Jurisdiction,
    /// Computationally friendly name of the value set
    Name,
    /// Name of the publisher of the value set
    Publisher,
    /// A code system included or excluded in the value set or an imported value set
    Reference,
    /// The current status of the value set
    Status,
    /// The human-friendly name of the value set
    Title,
    /// The uri that identifies the value set
    Url,
    /// The business version of the value set
    Version,
}
impl ResourceSearchParameterDefinition for ValueSetSearchParameter {
    fn resource_type(&self) -> &'static str {
        "ValueSet"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Date => "date",
            Self::Description => "description",
            Self::Expansion => "expansion",
            Self::Identifier => "identifier",
            Self::Jurisdiction => "jurisdiction",
            Self::Name => "name",
            Self::Publisher => "publisher",
            Self::Reference => "reference",
            Self::Status => "status",
            Self::Title => "title",
            Self::Url => "url",
            Self::Version => "version",
        }
    }
}
impl SearchableResource for resources::ActivityDefinition {
    /// Parameters that can be used when searching ActivityDefinition resources
    type Params = ActivityDefinitionSearchParameter;
}
impl SearchableResource for resources::AdverseEvent {
    /// Parameters that can be used when searching AdverseEvent resources
    type Params = AdverseEventSearchParameter;
}
impl SearchableResource for resources::AllergyIntolerance {
    /// Parameters that can be used when searching AllergyIntolerance resources
    type Params = AllergyIntoleranceSearchParameter;
}
impl SearchableResource for resources::Appointment {
    /// Parameters that can be used when searching Appointment resources
    type Params = AppointmentSearchParameter;
}
impl SearchableResource for resources::AuditEvent {
    /// Parameters that can be used when searching AuditEvent resources
    type Params = AuditEventSearchParameter;
}
impl SearchableResource for resources::Binary {
    /// Parameters that can be used when searching Binary resources
    type Params = BinarySearchParameter;
}
impl SearchableResource for resources::BodySite {
    /// Parameters that can be used when searching BodySite resources
    type Params = BodySiteSearchParameter;
}
impl SearchableResource for resources::Bundle {
    /// Parameters that can be used when searching Bundle resources
    type Params = BundleSearchParameter;
}
impl SearchableResource for resources::CapabilityStatement {
    /// Parameters that can be used when searching CapabilityStatement resources
    type Params = CapabilityStatementSearchParameter;
}
impl SearchableResource for resources::CarePlan {
    /// Parameters that can be used when searching CarePlan resources
    type Params = CarePlanSearchParameter;
}
impl SearchableResource for resources::CareTeam {
    /// Parameters that can be used when searching CareTeam resources
    type Params = CareTeamSearchParameter;
}
impl SearchableResource for resources::ChargeItem {
    /// Parameters that can be used when searching ChargeItem resources
    type Params = ChargeItemSearchParameter;
}
impl SearchableResource for resources::CodeSystem {
    /// Parameters that can be used when searching CodeSystem resources
    type Params = CodeSystemSearchParameter;
}
impl SearchableResource for resources::CompartmentDefinition {
    /// Parameters that can be used when searching CompartmentDefinition resources
    type Params = CompartmentDefinitionSearchParameter;
}
impl SearchableResource for resources::Composition {
    /// Parameters that can be used when searching Composition resources
    type Params = CompositionSearchParameter;
}
impl SearchableResource for resources::ConceptMap {
    /// Parameters that can be used when searching ConceptMap resources
    type Params = ConceptMapSearchParameter;
}
impl SearchableResource for resources::Condition {
    /// Parameters that can be used when searching Condition resources
    type Params = ConditionSearchParameter;
}
impl SearchableResource for resources::Consent {
    /// Parameters that can be used when searching Consent resources
    type Params = ConsentSearchParameter;
}
impl SearchableResource for resources::Contract {
    /// Parameters that can be used when searching Contract resources
    type Params = ContractSearchParameter;
}
impl SearchableResource for resources::Coverage {
    /// Parameters that can be used when searching Coverage resources
    type Params = CoverageSearchParameter;
}
impl SearchableResource for resources::DetectedIssue {
    /// Parameters that can be used when searching DetectedIssue resources
    type Params = DetectedIssueSearchParameter;
}
impl SearchableResource for resources::Device {
    /// Parameters that can be used when searching Device resources
    type Params = DeviceSearchParameter;
}
impl SearchableResource for resources::DeviceComponent {
    /// Parameters that can be used when searching DeviceComponent resources
    type Params = DeviceComponentSearchParameter;
}
impl SearchableResource for resources::DeviceMetric {
    /// Parameters that can be used when searching DeviceMetric resources
    type Params = DeviceMetricSearchParameter;
}
impl SearchableResource for resources::DeviceRequest {
    /// Parameters that can be used when searching DeviceRequest resources
    type Params = DeviceRequestSearchParameter;
}
impl SearchableResource for resources::DeviceUseStatement {
    /// Parameters that can be used when searching DeviceUseStatement resources
    type Params = DeviceUseStatementSearchParameter;
}
impl SearchableResource for resources::DiagnosticReport {
    /// Parameters that can be used when searching DiagnosticReport resources
    type Params = DiagnosticReportSearchParameter;
}
impl SearchableResource for resources::DocumentReference {
    /// Parameters that can be used when searching DocumentReference resources
    type Params = DocumentReferenceSearchParameter;
}
impl SearchableResource for resources::Encounter {
    /// Parameters that can be used when searching Encounter resources
    type Params = EncounterSearchParameter;
}
impl SearchableResource for resources::EpisodeOfCare {
    /// Parameters that can be used when searching EpisodeOfCare resources
    type Params = EpisodeOfCareSearchParameter;
}
impl SearchableResource for resources::ExpansionProfile {
    /// Parameters that can be used when searching ExpansionProfile resources
    type Params = ExpansionProfileSearchParameter;
}
impl SearchableResource for resources::Flag {
    /// Parameters that can be used when searching Flag resources
    type Params = FlagSearchParameter;
}
impl SearchableResource for resources::GraphDefinition {
    /// Parameters that can be used when searching GraphDefinition resources
    type Params = GraphDefinitionSearchParameter;
}
impl SearchableResource for resources::GuidanceResponse {
    /// Parameters that can be used when searching GuidanceResponse resources
    type Params = GuidanceResponseSearchParameter;
}
impl SearchableResource for resources::ImagingManifest {
    /// Parameters that can be used when searching ImagingManifest resources
    type Params = ImagingManifestSearchParameter;
}
impl SearchableResource for resources::Immunization {
    /// Parameters that can be used when searching Immunization resources
    type Params = ImmunizationSearchParameter;
}
impl SearchableResource for resources::ImmunizationRecommendation {
    /// Parameters that can be used when searching ImmunizationRecommendation resources
    type Params = ImmunizationRecommendationSearchParameter;
}
impl SearchableResource for resources::ImplementationGuide {
    /// Parameters that can be used when searching ImplementationGuide resources
    type Params = ImplementationGuideSearchParameter;
}
impl SearchableResource for resources::Linkage {
    /// Parameters that can be used when searching Linkage resources
    type Params = LinkageSearchParameter;
}
impl SearchableResource for resources::Location {
    /// Parameters that can be used when searching Location resources
    type Params = LocationSearchParameter;
}
impl SearchableResource for resources::MeasureReport {
    /// Parameters that can be used when searching MeasureReport resources
    type Params = MeasureReportSearchParameter;
}
impl SearchableResource for resources::Medication {
    /// Parameters that can be used when searching Medication resources
    type Params = MedicationSearchParameter;
}
impl SearchableResource for resources::MedicationAdministration {
    /// Parameters that can be used when searching MedicationAdministration resources
    type Params = MedicationAdministrationSearchParameter;
}
impl SearchableResource for resources::MedicationDispense {
    /// Parameters that can be used when searching MedicationDispense resources
    type Params = MedicationDispenseSearchParameter;
}
impl SearchableResource for resources::MedicationRequest {
    /// Parameters that can be used when searching MedicationRequest resources
    type Params = MedicationRequestSearchParameter;
}
impl SearchableResource for resources::MedicationStatement {
    /// Parameters that can be used when searching MedicationStatement resources
    type Params = MedicationStatementSearchParameter;
}
impl SearchableResource for resources::MessageDefinition {
    /// Parameters that can be used when searching MessageDefinition resources
    type Params = MessageDefinitionSearchParameter;
}
impl SearchableResource for resources::MessageHeader {
    /// Parameters that can be used when searching MessageHeader resources
    type Params = MessageHeaderSearchParameter;
}
impl SearchableResource for resources::NamingSystem {
    /// Parameters that can be used when searching NamingSystem resources
    type Params = NamingSystemSearchParameter;
}
impl SearchableResource for resources::NutritionOrder {
    /// Parameters that can be used when searching NutritionOrder resources
    type Params = NutritionOrderSearchParameter;
}
impl SearchableResource for resources::Observation {
    /// Parameters that can be used when searching Observation resources
    type Params = ObservationSearchParameter;
}
impl SearchableResource for resources::OperationDefinition {
    /// Parameters that can be used when searching OperationDefinition resources
    type Params = OperationDefinitionSearchParameter;
}
impl SearchableResource for resources::Organization {
    /// Parameters that can be used when searching Organization resources
    type Params = OrganizationSearchParameter;
}
impl SearchableResource for resources::Patient {
    /// Parameters that can be used when searching Patient resources
    type Params = PatientSearchParameter;
}
impl SearchableResource for resources::Person {
    /// Parameters that can be used when searching Person resources
    type Params = PersonSearchParameter;
}
impl SearchableResource for resources::PlanDefinition {
    /// Parameters that can be used when searching PlanDefinition resources
    type Params = PlanDefinitionSearchParameter;
}
impl SearchableResource for resources::Practitioner {
    /// Parameters that can be used when searching Practitioner resources
    type Params = PractitionerSearchParameter;
}
impl SearchableResource for resources::PractitionerRole {
    /// Parameters that can be used when searching PractitionerRole resources
    type Params = PractitionerRoleSearchParameter;
}
impl SearchableResource for resources::Procedure {
    /// Parameters that can be used when searching Procedure resources
    type Params = ProcedureSearchParameter;
}
impl SearchableResource for resources::ProcedureRequest {
    /// Parameters that can be used when searching ProcedureRequest resources
    type Params = ProcedureRequestSearchParameter;
}
impl SearchableResource for resources::Provenance {
    /// Parameters that can be used when searching Provenance resources
    type Params = ProvenanceSearchParameter;
}
impl SearchableResource for resources::Questionnaire {
    /// Parameters that can be used when searching Questionnaire resources
    type Params = QuestionnaireSearchParameter;
}
impl SearchableResource for resources::QuestionnaireResponse {
    /// Parameters that can be used when searching QuestionnaireResponse resources
    type Params = QuestionnaireResponseSearchParameter;
}
impl SearchableResource for resources::RelatedPerson {
    /// Parameters that can be used when searching RelatedPerson resources
    type Params = RelatedPersonSearchParameter;
}
impl SearchableResource for resources::ResearchSubject {
    /// Parameters that can be used when searching ResearchSubject resources
    type Params = ResearchSubjectSearchParameter;
}
impl SearchableResource for resources::RiskAssessment {
    /// Parameters that can be used when searching RiskAssessment resources
    type Params = RiskAssessmentSearchParameter;
}
impl SearchableResource for resources::SearchParameter {
    /// Parameters that can be used when searching SearchParameter resources
    type Params = SearchParameterSearchParameter;
}
impl SearchableResource for resources::Specimen {
    /// Parameters that can be used when searching Specimen resources
    type Params = SpecimenSearchParameter;
}
impl SearchableResource for resources::StructureDefinition {
    /// Parameters that can be used when searching StructureDefinition resources
    type Params = StructureDefinitionSearchParameter;
}
impl SearchableResource for resources::StructureMap {
    /// Parameters that can be used when searching StructureMap resources
    type Params = StructureMapSearchParameter;
}
impl SearchableResource for resources::Subscription {
    /// Parameters that can be used when searching Subscription resources
    type Params = SubscriptionSearchParameter;
}
impl SearchableResource for resources::Substance {
    /// Parameters that can be used when searching Substance resources
    type Params = SubstanceSearchParameter;
}
impl SearchableResource for resources::SupplyDelivery {
    /// Parameters that can be used when searching SupplyDelivery resources
    type Params = SupplyDeliverySearchParameter;
}
impl SearchableResource for resources::SupplyRequest {
    /// Parameters that can be used when searching SupplyRequest resources
    type Params = SupplyRequestSearchParameter;
}
impl SearchableResource for resources::TestReport {
    /// Parameters that can be used when searching TestReport resources
    type Params = TestReportSearchParameter;
}
impl SearchableResource for resources::TestScript {
    /// Parameters that can be used when searching TestScript resources
    type Params = TestScriptSearchParameter;
}
impl SearchableResource for resources::ValueSet {
    /// Parameters that can be used when searching ValueSet resources
    type Params = ValueSetSearchParameter;
}
