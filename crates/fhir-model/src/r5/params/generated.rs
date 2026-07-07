//! Generated code! Take a look at the generator-crate for changing this file!
#![allow(clippy::too_many_lines)]
use crate::params::*;
use super::super::resources;
/// Search parameters for the ActivityDefinition resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ActivityDefinitionSearchParameter {
    /// What resource is being referenced
    ComposedOf,
    /// A use context assigned to the activity definition
    Context,
    /// A quantity- or range-valued use context assigned to the activity definition
    ContextQuantity,
    /// A type of use context assigned to the activity definition
    ContextType,
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
    /// The kind of activity definition
    Kind,
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
    /// A use context type and quantity- or range-based value assigned to the activity definition
    ContextTypeQuantity,
    /// A use context type and value assigned to the activity definition
    ContextTypeValue,
}
impl ResourceSearchParameterDefinition for ActivityDefinitionSearchParameter {
    fn resource_type(&self) -> &'static str {
        "ActivityDefinition"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::ComposedOf => "composed-of",
            Self::Context => "context",
            Self::ContextQuantity => "context-quantity",
            Self::ContextType => "context-type",
            Self::Date => "date",
            Self::DependsOn => "depends-on",
            Self::DerivedFrom => "derived-from",
            Self::Description => "description",
            Self::Effective => "effective",
            Self::Identifier => "identifier",
            Self::Jurisdiction => "jurisdiction",
            Self::Kind => "kind",
            Self::Name => "name",
            Self::Predecessor => "predecessor",
            Self::Publisher => "publisher",
            Self::Status => "status",
            Self::Successor => "successor",
            Self::Title => "title",
            Self::Topic => "topic",
            Self::Url => "url",
            Self::Version => "version",
            Self::ContextTypeQuantity => "context-type-quantity",
            Self::ContextTypeValue => "context-type-value",
        }
    }
}
/// Search parameters for the ActorDefinition resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ActorDefinitionSearchParameter {
    /// A use context assigned to the Actor Definition
    Context,
    /// A quantity- or range-valued use context assigned to the Actor Definition
    ContextQuantity,
    /// A type of use context assigned to the Actor Definition
    ContextType,
    /// The Actor Definition publication date
    Date,
    /// The description of the Actor Definition
    Description,
    /// External identifier for the Actor Definition
    Identifier,
    /// Intended jurisdiction for the Actor Definition
    Jurisdiction,
    /// Name of the publisher of the Actor Definition
    Publisher,
    /// The current status of the Actor Definition
    Status,
    /// The human-friendly name of the Actor Definition
    Title,
    /// The uri that identifies the Actor Definition
    Url,
    /// The business version of the Actor Definition
    Version,
    /// A use context type and quantity- or range-based value assigned to the Actor Definition
    ContextTypeQuantity,
    /// A use context type and value assigned to the Actor Definition
    ContextTypeValue,
    /// The type of actor
    Type,
}
impl ResourceSearchParameterDefinition for ActorDefinitionSearchParameter {
    fn resource_type(&self) -> &'static str {
        "ActorDefinition"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Context => "context",
            Self::ContextQuantity => "context-quantity",
            Self::ContextType => "context-type",
            Self::Date => "date",
            Self::Description => "description",
            Self::Identifier => "identifier",
            Self::Jurisdiction => "jurisdiction",
            Self::Publisher => "publisher",
            Self::Status => "status",
            Self::Title => "title",
            Self::Url => "url",
            Self::Version => "version",
            Self::ContextTypeQuantity => "context-type-quantity",
            Self::ContextTypeValue => "context-type-value",
            Self::Type => "type",
        }
    }
}
/// Search parameters for the AdministrableProductDefinition resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AdministrableProductDefinitionSearchParameter {
    /// A device that is integral to the medicinal product, in effect being considered as an "ingredient" of the medicinal product. This is not intended for devices that are just co-packaged
    Device,
    /// The administrable dose form, i.e. the dose form of the final product after necessary reconstitution or processing
    DoseForm,
    /// The medicinal product that this is an administrable form of. This is not a reference to the item(s) that make up this administrable form - it is the whole product
    FormOf,
    /// An identifier for the administrable product
    Identifier,
    /// The ingredients of this administrable medicinal product
    Ingredient,
    /// The manufactured item(s) that this administrable product is produced from. Either a single item, or several that are mixed before administration (e.g. a power item and a solution item). Note that these are not raw ingredients
    ManufacturedItem,
    /// Coded expression for the route
    Route,
    /// The status of this administrable product. Enables tracking the life-cycle of the content.
    Status,
    /// Coded expression for the species
    TargetSpecies,
}
impl ResourceSearchParameterDefinition
for AdministrableProductDefinitionSearchParameter {
    fn resource_type(&self) -> &'static str {
        "AdministrableProductDefinition"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Device => "device",
            Self::DoseForm => "dose-form",
            Self::FormOf => "form-of",
            Self::Identifier => "identifier",
            Self::Ingredient => "ingredient",
            Self::ManufacturedItem => "manufactured-item",
            Self::Route => "route",
            Self::Status => "status",
            Self::TargetSpecies => "target-species",
        }
    }
}
/// Search parameters for the AdverseEvent resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AdverseEventSearchParameter {
    /// Business identifier for the event
    Identifier,
    /// Subject impacted by event
    Patient,
    /// actual | potential
    Actuality,
    /// wrong-patient | procedure-mishap | medication-mishap | device | unsafe-physical-environment | hospital-aquired-infection | wrong-body-site
    Category,
    /// Event or incident that occurred or was averted
    Code,
    /// When the event occurred
    Date,
    /// Location where adverse event occurred
    Location,
    /// Who recorded the adverse event
    Recorder,
    /// Effect on the subject due to this event
    Resultingeffect,
    /// Seriousness or gravity of the event
    Seriousness,
    /// in-progress | completed | entered-in-error | unknown
    Status,
    /// Research study that the subject is enrolled in
    Study,
    /// Subject impacted by event
    Subject,
    /// Refers to the specific entity that caused the adverse event
    Substance,
}
impl ResourceSearchParameterDefinition for AdverseEventSearchParameter {
    fn resource_type(&self) -> &'static str {
        "AdverseEvent"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Actuality => "actuality",
            Self::Category => "category",
            Self::Code => "code",
            Self::Date => "date",
            Self::Location => "location",
            Self::Recorder => "recorder",
            Self::Resultingeffect => "resultingeffect",
            Self::Seriousness => "seriousness",
            Self::Status => "status",
            Self::Study => "study",
            Self::Subject => "subject",
            Self::Substance => "substance",
        }
    }
}
/// Search parameters for the AllergyIntolerance resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AllergyIntoleranceSearchParameter {
    /// External ids for this item
    Identifier,
    /// Who the sensitivity is for
    Patient,
    /// allergy | intolerance - Underlying mechanism (if known)
    Type,
    /// Code that identifies the allergy or intolerance
    Code,
    /// Date first version of the resource instance was recorded
    Date,
    /// food | medication | environment | biologic
    Category,
    /// active | inactive | resolved
    ClinicalStatus,
    /// low | high | unable-to-assess
    Criticality,
    /// Date(/time) of last known occurrence of a reaction
    LastDate,
    /// Clinical symptoms/signs associated with the Event
    ManifestationCode,
    /// Clinical symptoms/signs associated with the Event
    ManifestationReference,
    /// Who or what participated in the activities related to the allergy or intolerance
    Participant,
    /// How the subject was exposed to the substance
    Route,
    /// mild | moderate | severe (of event as a whole)
    Severity,
    /// unconfirmed | presumed | confirmed | refuted | entered-in-error
    VerificationStatus,
}
impl ResourceSearchParameterDefinition for AllergyIntoleranceSearchParameter {
    fn resource_type(&self) -> &'static str {
        "AllergyIntolerance"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Type => "type",
            Self::Code => "code",
            Self::Date => "date",
            Self::Category => "category",
            Self::ClinicalStatus => "clinical-status",
            Self::Criticality => "criticality",
            Self::LastDate => "last-date",
            Self::ManifestationCode => "manifestation-code",
            Self::ManifestationReference => "manifestation-reference",
            Self::Participant => "participant",
            Self::Route => "route",
            Self::Severity => "severity",
            Self::VerificationStatus => "verification-status",
        }
    }
}
/// Search parameters for the Appointment resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AppointmentSearchParameter {
    /// An Identifier of the Appointment
    Identifier,
    /// One of the individuals of the appointment is this patient
    Patient,
    /// Appointment date/time.
    Date,
    /// Any one of the individuals participating in the appointment
    Actor,
    /// The style of appointment or patient that has been booked in the slot (not service type)
    AppointmentType,
    /// The service request this appointment is allocated to assess
    BasedOn,
    /// One of the individuals of the appointment is this patient
    Group,
    /// This location is listed in the participants of the appointment
    Location,
    /// The Participation status of the subject, or other participant on the appointment. Can be used to locate participants that have not responded to meeting requests.
    PartStatus,
    /// One of the individuals of the appointment is this practitioner
    Practitioner,
    /// Reference to a concept (by class)
    ReasonCode,
    /// Reference to a resource (by instance)
    ReasonReference,
    /// During what period was the Appointment requested to take place
    RequestedPeriod,
    /// A broad categorization of the service that is to be performed during this appointment
    ServiceCategory,
    /// The specific service (by coding) that is to be performed during this appointment
    ServiceType,
    /// The specific service (by HealthcareService) that is to be performed during this appointment
    ServiceTypeReference,
    /// The slots that this appointment is filling
    Slot,
    /// The specialty of a practitioner that would be required to perform the service requested in this appointment
    Specialty,
    /// The overall status of the appointment
    Status,
    /// One of the individuals of the appointment is this patient
    Subject,
    /// Additional information to support the appointment
    SupportingInfo,
}
impl ResourceSearchParameterDefinition for AppointmentSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Appointment"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Date => "date",
            Self::Actor => "actor",
            Self::AppointmentType => "appointment-type",
            Self::BasedOn => "based-on",
            Self::Group => "group",
            Self::Location => "location",
            Self::PartStatus => "part-status",
            Self::Practitioner => "practitioner",
            Self::ReasonCode => "reason-code",
            Self::ReasonReference => "reason-reference",
            Self::RequestedPeriod => "requested-period",
            Self::ServiceCategory => "service-category",
            Self::ServiceType => "service-type",
            Self::ServiceTypeReference => "service-type-reference",
            Self::Slot => "slot",
            Self::Specialty => "specialty",
            Self::Status => "status",
            Self::Subject => "subject",
            Self::SupportingInfo => "supporting-info",
        }
    }
}
/// Search parameters for the ArtifactAssessment resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ArtifactAssessmentSearchParameter {
    /// The artifact assessment publication date
    Date,
    /// The artifact assessment identifier
    Identifier,
}
impl ResourceSearchParameterDefinition for ArtifactAssessmentSearchParameter {
    fn resource_type(&self) -> &'static str {
        "ArtifactAssessment"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Date => "date",
            Self::Identifier => "identifier",
        }
    }
}
/// Search parameters for the AuditEvent resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AuditEventSearchParameter {
    /// Where the activity involved patient data
    Patient,
    /// More specific code for the event
    Code,
    /// Time when the event was recorded
    Date,
    /// Type of action performed during the event
    Action,
    /// Identifier of who
    Agent,
    /// Agent role in the event
    AgentRole,
    /// Reference to the service request.
    BasedOn,
    /// Category of event
    Category,
    /// Encounter related to the activity recorded in the AuditEvent
    Encounter,
    /// Specific instance of resource
    Entity,
    /// What role the entity played
    EntityRole,
    /// Whether the event succeeded or failed
    Outcome,
    /// Policy that authorized event
    Policy,
    /// The authorization (purposeOfUse) of the event
    Purpose,
    /// The identity of source detecting the event
    Source,
}
impl ResourceSearchParameterDefinition for AuditEventSearchParameter {
    fn resource_type(&self) -> &'static str {
        "AuditEvent"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Patient => "patient",
            Self::Code => "code",
            Self::Date => "date",
            Self::Action => "action",
            Self::Agent => "agent",
            Self::AgentRole => "agent-role",
            Self::BasedOn => "based-on",
            Self::Category => "category",
            Self::Encounter => "encounter",
            Self::Entity => "entity",
            Self::EntityRole => "entity-role",
            Self::Outcome => "outcome",
            Self::Policy => "policy",
            Self::Purpose => "purpose",
            Self::Source => "source",
        }
    }
}
/// Search parameters for the BiologicallyDerivedProduct resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BiologicallyDerivedProductSearchParameter {
    /// The biological source for the biologically derived product
    BiologicalSourceEvent,
    /// A code that identifies the kind of this biologically derived product (SNOMED CT code).
    Code,
    /// Procedure request to obtain this biologically derived product.
    Collector,
    /// Identifier
    Identifier,
    /// Broad category of this product.
    ProductCategory,
    /// Whether the product is currently available.
    ProductStatus,
    /// Procedure request to obtain this biologically derived product.
    Request,
    /// Identifier
    SerialNumber,
}
impl ResourceSearchParameterDefinition for BiologicallyDerivedProductSearchParameter {
    fn resource_type(&self) -> &'static str {
        "BiologicallyDerivedProduct"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::BiologicalSourceEvent => "biological-source-event",
            Self::Code => "code",
            Self::Collector => "collector",
            Self::Identifier => "identifier",
            Self::ProductCategory => "product-category",
            Self::ProductStatus => "product-status",
            Self::Request => "request",
            Self::SerialNumber => "serial-number",
        }
    }
}
/// Search parameters for the BiologicallyDerivedProductDispense resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BiologicallyDerivedProductDispenseSearchParameter {
    /// The identifier of the dispense
    Identifier,
    /// The identity of a patient for whom to list dispenses
    Patient,
    /// The identity of a patient for whom to list dispenses
    Performer,
    /// Search for products that match this code
    Product,
    /// The status of the dispense
    Status,
}
impl ResourceSearchParameterDefinition
for BiologicallyDerivedProductDispenseSearchParameter {
    fn resource_type(&self) -> &'static str {
        "BiologicallyDerivedProductDispense"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Performer => "performer",
            Self::Product => "product",
            Self::Status => "status",
        }
    }
}
/// Search parameters for the BodyStructure resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BodyStructureSearchParameter {
    /// Bodystructure identifier
    Identifier,
    /// Who this is about
    Patient,
    /// Body site excludedStructure structure
    ExcludedStructure,
    /// Body site includedStructure structure
    IncludedStructure,
    /// Kind of Structure
    Morphology,
}
impl ResourceSearchParameterDefinition for BodyStructureSearchParameter {
    fn resource_type(&self) -> &'static str {
        "BodyStructure"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::ExcludedStructure => "excluded_structure",
            Self::IncludedStructure => "included_structure",
            Self::Morphology => "morphology",
        }
    }
}
/// Search parameters for the Bundle resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BundleSearchParameter {
    /// The first resource in the bundle, if the bundle type is "document" - this is a composition, and this parameter provides access to search its contents
    Composition,
    /// Persistent identifier for the bundle
    Identifier,
    /// The first resource in the bundle, if the bundle type is "message" - this is a message header, and this parameter provides access to search its contents
    Message,
    /// When the bundle was assembled
    Timestamp,
    /// document | message | transaction | transaction-response | batch | batch-response | history | searchset | collection | subscription-notification
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
            Self::Timestamp => "timestamp",
            Self::Type => "type",
        }
    }
}
/// Search parameters for the CapabilityStatement resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CapabilityStatementSearchParameter {
    /// A use context assigned to the capability statement
    Context,
    /// A quantity- or range-valued use context assigned to the capability statement
    ContextQuantity,
    /// A type of use context assigned to the capability statement
    ContextType,
    /// The capability statement publication date
    Date,
    /// The description of the capability statement
    Description,
    /// External identifier for the capability statement
    Identifier,
    /// Intended jurisdiction for the capability statement
    Jurisdiction,
    /// Computationally friendly name of the capability statement
    Name,
    /// Name of the publisher of the capability statement
    Publisher,
    /// The current status of the capability statement
    Status,
    /// The human-friendly name of the capability statement
    Title,
    /// The uri that identifies the capability statement
    Url,
    /// The business version of the capability statement
    Version,
    /// A use context type and quantity- or range-based value assigned to the capability statement
    ContextTypeQuantity,
    /// A use context type and value assigned to the capability statement
    ContextTypeValue,
    /// The version of FHIR
    Fhirversion,
    /// formats supported (xml | json | ttl | mime type)
    Format,
    /// Implementation guides supported
    Guide,
    /// Mode - restful (server/client) or messaging (sender/receiver)
    Mode,
    /// Name of a resource mentioned in a capability statement
    Resource,
    /// A profile id invoked in a capability statement
    ResourceProfile,
    /// OAuth | SMART-on-FHIR | NTLM | Basic | Kerberos | Certificates
    SecurityService,
    /// Part of the name of a software application
    Software,
    /// Profiles for use cases supported
    SupportedProfile,
}
impl ResourceSearchParameterDefinition for CapabilityStatementSearchParameter {
    fn resource_type(&self) -> &'static str {
        "CapabilityStatement"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Context => "context",
            Self::ContextQuantity => "context-quantity",
            Self::ContextType => "context-type",
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
            Self::ContextTypeQuantity => "context-type-quantity",
            Self::ContextTypeValue => "context-type-value",
            Self::Fhirversion => "fhirversion",
            Self::Format => "format",
            Self::Guide => "guide",
            Self::Mode => "mode",
            Self::Resource => "resource",
            Self::ResourceProfile => "resource-profile",
            Self::SecurityService => "security-service",
            Self::Software => "software",
            Self::SupportedProfile => "supported-profile",
        }
    }
}
/// Search parameters for the CarePlan resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CarePlanSearchParameter {
    /// External Ids for this plan
    Identifier,
    /// Who the care plan is for
    Patient,
    /// Time period plan covers
    Date,
    /// The Encounter during which this CarePlan was created
    Encounter,
    /// Activity that is intended to be part of the care plan
    ActivityReference,
    /// Fulfills CarePlan
    BasedOn,
    /// Who's involved in plan?
    CareTeam,
    /// Type of plan
    Category,
    /// Reference to a resource (by instance)
    Condition,
    /// Who is the designated responsible party
    Custodian,
    /// Desired outcome of plan
    Goal,
    /// Instantiates FHIR protocol or definition
    InstantiatesCanonical,
    /// Instantiates external protocol or definition
    InstantiatesUri,
    /// proposal | plan | order | option | directive
    Intent,
    /// Part of referenced CarePlan
    PartOf,
    /// CarePlan replaced by this CarePlan
    Replaces,
    /// draft | active | on-hold | revoked | completed | entered-in-error | unknown
    Status,
    /// Who the care plan is for
    Subject,
}
impl ResourceSearchParameterDefinition for CarePlanSearchParameter {
    fn resource_type(&self) -> &'static str {
        "CarePlan"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Date => "date",
            Self::Encounter => "encounter",
            Self::ActivityReference => "activity-reference",
            Self::BasedOn => "based-on",
            Self::CareTeam => "care-team",
            Self::Category => "category",
            Self::Condition => "condition",
            Self::Custodian => "custodian",
            Self::Goal => "goal",
            Self::InstantiatesCanonical => "instantiates-canonical",
            Self::InstantiatesUri => "instantiates-uri",
            Self::Intent => "intent",
            Self::PartOf => "part-of",
            Self::Replaces => "replaces",
            Self::Status => "status",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the CareTeam resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CareTeamSearchParameter {
    /// External Ids for this team
    Identifier,
    /// Who care team is for
    Patient,
    /// A date within the coverage time period.
    Date,
    /// Type of team
    Category,
    /// Name of the team, such as crisis assessment team
    Name,
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
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Date => "date",
            Self::Category => "category",
            Self::Name => "name",
            Self::Participant => "participant",
            Self::Status => "status",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the ChargeItem resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ChargeItemSearchParameter {
    /// Business Identifier for item
    Identifier,
    /// Individual service was done for/to
    Patient,
    /// A code that identifies the charge, like a billing code
    Code,
    /// Encounter associated with event
    Encounter,
    /// Account to place this charge
    Account,
    /// Date the charge item was entered
    EnteredDate,
    /// Individual who was entering
    Enterer,
    /// Factor overriding the associated rules
    FactorOverride,
    /// When the charged service was applied
    Occurrence,
    /// Individual who was performing
    PerformerActor,
    /// What type of performance was done
    PerformerFunction,
    /// Organization providing the charged service
    PerformingOrganization,
    /// Price overriding the associated rules
    PriceOverride,
    /// Quantity of which the charge item has been serviced
    Quantity,
    /// Organization requesting the charged service
    RequestingOrganization,
    /// Which rendered service is being charged?
    Service,
    /// Is this charge item active
    Status,
    /// Individual service was done for/to
    Subject,
}
impl ResourceSearchParameterDefinition for ChargeItemSearchParameter {
    fn resource_type(&self) -> &'static str {
        "ChargeItem"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Code => "code",
            Self::Encounter => "encounter",
            Self::Account => "account",
            Self::EnteredDate => "entered-date",
            Self::Enterer => "enterer",
            Self::FactorOverride => "factor-override",
            Self::Occurrence => "occurrence",
            Self::PerformerActor => "performer-actor",
            Self::PerformerFunction => "performer-function",
            Self::PerformingOrganization => "performing-organization",
            Self::PriceOverride => "price-override",
            Self::Quantity => "quantity",
            Self::RequestingOrganization => "requesting-organization",
            Self::Service => "service",
            Self::Status => "status",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the ChargeItemDefinition resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ChargeItemDefinitionSearchParameter {
    /// A use context assigned to the charge item definition
    Context,
    /// A quantity- or range-valued use context assigned to the charge item definition
    ContextQuantity,
    /// A type of use context assigned to the charge item definition
    ContextType,
    /// The charge item definition publication date
    Date,
    /// The description of the charge item definition
    Description,
    /// The time during which the charge item definition is intended to be in use
    Effective,
    /// External identifier for the charge item definition
    Identifier,
    /// Intended jurisdiction for the charge item definition
    Jurisdiction,
    /// Name of the publisher of the charge item definition
    Publisher,
    /// The current status of the charge item definition
    Status,
    /// The human-friendly name of the charge item definition
    Title,
    /// The uri that identifies the charge item definition
    Url,
    /// The business version of the charge item definition
    Version,
    /// A use context type and quantity- or range-based value assigned to the charge item definition
    ContextTypeQuantity,
    /// A use context type and value assigned to the charge item definition
    ContextTypeValue,
}
impl ResourceSearchParameterDefinition for ChargeItemDefinitionSearchParameter {
    fn resource_type(&self) -> &'static str {
        "ChargeItemDefinition"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Context => "context",
            Self::ContextQuantity => "context-quantity",
            Self::ContextType => "context-type",
            Self::Date => "date",
            Self::Description => "description",
            Self::Effective => "effective",
            Self::Identifier => "identifier",
            Self::Jurisdiction => "jurisdiction",
            Self::Publisher => "publisher",
            Self::Status => "status",
            Self::Title => "title",
            Self::Url => "url",
            Self::Version => "version",
            Self::ContextTypeQuantity => "context-type-quantity",
            Self::ContextTypeValue => "context-type-value",
        }
    }
}
/// Search parameters for the Citation resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CitationSearchParameter {
    /// A use context assigned to the citation
    Context,
    /// A quantity- or range-valued use context assigned to the citation
    ContextQuantity,
    /// A type of use context assigned to the citation
    ContextType,
    /// The citation publication date
    Date,
    /// The description of the citation
    Description,
    /// The time during which the citation is intended to be in use
    Effective,
    /// External identifier for the citation
    Identifier,
    /// Intended jurisdiction for the citation
    Jurisdiction,
    /// Computationally friendly name of the citation
    Name,
    /// Name of the publisher of the citation
    Publisher,
    /// The current status of the citation
    Status,
    /// The human-friendly name of the citation
    Title,
    /// The uri that identifies the citation
    Url,
    /// The business version of the citation
    Version,
    /// A use context type and quantity- or range-based value assigned to the citation
    ContextTypeQuantity,
    /// A use context type and value assigned to the citation
    ContextTypeValue,
    /// A type of classification assigned to the citation
    ClassificationType,
    /// A classifier assigned to the citation
    Classifier,
    /// A classification type and value assigned to the citation
    Classification,
}
impl ResourceSearchParameterDefinition for CitationSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Citation"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Context => "context",
            Self::ContextQuantity => "context-quantity",
            Self::ContextType => "context-type",
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
            Self::ContextTypeQuantity => "context-type-quantity",
            Self::ContextTypeValue => "context-type-value",
            Self::ClassificationType => "classification-type",
            Self::Classifier => "classifier",
            Self::Classification => "classification",
        }
    }
}
/// Search parameters for the ClinicalUseDefinition resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ClinicalUseDefinitionSearchParameter {
    /// The situation that is being documented as contraindicating against this item, as a code
    Contraindication,
    /// The situation that is being documented as contraindicating against this item, as a reference
    ContraindicationReference,
    /// The situation in which the undesirable effect may manifest, as a code
    Effect,
    /// The situation in which the undesirable effect may manifest, as a reference
    EffectReference,
    /// Business identifier for this issue
    Identifier,
    /// The situation that is being documented as an indicaton for this item, as a code
    Indication,
    /// The situation that is being documented as an indicaton for this item, as a reference
    IndicationReference,
    /// The type of the interaction e.g. drug-drug interaction, drug-food interaction, drug-lab test interaction
    Interaction,
    /// The medicinal product for which this is a clinical usage issue
    Product,
    /// Whether this is a current issue or one that has been retired etc.
    Status,
    /// The resource for which this is a clinical usage issue
    Subject,
    /// indication | contraindication | interaction | undesirable-effect | warning
    Type,
}
impl ResourceSearchParameterDefinition for ClinicalUseDefinitionSearchParameter {
    fn resource_type(&self) -> &'static str {
        "ClinicalUseDefinition"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Contraindication => "contraindication",
            Self::ContraindicationReference => "contraindication-reference",
            Self::Effect => "effect",
            Self::EffectReference => "effect-reference",
            Self::Identifier => "identifier",
            Self::Indication => "indication",
            Self::IndicationReference => "indication-reference",
            Self::Interaction => "interaction",
            Self::Product => "product",
            Self::Status => "status",
            Self::Subject => "subject",
            Self::Type => "type",
        }
    }
}
/// Search parameters for the CodeSystem resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CodeSystemSearchParameter {
    /// A use context assigned to the code system
    Context,
    /// A quantity- or range-valued use context assigned to the code system
    ContextQuantity,
    /// A type of use context assigned to the code system
    ContextType,
    /// The code system publication date
    Date,
    /// A resource that the CodeSystem is derived from
    DerivedFrom,
    /// The description of the code system
    Description,
    /// The time during which the CodeSystem is intended to be in use
    Effective,
    /// External identifier for the code system
    Identifier,
    /// Intended jurisdiction for the code system
    Jurisdiction,
    /// Computationally friendly name of the code system
    Name,
    /// The predecessor of the CodeSystem
    Predecessor,
    /// Name of the publisher of the code system
    Publisher,
    /// The current status of the code system
    Status,
    /// The human-friendly name of the code system
    Title,
    /// Topics associated with the CodeSystem
    Topic,
    /// The uri that identifies the code system
    Url,
    /// The business version of the code system
    Version,
    /// A use context type and quantity- or range-based value assigned to the code system
    ContextTypeQuantity,
    /// A use context type and value assigned to the code system
    ContextTypeValue,
    /// A code defined in the code system
    Code,
    /// not-present | example | fragment | complete | supplement
    ContentMode,
    /// A language in which a designation is provided
    Language,
    /// Find code system supplements for the referenced code system
    Supplements,
    /// The system for any codes defined by this code system (same as 'url')
    System,
}
impl ResourceSearchParameterDefinition for CodeSystemSearchParameter {
    fn resource_type(&self) -> &'static str {
        "CodeSystem"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Context => "context",
            Self::ContextQuantity => "context-quantity",
            Self::ContextType => "context-type",
            Self::Date => "date",
            Self::DerivedFrom => "derived-from",
            Self::Description => "description",
            Self::Effective => "effective",
            Self::Identifier => "identifier",
            Self::Jurisdiction => "jurisdiction",
            Self::Name => "name",
            Self::Predecessor => "predecessor",
            Self::Publisher => "publisher",
            Self::Status => "status",
            Self::Title => "title",
            Self::Topic => "topic",
            Self::Url => "url",
            Self::Version => "version",
            Self::ContextTypeQuantity => "context-type-quantity",
            Self::ContextTypeValue => "context-type-value",
            Self::Code => "code",
            Self::ContentMode => "content-mode",
            Self::Language => "language",
            Self::Supplements => "supplements",
            Self::System => "system",
        }
    }
}
/// Search parameters for the CompartmentDefinition resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CompartmentDefinitionSearchParameter {
    /// A use context assigned to the compartment definition
    Context,
    /// A quantity- or range-valued use context assigned to the compartment definition
    ContextQuantity,
    /// A type of use context assigned to the compartment definition
    ContextType,
    /// The compartment definition publication date
    Date,
    /// The description of the compartment definition
    Description,
    /// Computationally friendly name of the compartment definition
    Name,
    /// Name of the publisher of the compartment definition
    Publisher,
    /// The current status of the compartment definition
    Status,
    /// The uri that identifies the compartment definition
    Url,
    /// The business version of the compartment definition
    Version,
    /// A use context type and quantity- or range-based value assigned to the compartment definition
    ContextTypeQuantity,
    /// A use context type and value assigned to the compartment definition
    ContextTypeValue,
    /// Patient | Encounter | RelatedPerson | Practitioner | Device
    Code,
    /// Name of resource type
    Resource,
}
impl ResourceSearchParameterDefinition for CompartmentDefinitionSearchParameter {
    fn resource_type(&self) -> &'static str {
        "CompartmentDefinition"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Context => "context",
            Self::ContextQuantity => "context-quantity",
            Self::ContextType => "context-type",
            Self::Date => "date",
            Self::Description => "description",
            Self::Name => "name",
            Self::Publisher => "publisher",
            Self::Status => "status",
            Self::Url => "url",
            Self::Version => "version",
            Self::ContextTypeQuantity => "context-type-quantity",
            Self::ContextTypeValue => "context-type-value",
            Self::Code => "code",
            Self::Resource => "resource",
        }
    }
}
/// Search parameters for the Composition resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CompositionSearchParameter {
    /// Version-independent identifier for the Composition
    Identifier,
    /// Who and/or what the composition is about
    Patient,
    /// Kind of composition (LOINC if possible)
    Type,
    /// Composition editing time
    Date,
    /// Context of the Composition
    Encounter,
    /// Who attested the composition
    Attester,
    /// Who and/or what authored the composition
    Author,
    /// Categorization of Composition
    Category,
    /// A reference to data that supports this section
    Entry,
    /// Main clinical acts documented as codes
    EventCode,
    /// Main clinical acts documented as references
    EventReference,
    /// The period covered by the documentation
    Period,
    /// Target of the relationship
    Related,
    /// Classification of section (recommended)
    Section,
    /// preliminary | final | amended | entered-in-error
    Status,
    /// Who and/or what the composition is about
    Subject,
    /// Human Readable name/title
    Title,
    /// The uri that identifies the activity definition
    Url,
    /// The business version of the activity definition
    Version,
    /// Search on the section narrative of the resource
    SectionCodeText,
}
impl ResourceSearchParameterDefinition for CompositionSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Composition"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Type => "type",
            Self::Date => "date",
            Self::Encounter => "encounter",
            Self::Attester => "attester",
            Self::Author => "author",
            Self::Category => "category",
            Self::Entry => "entry",
            Self::EventCode => "event-code",
            Self::EventReference => "event-reference",
            Self::Period => "period",
            Self::Related => "related",
            Self::Section => "section",
            Self::Status => "status",
            Self::Subject => "subject",
            Self::Title => "title",
            Self::Url => "url",
            Self::Version => "version",
            Self::SectionCodeText => "section-code-text",
        }
    }
}
/// Search parameters for the ConceptMap resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ConceptMapSearchParameter {
    /// A use context assigned to the concept map
    Context,
    /// A quantity- or range-valued use context assigned to the concept map
    ContextQuantity,
    /// A type of use context assigned to the concept map
    ContextType,
    /// The concept map publication date
    Date,
    /// A resource that the ConceptMap is derived from
    DerivedFrom,
    /// The description of the concept map
    Description,
    /// The time during which the ConceptMap is intended to be in use
    Effective,
    /// External identifier for the concept map
    Identifier,
    /// Intended jurisdiction for the concept map
    Jurisdiction,
    /// Computationally friendly name of the concept map
    Name,
    /// The predecessor of the ConceptMap
    Predecessor,
    /// Name of the publisher of the concept map
    Publisher,
    /// The current status of the concept map
    Status,
    /// The human-friendly name of the concept map
    Title,
    /// Topics associated with the ConceptMap
    Topic,
    /// The URI that identifies the concept map
    Url,
    /// The business version of the concept map
    Version,
    /// A use context type and quantity- or range-based value assigned to the concept map
    ContextTypeQuantity,
    /// A use context type and value assigned to the concept map
    ContextTypeValue,
    /// Other properties required for this mapping
    MappingProperty,
    /// canonical reference to an additional ConceptMap to use for mapping if the source concept is unmapped
    OtherMap,
    /// Identifies elements being mapped
    SourceCode,
    /// Source system where concepts to be mapped are defined
    SourceGroupSystem,
    /// The source value set that contains the concepts that are being mapped
    SourceScope,
    /// The URI for the source value set that contains the concepts being mapped
    SourceScopeUri,
    /// Code that identifies the target element
    TargetCode,
    /// Target system that the concepts are to be mapped to
    TargetGroupSystem,
    /// The target value set which provides context for the mappings
    TargetScope,
    /// The URI for the target value set that contains the concepts being mapped.
    TargetScopeUri,
}
impl ResourceSearchParameterDefinition for ConceptMapSearchParameter {
    fn resource_type(&self) -> &'static str {
        "ConceptMap"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Context => "context",
            Self::ContextQuantity => "context-quantity",
            Self::ContextType => "context-type",
            Self::Date => "date",
            Self::DerivedFrom => "derived-from",
            Self::Description => "description",
            Self::Effective => "effective",
            Self::Identifier => "identifier",
            Self::Jurisdiction => "jurisdiction",
            Self::Name => "name",
            Self::Predecessor => "predecessor",
            Self::Publisher => "publisher",
            Self::Status => "status",
            Self::Title => "title",
            Self::Topic => "topic",
            Self::Url => "url",
            Self::Version => "version",
            Self::ContextTypeQuantity => "context-type-quantity",
            Self::ContextTypeValue => "context-type-value",
            Self::MappingProperty => "mapping-property",
            Self::OtherMap => "other-map",
            Self::SourceCode => "source-code",
            Self::SourceGroupSystem => "source-group-system",
            Self::SourceScope => "source-scope",
            Self::SourceScopeUri => "source-scope-uri",
            Self::TargetCode => "target-code",
            Self::TargetGroupSystem => "target-group-system",
            Self::TargetScope => "target-scope",
            Self::TargetScopeUri => "target-scope-uri",
        }
    }
}
/// Search parameters for the Condition resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ConditionSearchParameter {
    /// A unique identifier of the condition record
    Identifier,
    /// Who has the condition?
    Patient,
    /// Code for the condition
    Code,
    /// The Encounter during which this Condition was created
    Encounter,
    /// Abatement as age or age range
    AbatementAge,
    /// Date-related abatements (dateTime and period)
    AbatementDate,
    /// Abatement as a string
    AbatementString,
    /// Anatomical location, if relevant
    BodySite,
    /// The category of the condition
    Category,
    /// The clinical status of the condition
    ClinicalStatus,
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
    /// Who or what participated in the activities related to the condition
    ParticipantActor,
    /// Type of involvement of the actor in the activities related to the condition
    ParticipantFunction,
    /// Date record was first recorded
    RecordedDate,
    /// The severity of the condition
    Severity,
    /// Simple summary (disease specific)
    Stage,
    /// Who has the condition?
    Subject,
    /// unconfirmed | provisional | differential | confirmed | refuted | entered-in-error
    VerificationStatus,
}
impl ResourceSearchParameterDefinition for ConditionSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Condition"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Code => "code",
            Self::Encounter => "encounter",
            Self::AbatementAge => "abatement-age",
            Self::AbatementDate => "abatement-date",
            Self::AbatementString => "abatement-string",
            Self::BodySite => "body-site",
            Self::Category => "category",
            Self::ClinicalStatus => "clinical-status",
            Self::Evidence => "evidence",
            Self::EvidenceDetail => "evidence-detail",
            Self::OnsetAge => "onset-age",
            Self::OnsetDate => "onset-date",
            Self::OnsetInfo => "onset-info",
            Self::ParticipantActor => "participant-actor",
            Self::ParticipantFunction => "participant-function",
            Self::RecordedDate => "recorded-date",
            Self::Severity => "severity",
            Self::Stage => "stage",
            Self::Subject => "subject",
            Self::VerificationStatus => "verification-status",
        }
    }
}
/// Search parameters for the ConditionDefinition resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ConditionDefinitionSearchParameter {
    /// A use context assigned to the condition definition
    Context,
    /// A quantity- or range-valued use context assigned to the condition definition
    ContextQuantity,
    /// A type of use context assigned to the condition definition
    ContextType,
    /// The condition definition publication date
    Date,
    /// The description of the condition definition
    Description,
    /// External identifier for the condition definition
    Identifier,
    /// Intended jurisdiction for the condition definition
    Jurisdiction,
    /// Computationally friendly name of the condition definition
    Name,
    /// Name of the publisher of the condition definition
    Publisher,
    /// The current status of the condition definition
    Status,
    /// The human-friendly name of the condition definition
    Title,
    /// The uri that identifies the condition definition
    Url,
    /// The business version of the condition definition
    Version,
    /// A use context type and quantity- or range-based value assigned to the condition definition
    ContextTypeQuantity,
    /// A use context type and value assigned to the condition definition
    ContextTypeValue,
}
impl ResourceSearchParameterDefinition for ConditionDefinitionSearchParameter {
    fn resource_type(&self) -> &'static str {
        "ConditionDefinition"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Context => "context",
            Self::ContextQuantity => "context-quantity",
            Self::ContextType => "context-type",
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
            Self::ContextTypeQuantity => "context-type-quantity",
            Self::ContextTypeValue => "context-type-value",
        }
    }
}
/// Search parameters for the Consent resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ConsentSearchParameter {
    /// Identifier for this record (external references)
    Identifier,
    /// Who the consent applies to
    Patient,
    /// When consent was agreed to
    Date,
    /// Actions controlled by this rule
    Action,
    /// Resource for the actor (or group, by role)
    Actor,
    /// Classification of the consent statement - for indexing/retrieval
    Category,
    /// Consent Enforcer
    Controller,
    /// The actual data reference
    Data,
    /// Who is agreeing to the policy and rules
    Grantee,
    /// Consent workflow management
    Manager,
    /// Timeframe for this rule
    Period,
    /// Context of activities covered by this rule
    Purpose,
    /// Security Labels that define affected resources
    SecurityLabel,
    /// Search by reference to a Consent, DocumentReference, Contract  or QuestionnaireResponse
    SourceReference,
    /// draft | active | inactive | entered-in-error | unknown
    Status,
    /// Who the consent applies to
    Subject,
    /// Has been verified
    Verified,
    /// When consent verified
    VerifiedDate,
}
impl ResourceSearchParameterDefinition for ConsentSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Consent"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Date => "date",
            Self::Action => "action",
            Self::Actor => "actor",
            Self::Category => "category",
            Self::Controller => "controller",
            Self::Data => "data",
            Self::Grantee => "grantee",
            Self::Manager => "manager",
            Self::Period => "period",
            Self::Purpose => "purpose",
            Self::SecurityLabel => "security-label",
            Self::SourceReference => "source-reference",
            Self::Status => "status",
            Self::Subject => "subject",
            Self::Verified => "verified",
            Self::VerifiedDate => "verified-date",
        }
    }
}
/// Search parameters for the Contract resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ContractSearchParameter {
    /// The identity of the contract
    Identifier,
    /// The identity of the subject of the contract (if a patient)
    Patient,
    /// The authority of the contract
    Authority,
    /// The domain of the contract
    Domain,
    /// A source definition of the contract
    Instantiates,
    /// The date/time the contract was issued
    Issued,
    /// Contract Signatory Party
    Signer,
    /// The status of the contract
    Status,
    /// The identity of the subject of the contract
    Subject,
    /// The basal contract definition
    Url,
}
impl ResourceSearchParameterDefinition for ContractSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Contract"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Authority => "authority",
            Self::Domain => "domain",
            Self::Instantiates => "instantiates",
            Self::Issued => "issued",
            Self::Signer => "signer",
            Self::Status => "status",
            Self::Subject => "subject",
            Self::Url => "url",
        }
    }
}
/// Search parameters for the Coverage resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CoverageSearchParameter {
    /// The primary identifier of the insured and the coverage
    Identifier,
    /// Retrieve coverages for a patient
    Patient,
    /// The kind of coverage (health plan, auto, Workers Compensation)
    Type,
    /// Covered party
    Beneficiary,
    /// Coverage class (e.g. plan, group)
    ClassType,
    /// Value of the class (e.g. Plan number, group number)
    ClassValue,
    /// Dependent number
    Dependent,
    /// The identity of the insurer
    Insurer,
    /// Parties who will pay for services
    PaymentbyParty,
    /// Reference to the policyholder
    PolicyHolder,
    /// The status of the Coverage
    Status,
    /// Reference to the subscriber
    Subscriber,
    /// Identifier of the subscriber
    Subscriberid,
}
impl ResourceSearchParameterDefinition for CoverageSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Coverage"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Type => "type",
            Self::Beneficiary => "beneficiary",
            Self::ClassType => "class-type",
            Self::ClassValue => "class-value",
            Self::Dependent => "dependent",
            Self::Insurer => "insurer",
            Self::PaymentbyParty => "paymentby-party",
            Self::PolicyHolder => "policy-holder",
            Self::Status => "status",
            Self::Subscriber => "subscriber",
            Self::Subscriberid => "subscriberid",
        }
    }
}
/// Search parameters for the CoverageEligibilityRequest resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CoverageEligibilityRequestSearchParameter {
    /// The business identifier of the Eligibility
    Identifier,
    /// The reference to the patient
    Patient,
    /// The creation date for the EOB
    Created,
    /// The party who is responsible for the request
    Enterer,
    /// Facility responsible for the goods and services
    Facility,
    /// The reference to the provider
    Provider,
    /// The status of the EligibilityRequest
    Status,
}
impl ResourceSearchParameterDefinition for CoverageEligibilityRequestSearchParameter {
    fn resource_type(&self) -> &'static str {
        "CoverageEligibilityRequest"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Created => "created",
            Self::Enterer => "enterer",
            Self::Facility => "facility",
            Self::Provider => "provider",
            Self::Status => "status",
        }
    }
}
/// Search parameters for the CoverageEligibilityResponse resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CoverageEligibilityResponseSearchParameter {
    /// The business identifier
    Identifier,
    /// The reference to the patient
    Patient,
    /// The creation date
    Created,
    /// The contents of the disposition message
    Disposition,
    /// The organization which generated this resource
    Insurer,
    /// The processing outcome
    Outcome,
    /// The EligibilityRequest reference
    Request,
    /// The EligibilityRequest provider
    Requestor,
    /// The EligibilityRequest status
    Status,
}
impl ResourceSearchParameterDefinition for CoverageEligibilityResponseSearchParameter {
    fn resource_type(&self) -> &'static str {
        "CoverageEligibilityResponse"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Created => "created",
            Self::Disposition => "disposition",
            Self::Insurer => "insurer",
            Self::Outcome => "outcome",
            Self::Request => "request",
            Self::Requestor => "requestor",
            Self::Status => "status",
        }
    }
}
/// Search parameters for the DetectedIssue resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DetectedIssueSearchParameter {
    /// Unique id for the detected issue
    Identifier,
    /// Associated patient
    Patient,
    /// Issue Type, e.g. drug-drug, duplicate therapy, etc.
    Code,
    /// The provider or device that identified the issue
    Author,
    /// Issue Category, e.g. drug-drug, duplicate therapy, etc.
    Category,
    /// When identified
    Identified,
    /// Problem resource
    Implicated,
    /// The status of the issue
    Status,
    /// Associated subject
    Subject,
}
impl ResourceSearchParameterDefinition for DetectedIssueSearchParameter {
    fn resource_type(&self) -> &'static str {
        "DetectedIssue"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Code => "code",
            Self::Author => "author",
            Self::Category => "category",
            Self::Identified => "identified",
            Self::Implicated => "implicated",
            Self::Status => "status",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the Device resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DeviceSearchParameter {
    /// The biological source for the device
    BiologicalSourceEvent,
    /// The definition / type of the device (code)
    Code,
    /// The definition / type of the device
    Definition,
    /// A server defined search that may match any of the string fields in Device.name or Device.type.
    DeviceName,
    /// The expiration date of the device
    ExpirationDate,
    /// Instance id from manufacturer, owner, and others
    Identifier,
    /// A location, where the resource is found
    Location,
    /// The lot number of the device
    LotNumber,
    /// The manufacture date of the device
    ManufactureDate,
    /// The manufacturer of the device
    Manufacturer,
    /// The model of the device
    Model,
    /// The organization responsible for the device
    Organization,
    /// The parent device
    Parent,
    /// The serial number of the device
    SerialNumber,
    /// The standards, specifications, or formal guidances.
    Specification,
    /// active | inactive | entered-in-error | unknown
    Status,
    /// The type of the device
    Type,
    /// UDI Barcode (RFID or other technology) string in *HRF* format.
    UdiCarrier,
    /// The udi Device Identifier (DI)
    UdiDi,
    /// Network address to contact device
    Url,
    /// The specific version of the device
    Version,
    /// Code and value parameter pair
    CodeValueConcept,
    /// A composite of both specification and version
    SpecificationVersion,
}
impl ResourceSearchParameterDefinition for DeviceSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Device"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::BiologicalSourceEvent => "biological-source-event",
            Self::Code => "code",
            Self::Definition => "definition",
            Self::DeviceName => "device-name",
            Self::ExpirationDate => "expiration-date",
            Self::Identifier => "identifier",
            Self::Location => "location",
            Self::LotNumber => "lot-number",
            Self::ManufactureDate => "manufacture-date",
            Self::Manufacturer => "manufacturer",
            Self::Model => "model",
            Self::Organization => "organization",
            Self::Parent => "parent",
            Self::SerialNumber => "serial-number",
            Self::Specification => "specification",
            Self::Status => "status",
            Self::Type => "type",
            Self::UdiCarrier => "udi-carrier",
            Self::UdiDi => "udi-di",
            Self::Url => "url",
            Self::Version => "version",
            Self::CodeValueConcept => "code-value-concept",
            Self::SpecificationVersion => "specification-version",
        }
    }
}
/// Search parameters for the DeviceAssociation resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DeviceAssociationSearchParameter {
    /// Search for products that match this code
    Device,
    /// The identifier of the device association
    Identifier,
    /// The identity of a operator for whom to list associations
    Operator,
    /// The identity of a patient for whom to list associations
    Patient,
    /// The status of the device associations
    Status,
    /// The identity of a patient for whom to list associations
    Subject,
}
impl ResourceSearchParameterDefinition for DeviceAssociationSearchParameter {
    fn resource_type(&self) -> &'static str {
        "DeviceAssociation"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Device => "device",
            Self::Identifier => "identifier",
            Self::Operator => "operator",
            Self::Patient => "patient",
            Self::Status => "status",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the DeviceDefinition resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DeviceDefinitionSearchParameter {
    /// A server defined search that may match any of the string fields in DeviceDefinition.name or DeviceDefinition.classification.type - the latter to search for 'generic' devices.
    DeviceName,
    /// The identifier of the component
    Identifier,
    /// The manufacturer of the device
    Manufacturer,
    /// The organization responsible for the device
    Organization,
    /// The specification that the device conforms to
    Specification,
    /// The device type
    Type,
    /// A composite of both specification and version
    SpecificationVersion,
}
impl ResourceSearchParameterDefinition for DeviceDefinitionSearchParameter {
    fn resource_type(&self) -> &'static str {
        "DeviceDefinition"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::DeviceName => "device-name",
            Self::Identifier => "identifier",
            Self::Manufacturer => "manufacturer",
            Self::Organization => "organization",
            Self::Specification => "specification",
            Self::Type => "type",
            Self::SpecificationVersion => "specification-version",
        }
    }
}
/// Search parameters for the DeviceDispense resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DeviceDispenseSearchParameter {
    /// Search for devices that match this code
    Code,
    /// The identifier of the dispense
    Identifier,
    /// Returns device dispenses for a specific patient
    Patient,
    /// The status of the dispense
    Status,
    /// The identity of a patient for whom to list dispenses
    Subject,
}
impl ResourceSearchParameterDefinition for DeviceDispenseSearchParameter {
    fn resource_type(&self) -> &'static str {
        "DeviceDispense"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Code => "code",
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Status => "status",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the DeviceMetric resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DeviceMetricSearchParameter {
    /// The category of the metric
    Category,
    /// The device resource
    Device,
    /// The identifier of the metric
    Identifier,
    /// The type of metric
    Type,
}
impl ResourceSearchParameterDefinition for DeviceMetricSearchParameter {
    fn resource_type(&self) -> &'static str {
        "DeviceMetric"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Category => "category",
            Self::Device => "device",
            Self::Identifier => "identifier",
            Self::Type => "type",
        }
    }
}
/// Search parameters for the DeviceRequest resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DeviceRequestSearchParameter {
    /// Business identifier for request/order
    Identifier,
    /// Individual the service is ordered for
    Patient,
    /// Code for what is being requested/ordered
    Code,
    /// Encounter during which request was created
    Encounter,
    /// When the request transitioned to being actionable
    AuthoredOn,
    /// Plan/proposal/order fulfilled by this request
    BasedOn,
    /// Reference to resource that is being requested/ordered
    Device,
    /// When service should occur
    EventDate,
    /// Composite request this is part of
    GroupIdentifier,
    /// Instantiates FHIR protocol or definition
    InstantiatesCanonical,
    /// Instantiates external protocol or definition
    InstantiatesUri,
    /// Associated insurance coverage
    Insurance,
    /// proposal | plan | original-order |reflex-order
    Intent,
    /// Desired performer for service
    Performer,
    /// Desired performer for service
    PerformerCode,
    /// Request takes the place of referenced completed or terminated requests
    PriorRequest,
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
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Code => "code",
            Self::Encounter => "encounter",
            Self::AuthoredOn => "authored-on",
            Self::BasedOn => "based-on",
            Self::Device => "device",
            Self::EventDate => "event-date",
            Self::GroupIdentifier => "group-identifier",
            Self::InstantiatesCanonical => "instantiates-canonical",
            Self::InstantiatesUri => "instantiates-uri",
            Self::Insurance => "insurance",
            Self::Intent => "intent",
            Self::Performer => "performer",
            Self::PerformerCode => "performer-code",
            Self::PriorRequest => "prior-request",
            Self::Requester => "requester",
            Self::Status => "status",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the DeviceUsage resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DeviceUsageSearchParameter {
    /// Search by identifier
    Identifier,
    /// Search by patient who used / uses the device
    Patient,
    /// Search by device
    Device,
    /// The status of the device usage
    Status,
}
impl ResourceSearchParameterDefinition for DeviceUsageSearchParameter {
    fn resource_type(&self) -> &'static str {
        "DeviceUsage"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Device => "device",
            Self::Status => "status",
        }
    }
}
/// Search parameters for the DiagnosticReport resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DiagnosticReportSearchParameter {
    /// An identifier for the report
    Identifier,
    /// The subject of the report if a patient
    Patient,
    /// The code for the report, as opposed to codes for the atomic results, which are the names on the observation resource referred to from the result
    Code,
    /// The clinically relevant time of the report
    Date,
    /// The Encounter when the order was made
    Encounter,
    /// Reference to the service request.
    BasedOn,
    /// Which diagnostic discipline/department created the report
    Category,
    /// A coded conclusion (interpretation/impression) on the report
    Conclusion,
    /// When the report was issued
    Issued,
    /// A reference to the image source.
    Media,
    /// Who is responsible for the report
    Performer,
    /// Link to an atomic result (observation resource)
    Result,
    /// Who was the source of the report
    ResultsInterpreter,
    /// The specimen details
    Specimen,
    /// The status of the report
    Status,
    /// Studies associated with the diagnostic report
    Study,
    /// The subject of the report
    Subject,
}
impl ResourceSearchParameterDefinition for DiagnosticReportSearchParameter {
    fn resource_type(&self) -> &'static str {
        "DiagnosticReport"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Code => "code",
            Self::Date => "date",
            Self::Encounter => "encounter",
            Self::BasedOn => "based-on",
            Self::Category => "category",
            Self::Conclusion => "conclusion",
            Self::Issued => "issued",
            Self::Media => "media",
            Self::Performer => "performer",
            Self::Result => "result",
            Self::ResultsInterpreter => "results-interpreter",
            Self::Specimen => "specimen",
            Self::Status => "status",
            Self::Study => "study",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the DocumentReference resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DocumentReferenceSearchParameter {
    /// Identifier of the attachment binary
    Identifier,
    /// Who/what is the subject of the document
    Patient,
    /// Kind of document (LOINC if possible)
    Type,
    /// When this document reference was created
    Date,
    /// Who attested the document
    Attester,
    /// Who and/or what authored the document
    Author,
    /// Procedure that caused this media to be created
    BasedOn,
    /// The body site studied
    Bodysite,
    /// The body site studied
    BodysiteReference,
    /// Categorization of document
    Category,
    /// Mime type of the content, with charset etc.
    Contenttype,
    /// Context of the document content
    Context,
    /// Date attachment was first created
    Creation,
    /// Organization which maintains the document
    Custodian,
    /// Human-readable description
    Description,
    /// preliminary | final | amended | entered-in-error
    DocStatus,
    /// Main clinical acts documented
    EventCode,
    /// Main clinical acts documented
    EventReference,
    /// Kind of facility where patient was seen
    Facility,
    /// Profile canonical content rules for the document
    FormatCanonical,
    /// Format code content rules for the document
    FormatCode,
    /// Profile URI content rules for the document
    FormatUri,
    /// Human language of the content (BCP-47)
    Language,
    /// Uri where the data can be found
    Location,
    /// The modality used
    Modality,
    /// Time of service that is being documented
    Period,
    /// Target of the relationship
    Relatesto,
    /// replaces | transforms | signs | appends
    Relation,
    /// Document security-tags
    SecurityLabel,
    /// Additional details about where the content was created (e.g. clinical specialty)
    Setting,
    /// current | superseded | entered-in-error
    Status,
    /// Who/what is the subject of the document
    Subject,
    /// The business version identifier
    Version,
    /// Combination of relation and relatesTo
    Relationship,
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
            Self::Date => "date",
            Self::Attester => "attester",
            Self::Author => "author",
            Self::BasedOn => "based-on",
            Self::Bodysite => "bodysite",
            Self::BodysiteReference => "bodysite-reference",
            Self::Category => "category",
            Self::Contenttype => "contenttype",
            Self::Context => "context",
            Self::Creation => "creation",
            Self::Custodian => "custodian",
            Self::Description => "description",
            Self::DocStatus => "doc-status",
            Self::EventCode => "event-code",
            Self::EventReference => "event-reference",
            Self::Facility => "facility",
            Self::FormatCanonical => "format-canonical",
            Self::FormatCode => "format-code",
            Self::FormatUri => "format-uri",
            Self::Language => "language",
            Self::Location => "location",
            Self::Modality => "modality",
            Self::Period => "period",
            Self::Relatesto => "relatesto",
            Self::Relation => "relation",
            Self::SecurityLabel => "security-label",
            Self::Setting => "setting",
            Self::Status => "status",
            Self::Subject => "subject",
            Self::Version => "version",
            Self::Relationship => "relationship",
        }
    }
}
/// Search parameters for the Encounter resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum EncounterSearchParameter {
    /// Identifier(s) by which this encounter is known
    Identifier,
    /// The patient present at the encounter
    Patient,
    /// Specific type of encounter
    Type,
    /// A date within the actualPeriod the Encounter lasted
    Date,
    /// The set of accounts that may be used for billing for this Encounter
    Account,
    /// The appointment that scheduled this encounter
    Appointment,
    /// The ServiceRequest that initiated this encounter
    BasedOn,
    /// Careteam allocated to participate in the encounter
    Careteam,
    /// Classification of patient encounter
    Class,
    /// The actual start date of the Encounter
    DateStart,
    /// The diagnosis or procedure relevant to the encounter (coded)
    DiagnosisCode,
    /// The diagnosis or procedure relevant to the encounter (resource reference)
    DiagnosisReference,
    /// The actual end date of the Encounter
    EndDate,
    /// Episode(s) of care that this encounter should be recorded against
    EpisodeOfCare,
    /// Length of encounter in days
    Length,
    /// Location the encounter takes place
    Location,
    /// Another Encounter this encounter is part of
    PartOf,
    /// Persons involved in the encounter other than the patient
    Participant,
    /// Role of participant in encounter
    ParticipantType,
    /// Persons involved in the encounter other than the patient
    Practitioner,
    /// Reference to a concept (coded)
    ReasonCode,
    /// Reference to a resource (resource reference)
    ReasonReference,
    /// The organization (facility) responsible for this encounter
    ServiceProvider,
    /// Wheelchair, translator, stretcher, etc.
    SpecialArrangement,
    /// planned | in-progress | on-hold | completed | cancelled | entered-in-error | unknown
    Status,
    /// The patient or group present at the encounter
    Subject,
    /// The current status of the subject in relation to the Encounter
    SubjectStatus,
    /// Time period during which the patient was present at the location
    LocationPeriod,
}
impl ResourceSearchParameterDefinition for EncounterSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Encounter"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Type => "type",
            Self::Date => "date",
            Self::Account => "account",
            Self::Appointment => "appointment",
            Self::BasedOn => "based-on",
            Self::Careteam => "careteam",
            Self::Class => "class",
            Self::DateStart => "date-start",
            Self::DiagnosisCode => "diagnosis-code",
            Self::DiagnosisReference => "diagnosis-reference",
            Self::EndDate => "end-date",
            Self::EpisodeOfCare => "episode-of-care",
            Self::Length => "length",
            Self::Location => "location",
            Self::PartOf => "part-of",
            Self::Participant => "participant",
            Self::ParticipantType => "participant-type",
            Self::Practitioner => "practitioner",
            Self::ReasonCode => "reason-code",
            Self::ReasonReference => "reason-reference",
            Self::ServiceProvider => "service-provider",
            Self::SpecialArrangement => "special-arrangement",
            Self::Status => "status",
            Self::Subject => "subject",
            Self::SubjectStatus => "subject-status",
            Self::LocationPeriod => "location-period",
        }
    }
}
/// Search parameters for the EncounterHistory resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum EncounterHistorySearchParameter {
    /// The Encounter associated with this set of history values
    Encounter,
    /// Identifier(s) by which this encounter is known
    Identifier,
    /// The patient present at the encounter
    Patient,
    /// Status of the Encounter history entry
    Status,
    /// The patient or group present at the encounter
    Subject,
}
impl ResourceSearchParameterDefinition for EncounterHistorySearchParameter {
    fn resource_type(&self) -> &'static str {
        "EncounterHistory"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Encounter => "encounter",
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Status => "status",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the EpisodeOfCare resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum EpisodeOfCareSearchParameter {
    /// Business Identifier(s) relevant for this EpisodeOfCare
    Identifier,
    /// The patient who is the focus of this episode of care
    Patient,
    /// Type/class  - e.g. specialist referral, disease management
    Type,
    /// The provided date search value falls within the episode of care's period
    Date,
    /// Care manager/care coordinator for the patient
    CareManager,
    /// Conditions/problems/diagnoses this episode of care is for (coded)
    DiagnosisCode,
    /// Conditions/problems/diagnoses this episode of care is for (resource reference)
    DiagnosisReference,
    /// Incoming Referral Request
    IncomingReferral,
    /// The organization that has assumed the specific responsibilities of this EpisodeOfCare
    Organization,
    /// Reference to a concept (coded)
    ReasonCode,
    /// Reference to a resource (resource reference)
    ReasonReference,
    /// The current status of the Episode of Care as provided (does not check the status history collection)
    Status,
}
impl ResourceSearchParameterDefinition for EpisodeOfCareSearchParameter {
    fn resource_type(&self) -> &'static str {
        "EpisodeOfCare"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Type => "type",
            Self::Date => "date",
            Self::CareManager => "care-manager",
            Self::DiagnosisCode => "diagnosis-code",
            Self::DiagnosisReference => "diagnosis-reference",
            Self::IncomingReferral => "incoming-referral",
            Self::Organization => "organization",
            Self::ReasonCode => "reason-code",
            Self::ReasonReference => "reason-reference",
            Self::Status => "status",
        }
    }
}
/// Search parameters for the EventDefinition resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum EventDefinitionSearchParameter {
    /// What resource is being referenced
    ComposedOf,
    /// A use context assigned to the event definition
    Context,
    /// A quantity- or range-valued use context assigned to the event definition
    ContextQuantity,
    /// A type of use context assigned to the event definition
    ContextType,
    /// The event definition publication date
    Date,
    /// What resource is being referenced
    DependsOn,
    /// What resource is being referenced
    DerivedFrom,
    /// The description of the event definition
    Description,
    /// The time during which the event definition is intended to be in use
    Effective,
    /// External identifier for the event definition
    Identifier,
    /// Intended jurisdiction for the event definition
    Jurisdiction,
    /// Computationally friendly name of the event definition
    Name,
    /// What resource is being referenced
    Predecessor,
    /// Name of the publisher of the event definition
    Publisher,
    /// The current status of the event definition
    Status,
    /// What resource is being referenced
    Successor,
    /// The human-friendly name of the event definition
    Title,
    /// Topics associated with the module
    Topic,
    /// The uri that identifies the event definition
    Url,
    /// The business version of the event definition
    Version,
    /// A use context type and quantity- or range-based value assigned to the event definition
    ContextTypeQuantity,
    /// A use context type and value assigned to the event definition
    ContextTypeValue,
}
impl ResourceSearchParameterDefinition for EventDefinitionSearchParameter {
    fn resource_type(&self) -> &'static str {
        "EventDefinition"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::ComposedOf => "composed-of",
            Self::Context => "context",
            Self::ContextQuantity => "context-quantity",
            Self::ContextType => "context-type",
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
            Self::ContextTypeQuantity => "context-type-quantity",
            Self::ContextTypeValue => "context-type-value",
        }
    }
}
/// Search parameters for the Evidence resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum EvidenceSearchParameter {
    /// A use context assigned to the evidence
    Context,
    /// A quantity- or range-valued use context assigned to the evidence
    ContextQuantity,
    /// A type of use context assigned to the evidence
    ContextType,
    /// The evidence publication date
    Date,
    /// The description of the evidence
    Description,
    /// External identifier for the evidence
    Identifier,
    /// Name of the publisher of the evidence
    Publisher,
    /// The current status of the evidence
    Status,
    /// The human-friendly name of the evidence
    Title,
    /// The uri that identifies the evidence
    Url,
    /// The business version of the evidence
    Version,
    /// A use context type and quantity- or range-based value assigned to the evidence
    ContextTypeQuantity,
    /// A use context type and value assigned to the evidence
    ContextTypeValue,
}
impl ResourceSearchParameterDefinition for EvidenceSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Evidence"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Context => "context",
            Self::ContextQuantity => "context-quantity",
            Self::ContextType => "context-type",
            Self::Date => "date",
            Self::Description => "description",
            Self::Identifier => "identifier",
            Self::Publisher => "publisher",
            Self::Status => "status",
            Self::Title => "title",
            Self::Url => "url",
            Self::Version => "version",
            Self::ContextTypeQuantity => "context-type-quantity",
            Self::ContextTypeValue => "context-type-value",
        }
    }
}
/// Search parameters for the EvidenceReport resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum EvidenceReportSearchParameter {
    /// A use context assigned to the evidence report
    Context,
    /// A quantity- or range-valued use context assigned to the evidence report
    ContextQuantity,
    /// A type of use context assigned to the evidence report
    ContextType,
    /// External identifier for the evidence report
    Identifier,
    /// Name of the publisher of the evidence report
    Publisher,
    /// The current status of the evidence report
    Status,
    /// The uri that identifies the evidence report
    Url,
    /// A use context type and quantity- or range-based value assigned to the evidence report
    ContextTypeQuantity,
    /// A use context type and value assigned to the evidence report
    ContextTypeValue,
}
impl ResourceSearchParameterDefinition for EvidenceReportSearchParameter {
    fn resource_type(&self) -> &'static str {
        "EvidenceReport"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Context => "context",
            Self::ContextQuantity => "context-quantity",
            Self::ContextType => "context-type",
            Self::Identifier => "identifier",
            Self::Publisher => "publisher",
            Self::Status => "status",
            Self::Url => "url",
            Self::ContextTypeQuantity => "context-type-quantity",
            Self::ContextTypeValue => "context-type-value",
        }
    }
}
/// Search parameters for the EvidenceVariable resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum EvidenceVariableSearchParameter {
    /// What resource is being referenced
    ComposedOf,
    /// A use context assigned to the evidence variable
    Context,
    /// A quantity- or range-valued use context assigned to the evidence variable
    ContextQuantity,
    /// A type of use context assigned to the evidence variable
    ContextType,
    /// The evidence variable publication date
    Date,
    /// What resource is being referenced
    DependsOn,
    /// What resource is being referenced
    DerivedFrom,
    /// The description of the evidence variable
    Description,
    /// External identifier for the evidence variable
    Identifier,
    /// Computationally friendly name of the evidence variable
    Name,
    /// What resource is being referenced
    Predecessor,
    /// Name of the publisher of the evidence variable
    Publisher,
    /// The current status of the evidence variable
    Status,
    /// What resource is being referenced
    Successor,
    /// The human-friendly name of the evidence variable
    Title,
    /// Topics associated with the EvidenceVariable
    Topic,
    /// The uri that identifies the evidence variable
    Url,
    /// The business version of the evidence variable
    Version,
    /// A use context type and quantity- or range-based value assigned to the evidence variable
    ContextTypeQuantity,
    /// A use context type and value assigned to the evidence variable
    ContextTypeValue,
}
impl ResourceSearchParameterDefinition for EvidenceVariableSearchParameter {
    fn resource_type(&self) -> &'static str {
        "EvidenceVariable"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::ComposedOf => "composed-of",
            Self::Context => "context",
            Self::ContextQuantity => "context-quantity",
            Self::ContextType => "context-type",
            Self::Date => "date",
            Self::DependsOn => "depends-on",
            Self::DerivedFrom => "derived-from",
            Self::Description => "description",
            Self::Identifier => "identifier",
            Self::Name => "name",
            Self::Predecessor => "predecessor",
            Self::Publisher => "publisher",
            Self::Status => "status",
            Self::Successor => "successor",
            Self::Title => "title",
            Self::Topic => "topic",
            Self::Url => "url",
            Self::Version => "version",
            Self::ContextTypeQuantity => "context-type-quantity",
            Self::ContextTypeValue => "context-type-value",
        }
    }
}
/// Search parameters for the ExampleScenario resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ExampleScenarioSearchParameter {
    /// A use context assigned to the example scenario
    Context,
    /// A quantity- or range-valued use context assigned to the example scenario
    ContextQuantity,
    /// A type of use context assigned to the example scenario
    ContextType,
    /// The example scenario publication date
    Date,
    /// External identifier for the example scenario
    Identifier,
    /// Intended jurisdiction for the example scenario
    Jurisdiction,
    /// Computationally friendly name of the example scenario
    Name,
    /// Name of the publisher of the example scenario
    Publisher,
    /// The current status of the example scenario
    Status,
    /// The uri that identifies the example scenario
    Url,
    /// The business version of the example scenario
    Version,
    /// A use context type and quantity- or range-based value assigned to the example scenario
    ContextTypeQuantity,
    /// A use context type and value assigned to the example scenario
    ContextTypeValue,
}
impl ResourceSearchParameterDefinition for ExampleScenarioSearchParameter {
    fn resource_type(&self) -> &'static str {
        "ExampleScenario"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Context => "context",
            Self::ContextQuantity => "context-quantity",
            Self::ContextType => "context-type",
            Self::Date => "date",
            Self::Identifier => "identifier",
            Self::Jurisdiction => "jurisdiction",
            Self::Name => "name",
            Self::Publisher => "publisher",
            Self::Status => "status",
            Self::Url => "url",
            Self::Version => "version",
            Self::ContextTypeQuantity => "context-type-quantity",
            Self::ContextTypeValue => "context-type-value",
        }
    }
}
/// Search parameters for the Flag resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FlagSearchParameter {
    /// Business identifier
    Identifier,
    /// The identity of a subject to list flags for
    Patient,
    /// Time period when flag is active
    Date,
    /// Alert relevant during encounter
    Encounter,
    /// Flag creator
    Author,
    /// The category of the flag, such as clinical, administrative, etc.
    Category,
    /// active | inactive | entered-in-error
    Status,
    /// The identity of a subject to list flags for
    Subject,
}
impl ResourceSearchParameterDefinition for FlagSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Flag"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Date => "date",
            Self::Encounter => "encounter",
            Self::Author => "author",
            Self::Category => "category",
            Self::Status => "status",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the FormularyItem resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FormularyItemSearchParameter {
    /// Returns formulary items for a specific code
    Code,
    /// Returns formulary items with this external identifier
    Identifier,
}
impl ResourceSearchParameterDefinition for FormularyItemSearchParameter {
    fn resource_type(&self) -> &'static str {
        "FormularyItem"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Code => "code",
            Self::Identifier => "identifier",
        }
    }
}
/// Search parameters for the GenomicStudy resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum GenomicStudySearchParameter {
    /// What the genomic study analysis is about, when it is not about the subject of record
    Focus,
    /// Identifiers for the Study
    Identifier,
    /// Who the study is about
    Patient,
    /// The status of the study
    Status,
    /// Who the study is about
    Subject,
}
impl ResourceSearchParameterDefinition for GenomicStudySearchParameter {
    fn resource_type(&self) -> &'static str {
        "GenomicStudy"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Focus => "focus",
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Status => "status",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the GraphDefinition resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum GraphDefinitionSearchParameter {
    /// A use context assigned to the graph definition
    Context,
    /// A quantity- or range-valued use context assigned to the graph definition
    ContextQuantity,
    /// A type of use context assigned to the graph definition
    ContextType,
    /// The graph definition publication date
    Date,
    /// The description of the graph definition
    Description,
    /// External identifier for the graph definition
    Identifier,
    /// Intended jurisdiction for the graph definition
    Jurisdiction,
    /// Computationally friendly name of the graph definition
    Name,
    /// Name of the publisher of the graph definition
    Publisher,
    /// The current status of the graph definition
    Status,
    /// The uri that identifies the graph definition
    Url,
    /// The business version of the graph definition
    Version,
    /// A use context type and quantity- or range-based value assigned to the graph definition
    ContextTypeQuantity,
    /// A use context type and value assigned to the graph definition
    ContextTypeValue,
    /// Type of resource at which the graph starts
    Start,
}
impl ResourceSearchParameterDefinition for GraphDefinitionSearchParameter {
    fn resource_type(&self) -> &'static str {
        "GraphDefinition"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Context => "context",
            Self::ContextQuantity => "context-quantity",
            Self::ContextType => "context-type",
            Self::Date => "date",
            Self::Description => "description",
            Self::Identifier => "identifier",
            Self::Jurisdiction => "jurisdiction",
            Self::Name => "name",
            Self::Publisher => "publisher",
            Self::Status => "status",
            Self::Url => "url",
            Self::Version => "version",
            Self::ContextTypeQuantity => "context-type-quantity",
            Self::ContextTypeValue => "context-type-value",
            Self::Start => "start",
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
    /// The status of the guidance response
    Status,
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
            Self::Status => "status",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the ImagingSelection resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ImagingSelectionSearchParameter {
    /// Identifiers for the imaging selection
    Identifier,
    /// Who the study is about
    Patient,
    /// The imaging selection status
    Code,
    /// The request associated with an imaging selection
    BasedOn,
    /// The body site code associated with the imaging selection
    BodySite,
    /// The body structure associated with the imaging selection
    BodyStructure,
    /// The imaging study from which the imaging selection was derived
    DerivedFrom,
    /// The date / time the imaging selection was created
    Issued,
    /// The status of the imaging selection
    Status,
    /// The DICOM Study Instance UID from which the images were selected
    StudyUid,
    /// The subject of the Imaging Selection, such as the associated Patient
    Subject,
}
impl ResourceSearchParameterDefinition for ImagingSelectionSearchParameter {
    fn resource_type(&self) -> &'static str {
        "ImagingSelection"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Code => "code",
            Self::BasedOn => "based-on",
            Self::BodySite => "body-site",
            Self::BodyStructure => "body-structure",
            Self::DerivedFrom => "derived-from",
            Self::Issued => "issued",
            Self::Status => "status",
            Self::StudyUid => "study-uid",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the Immunization resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ImmunizationSearchParameter {
    /// Business identifier
    Identifier,
    /// The patient for the vaccination record
    Patient,
    /// Vaccination  (non)-Administration Date
    Date,
    /// The service delivery location or facility in which the vaccine was / was to be administered
    Location,
    /// Vaccine Lot Number
    LotNumber,
    /// Vaccine Manufacturer
    Manufacturer,
    /// The practitioner, individual or organization who played a role in the vaccination
    Performer,
    /// Additional information on reaction
    Reaction,
    /// When reaction started
    ReactionDate,
    /// Reason why the vaccine was administered
    ReasonCode,
    /// Reference to a resource (by instance)
    ReasonReference,
    /// The series being followed by the provider
    Series,
    /// Immunization event status
    Status,
    /// Reason why the vaccine was not administered
    StatusReason,
    /// The target disease the dose is being administered against
    TargetDisease,
    /// Vaccine Product Administered
    VaccineCode,
}
impl ResourceSearchParameterDefinition for ImmunizationSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Immunization"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Date => "date",
            Self::Location => "location",
            Self::LotNumber => "lot-number",
            Self::Manufacturer => "manufacturer",
            Self::Performer => "performer",
            Self::Reaction => "reaction",
            Self::ReactionDate => "reaction-date",
            Self::ReasonCode => "reason-code",
            Self::ReasonReference => "reason-reference",
            Self::Series => "series",
            Self::Status => "status",
            Self::StatusReason => "status-reason",
            Self::TargetDisease => "target-disease",
            Self::VaccineCode => "vaccine-code",
        }
    }
}
/// Search parameters for the ImmunizationEvaluation resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ImmunizationEvaluationSearchParameter {
    /// ID of the evaluation
    Identifier,
    /// The patient being evaluated
    Patient,
    /// Date the evaluation was generated
    Date,
    /// The status of the dose relative to published recommendations
    DoseStatus,
    /// The vaccine administration event being evaluated
    ImmunizationEvent,
    /// Immunization evaluation status
    Status,
    /// The vaccine preventable disease being evaluated against
    TargetDisease,
}
impl ResourceSearchParameterDefinition for ImmunizationEvaluationSearchParameter {
    fn resource_type(&self) -> &'static str {
        "ImmunizationEvaluation"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Date => "date",
            Self::DoseStatus => "dose-status",
            Self::ImmunizationEvent => "immunization-event",
            Self::Status => "status",
            Self::TargetDisease => "target-disease",
        }
    }
}
/// Search parameters for the ImmunizationRecommendation resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ImmunizationRecommendationSearchParameter {
    /// Business identifier
    Identifier,
    /// Who this profile is for
    Patient,
    /// Date recommendation(s) created
    Date,
    /// Patient observations supporting recommendation
    Information,
    /// Vaccine recommendation status
    Status,
    /// Past immunizations supporting recommendation
    Support,
    /// Disease to be immunized against
    TargetDisease,
    /// Vaccine  or vaccine group recommendation applies to
    VaccineType,
}
impl ResourceSearchParameterDefinition for ImmunizationRecommendationSearchParameter {
    fn resource_type(&self) -> &'static str {
        "ImmunizationRecommendation"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Date => "date",
            Self::Information => "information",
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
    /// A use context assigned to the implementation guide
    Context,
    /// A quantity- or range-valued use context assigned to the implementation guide
    ContextQuantity,
    /// A type of use context assigned to the implementation guide
    ContextType,
    /// The implementation guide publication date
    Date,
    /// The description of the implementation guide
    Description,
    /// External identifier for the implementation guide
    Identifier,
    /// Intended jurisdiction for the implementation guide
    Jurisdiction,
    /// Computationally friendly name of the implementation guide
    Name,
    /// Name of the publisher of the implementation guide
    Publisher,
    /// The current status of the implementation guide
    Status,
    /// The human-friendly name of the implementation guide
    Title,
    /// The uri that identifies the implementation guide
    Url,
    /// The business version of the implementation guide
    Version,
    /// A use context type and quantity- or range-based value assigned to the implementation guide
    ContextTypeQuantity,
    /// A use context type and value assigned to the implementation guide
    ContextTypeValue,
    /// Identity of the IG that this depends on
    DependsOn,
    /// For testing purposes, not real usage
    Experimental,
    /// Profile that all resources must conform to
    Global,
    /// Location of the resource
    Resource,
}
impl ResourceSearchParameterDefinition for ImplementationGuideSearchParameter {
    fn resource_type(&self) -> &'static str {
        "ImplementationGuide"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Context => "context",
            Self::ContextQuantity => "context-quantity",
            Self::ContextType => "context-type",
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
            Self::ContextTypeQuantity => "context-type-quantity",
            Self::ContextTypeValue => "context-type-value",
            Self::DependsOn => "depends-on",
            Self::Experimental => "experimental",
            Self::Global => "global",
            Self::Resource => "resource",
        }
    }
}
/// Search parameters for the Ingredient resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum IngredientSearchParameter {
    /// The product which this ingredient is a constituent part of
    For,
    /// A classification of the ingredient identifying its precise purpose(s) in the drug product. This extends the Ingredient.role to add more detail. Example: Antioxidant, Alkalizing Agent
    Function,
    /// An identifier or code by which the ingredient can be referenced
    Identifier,
    /// The organization that manufactures this ingredient
    Manufacturer,
    /// A classification of the ingredient identifying its purpose within the product, e.g. active, inactive
    Role,
    /// The status of this ingredient. Enables tracking the life-cycle of the content
    Status,
    /// Ingredient concentration strength as quantity
    StrengthConcentrationQuantity,
    /// Ingredient presentation strength as quantity
    StrengthPresentationQuantity,
    /// Reference to a resource (by instance)
    Substance,
    /// Reference to a concept (by class)
    SubstanceCode,
    /// Reference to a resource (by instance)
    SubstanceDefinition,
    /// Ingredient concentration strength as ratio, numerator and denominator parameter pair
    StrengthConcentrationRatio,
    /// Ingredient presentation strength as ratio, numerator and denominator parameter pair
    StrengthPresentationRatio,
}
impl ResourceSearchParameterDefinition for IngredientSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Ingredient"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::For => "for",
            Self::Function => "function",
            Self::Identifier => "identifier",
            Self::Manufacturer => "manufacturer",
            Self::Role => "role",
            Self::Status => "status",
            Self::StrengthConcentrationQuantity => "strength-concentration-quantity",
            Self::StrengthPresentationQuantity => "strength-presentation-quantity",
            Self::Substance => "substance",
            Self::SubstanceCode => "substance-code",
            Self::SubstanceDefinition => "substance-definition",
            Self::StrengthConcentrationRatio => "strength-concentration-ratio",
            Self::StrengthPresentationRatio => "strength-presentation-ratio",
        }
    }
}
/// Search parameters for the InsurancePlan resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum InsurancePlanSearchParameter {
    /// A server defined search that may match any of the string fields in the Address, including line, city, district, state, country, postalCode, and/or text
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
    /// Product administrator
    AdministeredBy,
    /// Technical endpoint
    Endpoint,
    /// Any identifier for the organization (not the accreditation issuer's identifier)
    Identifier,
    /// A portion of the organization's name or alias
    Name,
    /// An organization of which this organization forms a part
    OwnedBy,
    /// A portion of the organization's name using some kind of phonetic matching algorithm
    Phonetic,
    /// Is the Organization record active
    Status,
    /// A code for the type of organization
    Type,
}
impl ResourceSearchParameterDefinition for InsurancePlanSearchParameter {
    fn resource_type(&self) -> &'static str {
        "InsurancePlan"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Address => "address",
            Self::AddressCity => "address-city",
            Self::AddressCountry => "address-country",
            Self::AddressPostalcode => "address-postalcode",
            Self::AddressState => "address-state",
            Self::AddressUse => "address-use",
            Self::AdministeredBy => "administered-by",
            Self::Endpoint => "endpoint",
            Self::Identifier => "identifier",
            Self::Name => "name",
            Self::OwnedBy => "owned-by",
            Self::Phonetic => "phonetic",
            Self::Status => "status",
            Self::Type => "type",
        }
    }
}
/// Search parameters for the InventoryItem resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum InventoryItemSearchParameter {
    /// Search for products that match this code
    Code,
    /// The identifier of the item
    Identifier,
    /// The status of the item
    Status,
    /// The identity of a patient for whom to list associations
    Subject,
}
impl ResourceSearchParameterDefinition for InventoryItemSearchParameter {
    fn resource_type(&self) -> &'static str {
        "InventoryItem"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Code => "code",
            Self::Identifier => "identifier",
            Self::Status => "status",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the InventoryReport resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum InventoryReportSearchParameter {
    /// Search by identifier
    Identifier,
    /// Search by items in inventory report
    Item,
    /// Search by items in inventory report
    ItemReference,
    /// Search by status
    Status,
}
impl ResourceSearchParameterDefinition for InventoryReportSearchParameter {
    fn resource_type(&self) -> &'static str {
        "InventoryReport"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Item => "item",
            Self::ItemReference => "item-reference",
            Self::Status => "status",
        }
    }
}
/// Search parameters for the Invoice resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum InvoiceSearchParameter {
    /// Business Identifier for item
    Identifier,
    /// Recipient(s) of goods and services
    Patient,
    /// Type of Invoice
    Type,
    /// Invoice date / posting date
    Date,
    /// Account that is being balanced
    Account,
    /// Issuing Organization of Invoice
    Issuer,
    /// Individual who was involved
    Participant,
    /// Type of involvement in creation of this Invoice
    ParticipantRole,
    /// Recipient of this invoice
    Recipient,
    /// draft | issued | balanced | cancelled | entered-in-error
    Status,
    /// Recipient(s) of goods and services
    Subject,
    /// Gross total of this Invoice
    Totalgross,
    /// Net total of this Invoice
    Totalnet,
}
impl ResourceSearchParameterDefinition for InvoiceSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Invoice"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Type => "type",
            Self::Date => "date",
            Self::Account => "account",
            Self::Issuer => "issuer",
            Self::Participant => "participant",
            Self::ParticipantRole => "participant-role",
            Self::Recipient => "recipient",
            Self::Status => "status",
            Self::Subject => "subject",
            Self::Totalgross => "totalgross",
            Self::Totalnet => "totalnet",
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
    /// One of the Location's characteristics
    Characteristic,
    /// Technical endpoints providing access to services operated for the location
    Endpoint,
    /// An identifier for the location
    Identifier,
    /// A portion of the location's name or alias
    Name,
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
            Self::Characteristic => "characteristic",
            Self::Endpoint => "endpoint",
            Self::Identifier => "identifier",
            Self::Name => "name",
            Self::OperationalStatus => "operational-status",
            Self::Organization => "organization",
            Self::Partof => "partof",
            Self::Status => "status",
            Self::Type => "type",
        }
    }
}
/// Search parameters for the ManufacturedItemDefinition resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ManufacturedItemDefinitionSearchParameter {
    /// Dose form as manufactured and before any transformation into the pharmaceutical product
    DoseForm,
    /// Unique identifier
    Identifier,
    /// An ingredient of this item
    Ingredient,
    /// A descriptive name applied to this item
    Name,
    /// The status of this item. Enables tracking the life-cycle of the content.
    Status,
}
impl ResourceSearchParameterDefinition for ManufacturedItemDefinitionSearchParameter {
    fn resource_type(&self) -> &'static str {
        "ManufacturedItemDefinition"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::DoseForm => "dose-form",
            Self::Identifier => "identifier",
            Self::Ingredient => "ingredient",
            Self::Name => "name",
            Self::Status => "status",
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
    /// The date of the measure report
    Date,
    /// An evaluated resource referenced by the measure report
    EvaluatedResource,
    /// The location to return measure report results for
    Location,
    /// The measure to return measure report results for
    Measure,
    /// The period of the measure report
    Period,
    /// The reporter to return measure report results for
    Reporter,
    /// The status of the measure report
    Status,
    /// The identity of a subject to search for individual measure report results for
    Subject,
}
impl ResourceSearchParameterDefinition for MeasureReportSearchParameter {
    fn resource_type(&self) -> &'static str {
        "MeasureReport"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Date => "date",
            Self::EvaluatedResource => "evaluated-resource",
            Self::Location => "location",
            Self::Measure => "measure",
            Self::Period => "period",
            Self::Reporter => "reporter",
            Self::Status => "status",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the Medication resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MedicationSearchParameter {
    /// Returns medications with this external identifier
    Identifier,
    /// Returns medications for a specific code
    Code,
    /// Returns medications in a batch with this expiration date
    ExpirationDate,
    /// Returns medications for this ingredient reference
    Ingredient,
    /// Returns medications for this ingredient code
    IngredientCode,
    /// Returns medications in a batch with this lot number
    LotNumber,
    /// Returns medications made or sold for this marketing authorization holder
    Marketingauthorizationholder,
    /// Returns medications in a batch with this lot number
    SerialNumber,
    /// Returns medications for this status
    Status,
}
impl ResourceSearchParameterDefinition for MedicationSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Medication"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Code => "code",
            Self::ExpirationDate => "expiration-date",
            Self::Ingredient => "ingredient",
            Self::IngredientCode => "ingredient-code",
            Self::LotNumber => "lot-number",
            Self::Marketingauthorizationholder => "marketingauthorizationholder",
            Self::SerialNumber => "serial-number",
            Self::Status => "status",
        }
    }
}
/// Search parameters for the MedicationAdministration resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MedicationAdministrationSearchParameter {
    /// Return administrations with this external identifier
    Identifier,
    /// The identity of a patient to list administrations  for
    Patient,
    /// Return administrations of this medication code
    Code,
    /// Date administration happened (or did not happen)
    Date,
    /// Return administrations with this administration device identity
    Device,
    /// Return administrations that share this encounter
    Encounter,
    /// Return administrations of this medication reference
    Medication,
    /// The identity of the individual who administered the medication
    Performer,
    /// The identity of the individual who administered the medication
    PerformerDeviceCode,
    /// Reference to a resource (by instance)
    ReasonGiven,
    /// Reasons for administering the medication
    ReasonGivenCode,
    /// Reasons for not administering the medication
    ReasonNotGiven,
    /// The identity of a request to list administrations from
    Request,
    /// MedicationAdministration event status (for example one of active/paused/completed/nullified)
    Status,
    /// The identity of the individual or group to list administrations for
    Subject,
}
impl ResourceSearchParameterDefinition for MedicationAdministrationSearchParameter {
    fn resource_type(&self) -> &'static str {
        "MedicationAdministration"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Code => "code",
            Self::Date => "date",
            Self::Device => "device",
            Self::Encounter => "encounter",
            Self::Medication => "medication",
            Self::Performer => "performer",
            Self::PerformerDeviceCode => "performer-device-code",
            Self::ReasonGiven => "reason-given",
            Self::ReasonGivenCode => "reason-given-code",
            Self::ReasonNotGiven => "reason-not-given",
            Self::Request => "request",
            Self::Status => "status",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the MedicationDispense resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MedicationDispenseSearchParameter {
    /// Returns dispenses with this external identifier
    Identifier,
    /// The identity of a patient to list dispenses  for
    Patient,
    /// Returns dispenses of a specific type
    Type,
    /// Returns dispenses of this medicine code
    Code,
    /// Returns dispenses with a specific encounter
    Encounter,
    /// Returns dispenses of this medicine resource
    Medication,
    /// Returns dispenses with a specified dispense status
    Status,
    /// Returns dispenses that should be sent to a specific destination
    Destination,
    /// Returns dispense for a given location
    Location,
    /// Returns dispenses performed by a specific individual
    Performer,
    /// The identity of a prescription to list dispenses from
    Prescription,
    /// The identity of a receiver to list dispenses for
    Receiver,
    /// Returns dispenses where dispensing activity began on this date
    Recorded,
    /// Returns dispenses with the specified responsible party
    Responsibleparty,
    /// The identity of a patient for whom to list dispenses
    Subject,
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
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Type => "type",
            Self::Code => "code",
            Self::Encounter => "encounter",
            Self::Medication => "medication",
            Self::Status => "status",
            Self::Destination => "destination",
            Self::Location => "location",
            Self::Performer => "performer",
            Self::Prescription => "prescription",
            Self::Receiver => "receiver",
            Self::Recorded => "recorded",
            Self::Responsibleparty => "responsibleparty",
            Self::Subject => "subject",
            Self::Whenhandedover => "whenhandedover",
            Self::Whenprepared => "whenprepared",
        }
    }
}
/// Search parameters for the MedicationKnowledge resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MedicationKnowledgeSearchParameter {
    /// Business identifier for this medication
    Identifier,
    /// active | inactive | entered-in-error
    Status,
    /// Specific category assigned to the medication
    Classification,
    /// The type of category for the medication (for example, therapeutic classification, therapeutic sub-classification)
    ClassificationType,
    /// Code that identifies this medication
    Code,
    /// powder | tablets | capsule +
    Doseform,
    /// Reference to a resource (by instance)
    Ingredient,
    /// Reference to a concept (by class)
    IngredientCode,
    /// Name of the reviewing program
    MonitoringProgramName,
    /// Type of program under which the medication is monitored
    MonitoringProgramType,
    /// Associated documentation about the medication
    Monograph,
    /// The category of medication document
    MonographType,
    /// Category of the medication or product
    ProductType,
    /// The source or owner for the price information
    SourceCost,
}
impl ResourceSearchParameterDefinition for MedicationKnowledgeSearchParameter {
    fn resource_type(&self) -> &'static str {
        "MedicationKnowledge"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Status => "status",
            Self::Classification => "classification",
            Self::ClassificationType => "classification-type",
            Self::Code => "code",
            Self::Doseform => "doseform",
            Self::Ingredient => "ingredient",
            Self::IngredientCode => "ingredient-code",
            Self::MonitoringProgramName => "monitoring-program-name",
            Self::MonitoringProgramType => "monitoring-program-type",
            Self::Monograph => "monograph",
            Self::MonographType => "monograph-type",
            Self::ProductType => "product-type",
            Self::SourceCost => "source-cost",
        }
    }
}
/// Search parameters for the MedicationRequest resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MedicationRequestSearchParameter {
    /// Return prescriptions with this external identifier
    Identifier,
    /// Returns prescriptions for a specific patient
    Patient,
    /// Return prescriptions of this medication code
    Code,
    /// Return prescriptions with this encounter identifier
    Encounter,
    /// Return prescriptions for this medication reference
    Medication,
    /// Status of the prescription
    Status,
    /// Return prescriptions written on this date
    Authoredon,
    /// Returns prescriptions with different categories
    Category,
    /// Returns medication request to be administered on a specific date or within a date range
    ComboDate,
    /// Composite request this is part of
    GroupIdentifier,
    /// Returns prescriptions intended to be dispensed by this Organization
    IntendedDispenser,
    /// Returns the intended performer of the administration of the medication request
    IntendedPerformer,
    /// Returns requests for a specific type of performer
    IntendedPerformertype,
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
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Code => "code",
            Self::Encounter => "encounter",
            Self::Medication => "medication",
            Self::Status => "status",
            Self::Authoredon => "authoredon",
            Self::Category => "category",
            Self::ComboDate => "combo-date",
            Self::GroupIdentifier => "group-identifier",
            Self::IntendedDispenser => "intended-dispenser",
            Self::IntendedPerformer => "intended-performer",
            Self::IntendedPerformertype => "intended-performertype",
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
    /// Return statements with this external identifier
    Identifier,
    /// Returns statements for a specific patient.
    Patient,
    /// Return statements of this medication code
    Code,
    /// Returns statements for a specific encounter
    Encounter,
    /// Return statements of this medication reference
    Medication,
    /// Return statements that match the given status
    Status,
    /// Returns statements based on adherence or compliance
    Adherence,
    /// Returns statements of this category of MedicationStatement
    Category,
    /// Date when patient was taking (or not taking) the medication
    Effective,
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
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Code => "code",
            Self::Encounter => "encounter",
            Self::Medication => "medication",
            Self::Status => "status",
            Self::Adherence => "adherence",
            Self::Category => "category",
            Self::Effective => "effective",
            Self::Source => "source",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the MedicinalProductDefinition resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MedicinalProductDefinitionSearchParameter {
    /// Allows the key product features to be recorded, such as "sugar free", "modified release", "parallel import"
    Characteristic,
    /// A category for the characteristic
    CharacteristicType,
    /// A product specific contact, person (in a role), or an organization
    Contact,
    /// If this medicine applies to human or veterinary uses
    Domain,
    /// Business identifier for this product. Could be an MPID
    Identifier,
    /// An ingredient of this product
    Ingredient,
    /// A master file for to the medicinal product (e.g. Pharmacovigilance System Master File)
    MasterFile,
    /// The full product name
    Name,
    /// Language code for this name
    NameLanguage,
    /// Allows the product to be classified by various systems
    ProductClassification,
    /// The status within the lifecycle of this product record. A high-level status, this is not intended to duplicate details carried elsewhere such as legal status, or authorization status
    Status,
    /// Regulatory type, e.g. Investigational or Authorized
    Type,
}
impl ResourceSearchParameterDefinition for MedicinalProductDefinitionSearchParameter {
    fn resource_type(&self) -> &'static str {
        "MedicinalProductDefinition"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Characteristic => "characteristic",
            Self::CharacteristicType => "characteristic-type",
            Self::Contact => "contact",
            Self::Domain => "domain",
            Self::Identifier => "identifier",
            Self::Ingredient => "ingredient",
            Self::MasterFile => "master-file",
            Self::Name => "name",
            Self::NameLanguage => "name-language",
            Self::ProductClassification => "product-classification",
            Self::Status => "status",
            Self::Type => "type",
        }
    }
}
/// Search parameters for the MessageDefinition resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MessageDefinitionSearchParameter {
    /// A use context assigned to the message definition
    Context,
    /// A quantity- or range-valued use context assigned to the message definition
    ContextQuantity,
    /// A type of use context assigned to the message definition
    ContextType,
    /// The message definition publication date
    Date,
    /// The description of the message definition
    Description,
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
    /// A use context type and quantity- or range-based value assigned to the message definition
    ContextTypeQuantity,
    /// A use context type and value assigned to the message definition
    ContextTypeValue,
    /// The behavior associated with the message
    Category,
    /// The event that triggers the message or link to the event definition.
    Event,
    /// A resource that is a permitted focus of the message
    Focus,
    /// A resource that is the parent of the definition
    Parent,
}
impl ResourceSearchParameterDefinition for MessageDefinitionSearchParameter {
    fn resource_type(&self) -> &'static str {
        "MessageDefinition"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Context => "context",
            Self::ContextQuantity => "context-quantity",
            Self::ContextType => "context-type",
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
            Self::ContextTypeQuantity => "context-type-quantity",
            Self::ContextTypeValue => "context-type-value",
            Self::Category => "category",
            Self::Event => "event",
            Self::Focus => "focus",
            Self::Parent => "parent",
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
    /// Code for the event this message represents or link to event definition
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
    /// Particular delivery destination within the destination
    Target,
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
            Self::Event => "event",
            Self::Focus => "focus",
            Self::Receiver => "receiver",
            Self::ResponseId => "response-id",
            Self::Responsible => "responsible",
            Self::Sender => "sender",
            Self::Source => "source",
            Self::Target => "target",
        }
    }
}
/// Search parameters for the MolecularSequence resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MolecularSequenceSearchParameter {
    /// The unique identity for a particular sequence
    Identifier,
    /// The subject that the sequence is about
    Patient,
    /// Amino Acid Sequence/ DNA Sequence / RNA Sequence
    Type,
    /// What the molecular sequence is about, when it is not about the subject of record
    Focus,
    /// The subject that the sequence is about
    Subject,
}
impl ResourceSearchParameterDefinition for MolecularSequenceSearchParameter {
    fn resource_type(&self) -> &'static str {
        "MolecularSequence"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Type => "type",
            Self::Focus => "focus",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the NamingSystem resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum NamingSystemSearchParameter {
    /// A use context assigned to the naming system
    Context,
    /// A quantity- or range-valued use context assigned to the naming system
    ContextQuantity,
    /// A type of use context assigned to the naming system
    ContextType,
    /// The naming system publication date
    Date,
    /// A resource that the NamingSystem is derived from
    DerivedFrom,
    /// The description of the naming system
    Description,
    /// The time during which the NamingSystem is intended to be in use
    Effective,
    /// External identifier for the naming system
    Identifier,
    /// Intended jurisdiction for the naming system
    Jurisdiction,
    /// Computationally friendly name of the naming system
    Name,
    /// The predecessor of the NamingSystem
    Predecessor,
    /// Name of the publisher of the naming system
    Publisher,
    /// The current status of the naming system
    Status,
    /// Topics associated with the NamingSystem
    Topic,
    /// The uri that identifies the naming system
    Url,
    /// The business version of the naming system
    Version,
    /// A use context type and quantity- or range-based value assigned to the naming system
    ContextTypeQuantity,
    /// A use context type and value assigned to the naming system
    ContextTypeValue,
    /// Name of an individual to contact
    Contact,
    /// oid | uuid | uri | other
    IdType,
    /// codesystem | identifier | root
    Kind,
    /// When is identifier valid?
    Period,
    /// Who maintains system namespace?
    Responsible,
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
            Self::Context => "context",
            Self::ContextQuantity => "context-quantity",
            Self::ContextType => "context-type",
            Self::Date => "date",
            Self::DerivedFrom => "derived-from",
            Self::Description => "description",
            Self::Effective => "effective",
            Self::Identifier => "identifier",
            Self::Jurisdiction => "jurisdiction",
            Self::Name => "name",
            Self::Predecessor => "predecessor",
            Self::Publisher => "publisher",
            Self::Status => "status",
            Self::Topic => "topic",
            Self::Url => "url",
            Self::Version => "version",
            Self::ContextTypeQuantity => "context-type-quantity",
            Self::ContextTypeValue => "context-type-value",
            Self::Contact => "contact",
            Self::IdType => "id-type",
            Self::Kind => "kind",
            Self::Period => "period",
            Self::Responsible => "responsible",
            Self::Telecom => "telecom",
            Self::Type => "type",
            Self::Value => "value",
        }
    }
}
/// Search parameters for the NutritionIntake resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum NutritionIntakeSearchParameter {
    /// Return statements with this external identifier
    Identifier,
    /// Returns statements for a specific patient.
    Patient,
    /// Returns statements of this code of NutritionIntake
    Code,
    /// Date when patient was taking (or not taking) the medication
    Date,
    /// Returns statements for a specific encounter
    Encounter,
    /// Return intakes for a specific consumed item
    Nutrition,
    /// Who or where the information in the statement came from
    Source,
    /// Return statements that match the given status
    Status,
    /// The identity of a patient, animal or group to list statements for
    Subject,
}
impl ResourceSearchParameterDefinition for NutritionIntakeSearchParameter {
    fn resource_type(&self) -> &'static str {
        "NutritionIntake"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Code => "code",
            Self::Date => "date",
            Self::Encounter => "encounter",
            Self::Nutrition => "nutrition",
            Self::Source => "source",
            Self::Status => "status",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the NutritionOrder resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum NutritionOrderSearchParameter {
    /// Return nutrition orders with this external identifier
    Identifier,
    /// The identity of the individual or set of individuals who requires the diet, formula or nutritional supplement
    Patient,
    /// Return nutrition orders with this encounter identifier
    Encounter,
    /// Type of module component to add to the feeding
    Additive,
    /// Return nutrition orders requested on this date
    Datetime,
    /// Type of enteral or infant formula
    Formula,
    /// Composite Request ID
    GroupIdentifier,
    /// Type of diet that can be consumed orally (i.e., take via the mouth).
    Oraldiet,
    /// The identity of the provider who placed the nutrition order
    Provider,
    /// Status of the nutrition order.
    Status,
    /// The identity of the individual or set of individuals who requires the diet, formula or nutritional supplement
    Subject,
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
            Self::GroupIdentifier => "group-identifier",
            Self::Oraldiet => "oraldiet",
            Self::Provider => "provider",
            Self::Status => "status",
            Self::Subject => "subject",
            Self::Supplement => "supplement",
        }
    }
}
/// Search parameters for the NutritionProduct resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum NutritionProductSearchParameter {
    /// The code identifying a specific nutrition product
    Code,
    /// The identifier for the physical instance, typically a serial number
    Identifier,
    /// The lot number
    LotNumber,
    /// The serial number
    SerialNumber,
    /// The broad product group or category of the nutrition product
    Status,
}
impl ResourceSearchParameterDefinition for NutritionProductSearchParameter {
    fn resource_type(&self) -> &'static str {
        "NutritionProduct"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Code => "code",
            Self::Identifier => "identifier",
            Self::LotNumber => "lot-number",
            Self::SerialNumber => "serial-number",
            Self::Status => "status",
        }
    }
}
/// Search parameters for the Observation resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ObservationSearchParameter {
    /// The unique id for a particular observation
    Identifier,
    /// The subject that the observation is about (if patient)
    Patient,
    /// The code of the observation type
    Code,
    /// Clinically relevant time/time-period for observation
    Date,
    /// Encounter related to the observation
    Encounter,
    /// Reference to the service request.
    BasedOn,
    /// The classification of the type of observation
    Category,
    /// The code of the observation type or component type
    ComboCode,
    /// The reason why the expected value in the element Observation.value[x] or Observation.component.value[x] is missing.
    ComboDataAbsentReason,
    /// The value or component value of the observation, if the value is a CodeableConcept
    ComboValueConcept,
    /// The value or component value of the observation, if the value is a Quantity, or a SampledData (just search on the bounds of the values in sampled data)
    ComboValueQuantity,
    /// The component code of the observation type
    ComponentCode,
    /// The reason why the expected value in the element Observation.component.value[x] is missing.
    ComponentDataAbsentReason,
    /// URL contained in valueCanonical.
    ComponentValueCanonical,
    /// The value of the component observation, if the value is a CodeableConcept
    ComponentValueConcept,
    /// The value of the component observation, if the value is a Quantity, or a SampledData (just search on the bounds of the values in sampled data)
    ComponentValueQuantity,
    /// Reference contained in valueReference.
    ComponentValueReference,
    /// The reason why the expected value in the element Observation.value[x] is missing.
    DataAbsentReason,
    /// Related measurements the observation is made from
    DerivedFrom,
    /// The Device that generated the observation data.
    Device,
    /// The focus of an observation when the focus is not the patient of record.
    Focus,
    /// Related resource that belongs to the Observation group
    HasMember,
    /// The method used for the observation
    Method,
    /// Part of referenced event
    PartOf,
    /// Who performed the observation
    Performer,
    /// Specimen used for this observation
    Specimen,
    /// The status of the observation
    Status,
    /// The subject that the observation is about
    Subject,
    /// URL contained in valueCanonical.
    ValueCanonical,
    /// The value of the observation, if the value is a CodeableConcept
    ValueConcept,
    /// The value of the observation, if the value is a date or period of time
    ValueDate,
    /// The value of the observation, if the value is a string, and also searches in CodeableConcept.text
    ValueMarkdown,
    /// The value of the observation, if the value is a Quantity, or a SampledData (just search on the bounds of the values in sampled data)
    ValueQuantity,
    /// Reference contained in valueReference.
    ValueReference,
    /// Code and coded value parameter pair
    CodeValueConcept,
    /// Code and date/time value parameter pair
    CodeValueDate,
    /// Code and quantity value parameter pair
    CodeValueQuantity,
    /// Code and string value parameter pair
    CodeValueString,
    /// Code and coded value parameter pair, including in components
    ComboCodeValueConcept,
    /// Code and quantity value parameter pair, including in components
    ComboCodeValueQuantity,
    /// Component code and component coded value parameter pair
    ComponentCodeValueConcept,
    /// Component code and component quantity value parameter pair
    ComponentCodeValueQuantity,
}
impl ResourceSearchParameterDefinition for ObservationSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Observation"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Code => "code",
            Self::Date => "date",
            Self::Encounter => "encounter",
            Self::BasedOn => "based-on",
            Self::Category => "category",
            Self::ComboCode => "combo-code",
            Self::ComboDataAbsentReason => "combo-data-absent-reason",
            Self::ComboValueConcept => "combo-value-concept",
            Self::ComboValueQuantity => "combo-value-quantity",
            Self::ComponentCode => "component-code",
            Self::ComponentDataAbsentReason => "component-data-absent-reason",
            Self::ComponentValueCanonical => "component-value-canonical",
            Self::ComponentValueConcept => "component-value-concept",
            Self::ComponentValueQuantity => "component-value-quantity",
            Self::ComponentValueReference => "component-value-reference",
            Self::DataAbsentReason => "data-absent-reason",
            Self::DerivedFrom => "derived-from",
            Self::Device => "device",
            Self::Focus => "focus",
            Self::HasMember => "has-member",
            Self::Method => "method",
            Self::PartOf => "part-of",
            Self::Performer => "performer",
            Self::Specimen => "specimen",
            Self::Status => "status",
            Self::Subject => "subject",
            Self::ValueCanonical => "value-canonical",
            Self::ValueConcept => "value-concept",
            Self::ValueDate => "value-date",
            Self::ValueMarkdown => "value-markdown",
            Self::ValueQuantity => "value-quantity",
            Self::ValueReference => "value-reference",
            Self::CodeValueConcept => "code-value-concept",
            Self::CodeValueDate => "code-value-date",
            Self::CodeValueQuantity => "code-value-quantity",
            Self::CodeValueString => "code-value-string",
            Self::ComboCodeValueConcept => "combo-code-value-concept",
            Self::ComboCodeValueQuantity => "combo-code-value-quantity",
            Self::ComponentCodeValueConcept => "component-code-value-concept",
            Self::ComponentCodeValueQuantity => "component-code-value-quantity",
        }
    }
}
/// Search parameters for the ObservationDefinition resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ObservationDefinitionSearchParameter {
    /// The unique identifier associated with the specimen definition
    Identifier,
    /// Publication status of the ObservationDefinition: draft, active, retired, unknown
    Status,
    /// Human-friendly name of the ObservationDefinition
    Title,
    /// The uri that identifies the observation definition
    Url,
    /// Category (class) of observation
    Category,
    /// Observation code
    Code,
    /// Not for genuine usage (true)
    Experimental,
    /// Method of observation
    Method,
}
impl ResourceSearchParameterDefinition for ObservationDefinitionSearchParameter {
    fn resource_type(&self) -> &'static str {
        "ObservationDefinition"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Status => "status",
            Self::Title => "title",
            Self::Url => "url",
            Self::Category => "category",
            Self::Code => "code",
            Self::Experimental => "experimental",
            Self::Method => "method",
        }
    }
}
/// Search parameters for the OperationDefinition resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum OperationDefinitionSearchParameter {
    /// A use context assigned to the operation definition
    Context,
    /// A quantity- or range-valued use context assigned to the operation definition
    ContextQuantity,
    /// A type of use context assigned to the operation definition
    ContextType,
    /// The operation definition publication date
    Date,
    /// The description of the operation definition
    Description,
    /// External identifier for the search parameter
    Identifier,
    /// Intended jurisdiction for the operation definition
    Jurisdiction,
    /// Computationally friendly name of the operation definition
    Name,
    /// Name of the publisher of the operation definition
    Publisher,
    /// The current status of the operation definition
    Status,
    /// The human-friendly name of the operation definition
    Title,
    /// The uri that identifies the operation definition
    Url,
    /// The business version of the operation definition
    Version,
    /// A use context type and quantity- or range-based value assigned to the operation definition
    ContextTypeQuantity,
    /// A use context type and value assigned to the operation definition
    ContextTypeValue,
    /// Marks this as a profile of the base
    Base,
    /// Name used to invoke the operation
    Code,
    /// Validation information for in parameters
    InputProfile,
    /// Invoke on an instance?
    Instance,
    /// operation | query
    Kind,
    /// Validation information for out parameters
    OutputProfile,
    /// Invoke at the system level?
    System,
    /// Invoke at the type level?
    Type,
}
impl ResourceSearchParameterDefinition for OperationDefinitionSearchParameter {
    fn resource_type(&self) -> &'static str {
        "OperationDefinition"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Context => "context",
            Self::ContextQuantity => "context-quantity",
            Self::ContextType => "context-type",
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
            Self::ContextTypeQuantity => "context-type-quantity",
            Self::ContextTypeValue => "context-type-value",
            Self::Base => "base",
            Self::Code => "code",
            Self::InputProfile => "input-profile",
            Self::Instance => "instance",
            Self::Kind => "kind",
            Self::OutputProfile => "output-profile",
            Self::System => "system",
            Self::Type => "type",
        }
    }
}
/// Search parameters for the Organization resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum OrganizationSearchParameter {
    /// Is the Organization record active
    Active,
    /// A server defined search that may match any of the string fields in the Address, including line, city, district, state, country, postalCode, and/or text
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
/// Search parameters for the OrganizationAffiliation resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum OrganizationAffiliationSearchParameter {
    /// Whether this organization affiliation record is in active use
    Active,
    /// The period during which the participatingOrganization is affiliated with the primary organization
    Date,
    /// A value in an email contact
    Email,
    /// Technical endpoints providing access to services operated for this role
    Endpoint,
    /// An organization affiliation's Identifier
    Identifier,
    /// The location(s) at which the role occurs
    Location,
    /// Health insurance provider network in which the participatingOrganization provides the role's services (if defined) at the indicated locations (if defined)
    Network,
    /// The organization that provides services to the primary organization
    ParticipatingOrganization,
    /// A value in a phone contact
    Phone,
    /// The organization that receives the services from the participating organization
    PrimaryOrganization,
    /// Definition of the role the participatingOrganization plays
    Role,
    /// Healthcare services provided through the role
    Service,
    /// Specific specialty of the participatingOrganization in the context of the role
    Specialty,
    /// The value in any kind of contact
    Telecom,
}
impl ResourceSearchParameterDefinition for OrganizationAffiliationSearchParameter {
    fn resource_type(&self) -> &'static str {
        "OrganizationAffiliation"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Date => "date",
            Self::Email => "email",
            Self::Endpoint => "endpoint",
            Self::Identifier => "identifier",
            Self::Location => "location",
            Self::Network => "network",
            Self::ParticipatingOrganization => "participating-organization",
            Self::Phone => "phone",
            Self::PrimaryOrganization => "primary-organization",
            Self::Role => "role",
            Self::Service => "service",
            Self::Specialty => "specialty",
            Self::Telecom => "telecom",
        }
    }
}
/// Search parameters for the PackagedProductDefinition resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PackagedProductDefinitionSearchParameter {
    /// A biologically derived product within this packaged product
    Biological,
    /// Any of the contained items within this packaged product
    ContainedItem,
    /// A device within this packaged product
    Device,
    /// Unique identifier
    Identifier,
    /// A manufactured item of medication within this packaged product
    ManufacturedItem,
    /// A manufactured item of medication within this packaged product
    Medication,
    /// A name for this package. Typically what it would be listed as in a drug formulary or catalogue, inventory etc.
    Name,
    /// A nutrition product within this packaged product
    Nutrition,
    /// A complete packaged product within this packaged product
    Package,
    /// The product that this is a pack for
    PackageFor,
    /// The status within the lifecycle of this item. A high level status, this is not intended to duplicate details carried elsewhere such as legal status, or authorization or marketing status
    Status,
}
impl ResourceSearchParameterDefinition for PackagedProductDefinitionSearchParameter {
    fn resource_type(&self) -> &'static str {
        "PackagedProductDefinition"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Biological => "biological",
            Self::ContainedItem => "contained-item",
            Self::Device => "device",
            Self::Identifier => "identifier",
            Self::ManufacturedItem => "manufactured-item",
            Self::Medication => "medication",
            Self::Name => "name",
            Self::Nutrition => "nutrition",
            Self::Package => "package",
            Self::PackageFor => "package-for",
            Self::Status => "status",
        }
    }
}
/// Search parameters for the Patient resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PatientSearchParameter {
    /// Whether the patient record is active
    Active,
    /// A server defined search that may match any of the string fields in the Address, including line, city, district, state, country, postalCode, and/or text
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
    /// The patient's date of birth
    Birthdate,
    /// The date of death has been provided and satisfies this search value
    DeathDate,
    /// This patient has been marked as deceased, or has a death date entered
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
    /// All patients/related persons linked to the given patient
    Link,
    /// A server defined search that may match any of the string fields in the HumanName, including family, given, prefix, suffix, and/or text
    Name,
    /// The organization that is the custodian of the patient record
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
/// Search parameters for the Permission resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PermissionSearchParameter {
    /// active | entered-in-error | draft | rejected
    Status,
}
impl ResourceSearchParameterDefinition for PermissionSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Permission"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Status => "status",
        }
    }
}
/// Search parameters for the Person resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PersonSearchParameter {
    /// A person Identifier
    Identifier,
    /// The Person links to this Patient
    Patient,
    /// A server defined search that may match any of the string fields in the Address, including line, city, district, state, country, postalCode, and/or text
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
    /// The date of death has been provided and satisfies this search value
    DeathDate,
    /// This person has been marked as deceased, or has a death date entered
    Deceased,
    /// A portion of the family name of the person
    Family,
    /// A portion of the given name of the person
    Given,
    /// Any link has this Patient, Person, RelatedPerson or Practitioner reference
    Link,
    /// A server defined search that may match any of the string fields in the HumanName, including family, give, prefix, suffix, suffix, and/or text
    Name,
    /// The organization at which this person record is being managed
    Organization,
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
            Self::Identifier => "identifier",
            Self::Patient => "patient",
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
            Self::DeathDate => "death-date",
            Self::Deceased => "deceased",
            Self::Family => "family",
            Self::Given => "given",
            Self::Link => "link",
            Self::Name => "name",
            Self::Organization => "organization",
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
    /// A use context assigned to the plan definition
    Context,
    /// A quantity- or range-valued use context assigned to the plan definition
    ContextQuantity,
    /// A type of use context assigned to the plan definition
    ContextType,
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
    /// A use context type and quantity- or range-based value assigned to the plan definition
    ContextTypeQuantity,
    /// A use context type and value assigned to the plan definition
    ContextTypeValue,
    /// Activity or plan definitions used by plan definition
    Definition,
    /// The type of artifact the plan (e.g. order-set, eca-rule, protocol)
    Type,
}
impl ResourceSearchParameterDefinition for PlanDefinitionSearchParameter {
    fn resource_type(&self) -> &'static str {
        "PlanDefinition"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::ComposedOf => "composed-of",
            Self::Context => "context",
            Self::ContextQuantity => "context-quantity",
            Self::ContextType => "context-type",
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
            Self::ContextTypeQuantity => "context-type-quantity",
            Self::ContextTypeValue => "context-type-value",
            Self::Definition => "definition",
            Self::Type => "type",
        }
    }
}
/// Search parameters for the Practitioner resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PractitionerSearchParameter {
    /// A server defined search that may match any of the string fields in the Address, including line, city, district, state, country, postalCode, and/or text
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
    /// A language to communicate with the practitioner
    Communication,
    /// The date of death has been provided and satisfies this search value
    DeathDate,
    /// This Practitioner has been marked as deceased, or has a death date entered
    Deceased,
    /// A practitioner's Identifier
    Identifier,
    /// A server defined search that may match any of the string fields in the HumanName, including family, give, prefix, suffix, suffix, and/or text
    Name,
    /// The date(s) a qualification is valid for
    QualificationPeriod,
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
            Self::DeathDate => "death-date",
            Self::Deceased => "deceased",
            Self::Identifier => "identifier",
            Self::Name => "name",
            Self::QualificationPeriod => "qualification-period",
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
    /// Whether this practitioner role record is in active use
    Active,
    /// One of the PractitionerRole's characteristics
    Characteristic,
    /// One of the languages that the practitioner can communicate with
    Communication,
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
    /// Practitioner that is able to provide the defined services for the organization
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
            Self::Characteristic => "characteristic",
            Self::Communication => "communication",
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
    /// A unique identifier for a procedure
    Identifier,
    /// Search by subject - a patient
    Patient,
    /// A code to identify a  procedure
    Code,
    /// When the procedure occurred or is occurring
    Date,
    /// The Encounter during which this Procedure was created
    Encounter,
    /// A request for this procedure
    BasedOn,
    /// Classification of the procedure
    Category,
    /// Instantiates FHIR protocol or definition
    InstantiatesCanonical,
    /// Instantiates external protocol or definition
    InstantiatesUri,
    /// Where the procedure happened
    Location,
    /// Part of referenced event
    PartOf,
    /// Who performed the procedure
    Performer,
    /// Reference to a concept (by class)
    ReasonCode,
    /// Reference to a resource (by instance)
    ReasonReference,
    /// Any report resulting from the procedure
    Report,
    /// preparation | in-progress | not-done | on-hold | stopped | completed | entered-in-error | unknown
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
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Code => "code",
            Self::Date => "date",
            Self::Encounter => "encounter",
            Self::BasedOn => "based-on",
            Self::Category => "category",
            Self::InstantiatesCanonical => "instantiates-canonical",
            Self::InstantiatesUri => "instantiates-uri",
            Self::Location => "location",
            Self::PartOf => "part-of",
            Self::Performer => "performer",
            Self::ReasonCode => "reason-code",
            Self::ReasonReference => "reason-reference",
            Self::Report => "report",
            Self::Status => "status",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the Provenance resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ProvenanceSearchParameter {
    /// Where the activity involved patient data
    Patient,
    /// Encounter related to the Provenance
    Encounter,
    /// Activity that occurred
    Activity,
    /// Who participated
    Agent,
    /// What the agents role was
    AgentRole,
    /// How the agent participated
    AgentType,
    /// Reference to the service request.
    BasedOn,
    /// Identity of entity
    Entity,
    /// Where the activity occurred, if relevant
    Location,
    /// When the activity was recorded / updated
    Recorded,
    /// Indication of the reason the entity signed the object(s)
    SignatureType,
    /// Target Reference(s) (usually version specific)
    Target,
    /// When the activity occurred
    When,
}
impl ResourceSearchParameterDefinition for ProvenanceSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Provenance"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Patient => "patient",
            Self::Encounter => "encounter",
            Self::Activity => "activity",
            Self::Agent => "agent",
            Self::AgentRole => "agent-role",
            Self::AgentType => "agent-type",
            Self::BasedOn => "based-on",
            Self::Entity => "entity",
            Self::Location => "location",
            Self::Recorded => "recorded",
            Self::SignatureType => "signature-type",
            Self::Target => "target",
            Self::When => "when",
        }
    }
}
/// Search parameters for the Questionnaire resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum QuestionnaireSearchParameter {
    /// A use context assigned to the questionnaire
    Context,
    /// A quantity- or range-valued use context assigned to the questionnaire
    ContextQuantity,
    /// A type of use context assigned to the questionnaire
    ContextType,
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
    /// A use context type and quantity- or range-based value assigned to the questionnaire
    ContextTypeQuantity,
    /// A use context type and value assigned to the questionnaire
    ContextTypeValue,
    /// A code that corresponds to one of its items in the questionnaire
    ComboCode,
    /// ElementDefinition - details for the item
    Definition,
    /// A code that corresponds to one of the items in the questionnaire
    ItemCode,
    /// A code that matches one of the Questionnaire.code codings
    QuestionnaireCode,
    /// Resource that can be subject of QuestionnaireResponse
    SubjectType,
}
impl ResourceSearchParameterDefinition for QuestionnaireSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Questionnaire"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Context => "context",
            Self::ContextQuantity => "context-quantity",
            Self::ContextType => "context-type",
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
            Self::ContextTypeQuantity => "context-type-quantity",
            Self::ContextTypeValue => "context-type-value",
            Self::ComboCode => "combo-code",
            Self::Definition => "definition",
            Self::ItemCode => "item-code",
            Self::QuestionnaireCode => "questionnaire-code",
            Self::SubjectType => "subject-type",
        }
    }
}
/// Search parameters for the QuestionnaireResponse resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum QuestionnaireResponseSearchParameter {
    /// The unique identifier for the questionnaire response
    Identifier,
    /// The patient that is the subject of the questionnaire response
    Patient,
    /// Encounter associated with the questionnaire response
    Encounter,
    /// The author of the questionnaire response
    Author,
    /// When the questionnaire response was last changed
    Authored,
    /// Plan/proposal/order fulfilled by this questionnaire response
    BasedOn,
    /// Allows searching for QuestionnaireResponses by item value where the item has isSubject=true
    ItemSubject,
    /// Procedure or observation this questionnaire response was performed as a part of
    PartOf,
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
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Encounter => "encounter",
            Self::Author => "author",
            Self::Authored => "authored",
            Self::BasedOn => "based-on",
            Self::ItemSubject => "item-subject",
            Self::PartOf => "part-of",
            Self::Questionnaire => "questionnaire",
            Self::Source => "source",
            Self::Status => "status",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the RegulatedAuthorization resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum RegulatedAuthorizationSearchParameter {
    /// The case or procedure number
    Case,
    /// The defining type of case
    CaseType,
    /// The organization that holds the granted authorization
    Holder,
    /// Business identifier for the authorization, typically assigned by the authorizing body
    Identifier,
    /// The territory (e.g., country, jurisdiction etc.) in which the authorization has been granted
    Region,
    /// The status that is authorised e.g. approved. Intermediate states can be tracked with cases and applications
    Status,
    /// The type of regulated product, treatment, facility or activity that is being authorized
    Subject,
}
impl ResourceSearchParameterDefinition for RegulatedAuthorizationSearchParameter {
    fn resource_type(&self) -> &'static str {
        "RegulatedAuthorization"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Case => "case",
            Self::CaseType => "case-type",
            Self::Holder => "holder",
            Self::Identifier => "identifier",
            Self::Region => "region",
            Self::Status => "status",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the RelatedPerson resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum RelatedPersonSearchParameter {
    /// An Identifier of the RelatedPerson
    Identifier,
    /// The patient this related person is related to
    Patient,
    /// A server defined search that may match any of the string fields in the Address, including line, city, district, state, country, postalCode, and/or text
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
    /// A portion of the family name of the related person
    Family,
    /// A portion of the given name of the related person
    Given,
    /// A server defined search that may match any of the string fields in the HumanName, including family, give, prefix, suffix, suffix, and/or text
    Name,
    /// The relationship between the patient and the relatedperson
    Relationship,
}
impl ResourceSearchParameterDefinition for RelatedPersonSearchParameter {
    fn resource_type(&self) -> &'static str {
        "RelatedPerson"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
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
            Self::Family => "family",
            Self::Given => "given",
            Self::Name => "name",
            Self::Relationship => "relationship",
        }
    }
}
/// Search parameters for the RequestOrchestration resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum RequestOrchestrationSearchParameter {
    /// External identifiers for the request orchestration
    Identifier,
    /// The identity of a patient to search for request orchestrations
    Patient,
    /// The code of the request orchestration
    Code,
    /// The encounter the request orchestration applies to
    Encounter,
    /// The author of the request orchestration
    Author,
    /// The date the request orchestration was authored
    Authored,
    /// What this request fullfills.
    BasedOn,
    /// The group identifier for the request orchestration
    GroupIdentifier,
    /// The FHIR-based definition from which the request orchestration is realized
    InstantiatesCanonical,
    /// The external definition from which the request orchestration is realized
    InstantiatesUri,
    /// The intent of the request orchestration
    Intent,
    /// The participant in the requests in the orchestration
    Participant,
    /// The priority of the request orchestration
    Priority,
    /// The status of the request orchestration
    Status,
    /// The subject that the request orchestration is about
    Subject,
}
impl ResourceSearchParameterDefinition for RequestOrchestrationSearchParameter {
    fn resource_type(&self) -> &'static str {
        "RequestOrchestration"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Code => "code",
            Self::Encounter => "encounter",
            Self::Author => "author",
            Self::Authored => "authored",
            Self::BasedOn => "based-on",
            Self::GroupIdentifier => "group-identifier",
            Self::InstantiatesCanonical => "instantiates-canonical",
            Self::InstantiatesUri => "instantiates-uri",
            Self::Intent => "intent",
            Self::Participant => "participant",
            Self::Priority => "priority",
            Self::Status => "status",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the Requirements resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum RequirementsSearchParameter {
    /// A use context assigned to the requirements
    Context,
    /// A quantity- or range-valued use context assigned to the requirements
    ContextQuantity,
    /// A type of use context assigned to the requirements
    ContextType,
    /// The requirements publication date
    Date,
    /// The description of the requirements
    Description,
    /// External identifier for the requirements
    Identifier,
    /// Intended jurisdiction for the requirements
    Jurisdiction,
    /// Computationally friendly name of the requirements
    Name,
    /// Name of the publisher of the requirements
    Publisher,
    /// The current status of the requirements
    Status,
    /// The human-friendly name of the requirements
    Title,
    /// The uri that identifies the requirements
    Url,
    /// The business version of the requirements
    Version,
    /// A use context type and quantity- or range-based value assigned to the requirements
    ContextTypeQuantity,
    /// A use context type and value assigned to the requirements
    ContextTypeValue,
    /// An actor these requirements are for
    Actor,
    /// The requirements these are derived from
    DerivedFrom,
}
impl ResourceSearchParameterDefinition for RequirementsSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Requirements"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Context => "context",
            Self::ContextQuantity => "context-quantity",
            Self::ContextType => "context-type",
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
            Self::ContextTypeQuantity => "context-type-quantity",
            Self::ContextTypeValue => "context-type-value",
            Self::Actor => "actor",
            Self::DerivedFrom => "derived-from",
        }
    }
}
/// Search parameters for the ResearchSubject resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ResearchSubjectSearchParameter {
    /// Business Identifier for research subject in a study
    Identifier,
    /// Who or what is part of study
    Patient,
    /// Start and end of participation
    Date,
    /// draft | active | retired | unknown
    Status,
    /// Study subject is part of
    Study,
    /// Who or what is part of study
    Subject,
    /// candidate | eligible | follow-up | ineligible | not-registered | off-study | on-study | on-study-intervention | on-study-observation | pending-on-study | potential-candidate | screening | withdrawn
    SubjectState,
}
impl ResourceSearchParameterDefinition for ResearchSubjectSearchParameter {
    fn resource_type(&self) -> &'static str {
        "ResearchSubject"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Date => "date",
            Self::Status => "status",
            Self::Study => "study",
            Self::Subject => "subject",
            Self::SubjectState => "subject_state",
        }
    }
}
/// Search parameters for the RiskAssessment resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum RiskAssessmentSearchParameter {
    /// Unique identifier for the assessment
    Identifier,
    /// Who/what does assessment apply to?
    Patient,
    /// When was assessment made?
    Date,
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
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Date => "date",
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
    /// A use context assigned to the search parameter
    Context,
    /// A quantity- or range-valued use context assigned to the search parameter
    ContextQuantity,
    /// A type of use context assigned to the search parameter
    ContextType,
    /// The search parameter publication date
    Date,
    /// The description of the search parameter
    Description,
    /// External identifier for the search parameter
    Identifier,
    /// Intended jurisdiction for the search parameter
    Jurisdiction,
    /// Computationally friendly name of the search parameter
    Name,
    /// Name of the publisher of the search parameter
    Publisher,
    /// The current status of the search parameter
    Status,
    /// The uri that identifies the search parameter
    Url,
    /// The business version of the search parameter
    Version,
    /// A use context type and quantity- or range-based value assigned to the search parameter
    ContextTypeQuantity,
    /// A use context type and value assigned to the search parameter
    ContextTypeValue,
    /// The resource type(s) this search parameter applies to
    Base,
    /// Code used in URL
    Code,
    /// Defines how the part works
    Component,
    /// Original definition for the search parameter
    DerivedFrom,
    /// Types of resource (if a resource reference)
    Target,
    /// number | date | string | token | reference | composite | quantity | uri | special
    Type,
}
impl ResourceSearchParameterDefinition for SearchParameterSearchParameter {
    fn resource_type(&self) -> &'static str {
        "SearchParameter"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Context => "context",
            Self::ContextQuantity => "context-quantity",
            Self::ContextType => "context-type",
            Self::Date => "date",
            Self::Description => "description",
            Self::Identifier => "identifier",
            Self::Jurisdiction => "jurisdiction",
            Self::Name => "name",
            Self::Publisher => "publisher",
            Self::Status => "status",
            Self::Url => "url",
            Self::Version => "version",
            Self::ContextTypeQuantity => "context-type-quantity",
            Self::ContextTypeValue => "context-type-value",
            Self::Base => "base",
            Self::Code => "code",
            Self::Component => "component",
            Self::DerivedFrom => "derived-from",
            Self::Target => "target",
            Self::Type => "type",
        }
    }
}
/// Search parameters for the ServiceRequest resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ServiceRequestSearchParameter {
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
    /// Body structure Where procedure is going to be done
    BodyStructure,
    /// Classification of service
    Category,
    /// What is being requested/ordered
    CodeConcept,
    /// What is being requested/ordered
    CodeReference,
    /// Instantiates FHIR protocol or definition
    InstantiatesCanonical,
    /// Instantiates external protocol or definition
    InstantiatesUri,
    /// proposal | plan | directive | order +
    Intent,
    /// When service should occur
    Occurrence,
    /// Requested performer
    Performer,
    /// Performer role
    PerformerType,
    /// routine | urgent | asap | stat
    Priority,
    /// What request replaces
    Replaces,
    /// Who/what is requesting service
    Requester,
    /// Composite Request ID
    Requisition,
    /// Specimen to be tested
    Specimen,
    /// draft | active | on-hold | revoked | completed | entered-in-error | unknown
    Status,
    /// Search by subject
    Subject,
}
impl ResourceSearchParameterDefinition for ServiceRequestSearchParameter {
    fn resource_type(&self) -> &'static str {
        "ServiceRequest"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Encounter => "encounter",
            Self::Authored => "authored",
            Self::BasedOn => "based-on",
            Self::BodySite => "body-site",
            Self::BodyStructure => "body-structure",
            Self::Category => "category",
            Self::CodeConcept => "code-concept",
            Self::CodeReference => "code-reference",
            Self::InstantiatesCanonical => "instantiates-canonical",
            Self::InstantiatesUri => "instantiates-uri",
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
/// Search parameters for the Specimen resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SpecimenSearchParameter {
    /// The unique identifier associated with the specimen
    Identifier,
    /// The patient the specimen comes from
    Patient,
    /// The specimen type
    Type,
    /// The accession number associated with the specimen
    Accession,
    /// Reference to a resource (by instance)
    Bodysite,
    /// The date the specimen was collected
    Collected,
    /// Who collected the specimen
    Collector,
    /// The unique identifier associated with the specimen container
    ContainerDevice,
    /// The parent of the specimen
    Parent,
    /// The procedure that collected the specimen
    Procedure,
    /// available | unavailable | unsatisfactory | entered-in-error
    Status,
    /// The subject of the specimen
    Subject,
}
impl ResourceSearchParameterDefinition for SpecimenSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Specimen"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Type => "type",
            Self::Accession => "accession",
            Self::Bodysite => "bodysite",
            Self::Collected => "collected",
            Self::Collector => "collector",
            Self::ContainerDevice => "container-device",
            Self::Parent => "parent",
            Self::Procedure => "procedure",
            Self::Status => "status",
            Self::Subject => "subject",
        }
    }
}
/// Search parameters for the SpecimenDefinition resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SpecimenDefinitionSearchParameter {
    /// The unique identifier associated with the SpecimenDefinition
    Identifier,
    /// Publication status of the SpecimenDefinition: draft, active, retired, unknown
    Status,
    /// Human-friendly name of the SpecimenDefinition
    Title,
    /// The uri that identifies the specimen definition
    Url,
    /// The type of specimen conditioned in container expected by the lab
    Container,
    /// Not for genuine usage (true)
    Experimental,
    /// Primary specimen (false) or derived specimen (true)
    IsDerived,
    /// The type of collected specimen
    Type,
    /// The type of specimen conditioned for testing
    TypeTested,
}
impl ResourceSearchParameterDefinition for SpecimenDefinitionSearchParameter {
    fn resource_type(&self) -> &'static str {
        "SpecimenDefinition"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Status => "status",
            Self::Title => "title",
            Self::Url => "url",
            Self::Container => "container",
            Self::Experimental => "experimental",
            Self::IsDerived => "is-derived",
            Self::Type => "type",
            Self::TypeTested => "type-tested",
        }
    }
}
/// Search parameters for the StructureDefinition resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum StructureDefinitionSearchParameter {
    /// A use context assigned to the structure definition
    Context,
    /// A quantity- or range-valued use context assigned to the structure definition
    ContextQuantity,
    /// A type of use context assigned to the structure definition
    ContextType,
    /// The structure definition publication date
    Date,
    /// The description of the structure definition
    Description,
    /// External identifier for the structure definition
    Identifier,
    /// Intended jurisdiction for the structure definition
    Jurisdiction,
    /// Computationally friendly name of the structure definition
    Name,
    /// Name of the publisher of the structure definition
    Publisher,
    /// The current status of the structure definition
    Status,
    /// The human-friendly name of the structure definition
    Title,
    /// The uri that identifies the structure definition
    Url,
    /// The business version of the structure definition
    Version,
    /// A use context type and quantity- or range-based value assigned to the structure definition
    ContextTypeQuantity,
    /// A use context type and value assigned to the structure definition
    ContextTypeValue,
    /// Whether the structure is abstract
    Abstract,
    /// Definition that this type is constrained/specialized from
    Base,
    /// Path that identifies the base element
    BasePath,
    /// specialization | constraint - How relates to base definition
    Derivation,
    /// For testing purposes, not real usage
    Experimental,
    /// An expression of extension context assigned to the structure definition
    ExtContextExpression,
    /// A type of extension context assigned to the structure definition
    ExtContextType,
    /// A code for the StructureDefinition
    Keyword,
    /// primitive-type | complex-type | resource | logical
    Kind,
    /// A path that is constrained in the StructureDefinition
    Path,
    /// Type defined or constrained by this structure
    Type,
    /// A vocabulary binding reference
    Valueset,
    /// An extension context assigned to the structure definition
    ExtContext,
}
impl ResourceSearchParameterDefinition for StructureDefinitionSearchParameter {
    fn resource_type(&self) -> &'static str {
        "StructureDefinition"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Context => "context",
            Self::ContextQuantity => "context-quantity",
            Self::ContextType => "context-type",
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
            Self::ContextTypeQuantity => "context-type-quantity",
            Self::ContextTypeValue => "context-type-value",
            Self::Abstract => "abstract",
            Self::Base => "base",
            Self::BasePath => "base-path",
            Self::Derivation => "derivation",
            Self::Experimental => "experimental",
            Self::ExtContextExpression => "ext-context-expression",
            Self::ExtContextType => "ext-context-type",
            Self::Keyword => "keyword",
            Self::Kind => "kind",
            Self::Path => "path",
            Self::Type => "type",
            Self::Valueset => "valueset",
            Self::ExtContext => "ext-context",
        }
    }
}
/// Search parameters for the StructureMap resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum StructureMapSearchParameter {
    /// A use context assigned to the structure map
    Context,
    /// A quantity- or range-valued use context assigned to the structure map
    ContextQuantity,
    /// A type of use context assigned to the structure map
    ContextType,
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
    /// A use context type and quantity- or range-based value assigned to the structure map
    ContextTypeQuantity,
    /// A use context type and value assigned to the structure map
    ContextTypeValue,
}
impl ResourceSearchParameterDefinition for StructureMapSearchParameter {
    fn resource_type(&self) -> &'static str {
        "StructureMap"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Context => "context",
            Self::ContextQuantity => "context-quantity",
            Self::ContextType => "context-type",
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
            Self::ContextTypeQuantity => "context-type-quantity",
            Self::ContextTypeValue => "context-type-value",
        }
    }
}
/// Search parameters for the Subscription resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SubscriptionSearchParameter {
    /// Contact details for the subscription
    Contact,
    /// Content level included in notifications
    ContentLevel,
    /// Filter value used to narrow notifications
    FilterValue,
    /// A subscription identifier
    Identifier,
    /// A human-readable name
    Name,
    /// The managing entity
    Owner,
    /// The mime-type of notifications
    Payload,
    /// The current state of the subscription
    Status,
    /// The canonical topic url that triggers notifications
    Topic,
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
            Self::Contact => "contact",
            Self::ContentLevel => "content-level",
            Self::FilterValue => "filter-value",
            Self::Identifier => "identifier",
            Self::Name => "name",
            Self::Owner => "owner",
            Self::Payload => "payload",
            Self::Status => "status",
            Self::Topic => "topic",
            Self::Type => "type",
            Self::Url => "url",
        }
    }
}
/// Search parameters for the SubscriptionTopic resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SubscriptionTopicSearchParameter {
    /// Date status first applied
    Date,
    /// Business Identifier for SubscriptionTopic
    Identifier,
    /// draft | active | retired | unknown
    Status,
    /// Name for this SubscriptionTopic (Human friendly)
    Title,
    /// Logical canonical URL to reference this SubscriptionTopic (globally unique)
    Url,
    /// Business version of the SubscriptionTopic
    Version,
    /// A server defined search that matches either the url or derivedFrom
    DerivedOrSelf,
    /// Effective period
    Effective,
    /// Event trigger
    Event,
    /// Allowed resource for this definition
    Resource,
    /// Text representation of the trigger
    TriggerDescription,
}
impl ResourceSearchParameterDefinition for SubscriptionTopicSearchParameter {
    fn resource_type(&self) -> &'static str {
        "SubscriptionTopic"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Date => "date",
            Self::Identifier => "identifier",
            Self::Status => "status",
            Self::Title => "title",
            Self::Url => "url",
            Self::Version => "version",
            Self::DerivedOrSelf => "derived-or-self",
            Self::Effective => "effective",
            Self::Event => "event",
            Self::Resource => "resource",
            Self::TriggerDescription => "trigger-description",
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
    /// A reference to the defining substance
    CodeReference,
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
            Self::CodeReference => "code-reference",
            Self::Expiry => "expiry",
            Self::Identifier => "identifier",
            Self::Quantity => "quantity",
            Self::Status => "status",
            Self::SubstanceReference => "substance-reference",
        }
    }
}
/// Search parameters for the SubstanceDefinition resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum SubstanceDefinitionSearchParameter {
    /// High or low level categorization, e.g. polymer vs. nucleic acid or linear vs. branch chain
    Classification,
    /// The specific code
    Code,
    /// If the substance applies to only human or veterinary use
    Domain,
    /// Identifier by which this substance is known
    Identifier,
    /// The actual name
    Name,
}
impl ResourceSearchParameterDefinition for SubstanceDefinitionSearchParameter {
    fn resource_type(&self) -> &'static str {
        "SubstanceDefinition"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Classification => "classification",
            Self::Code => "code",
            Self::Domain => "domain",
            Self::Identifier => "identifier",
            Self::Name => "name",
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
    /// Business Identifier for SupplyRequest
    Identifier,
    /// The patient or subject for whom the supply is destined
    Patient,
    /// When the request was made
    Date,
    /// The kind of supply (central, non-stock, etc.)
    Category,
    /// Individual making the request
    Requester,
    /// draft | active | suspended +
    Status,
    /// The destination of the supply
    Subject,
    /// Who is intended to fulfill the request
    Supplier,
}
impl ResourceSearchParameterDefinition for SupplyRequestSearchParameter {
    fn resource_type(&self) -> &'static str {
        "SupplyRequest"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Patient => "patient",
            Self::Date => "date",
            Self::Category => "category",
            Self::Requester => "requester",
            Self::Status => "status",
            Self::Subject => "subject",
            Self::Supplier => "supplier",
        }
    }
}
/// Search parameters for the TerminologyCapabilities resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TerminologyCapabilitiesSearchParameter {
    /// A use context assigned to the terminology capabilities
    Context,
    /// A quantity- or range-valued use context assigned to the terminology capabilities
    ContextQuantity,
    /// A type of use context assigned to the terminology capabilities
    ContextType,
    /// The terminology capabilities publication date
    Date,
    /// The description of the terminology capabilities
    Description,
    /// External identifier for the terminology capabilities
    Identifier,
    /// Intended jurisdiction for the terminology capabilities
    Jurisdiction,
    /// Computationally friendly name of the terminology capabilities
    Name,
    /// Name of the publisher of the terminology capabilities
    Publisher,
    /// The current status of the terminology capabilities
    Status,
    /// The human-friendly name of the terminology capabilities
    Title,
    /// The uri that identifies the terminology capabilities
    Url,
    /// The business version of the terminology capabilities
    Version,
    /// A use context type and quantity- or range-based value assigned to the terminology capabilities
    ContextTypeQuantity,
    /// A use context type and value assigned to the terminology capabilities
    ContextTypeValue,
}
impl ResourceSearchParameterDefinition for TerminologyCapabilitiesSearchParameter {
    fn resource_type(&self) -> &'static str {
        "TerminologyCapabilities"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Context => "context",
            Self::ContextQuantity => "context-quantity",
            Self::ContextType => "context-type",
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
            Self::ContextTypeQuantity => "context-type-quantity",
            Self::ContextTypeValue => "context-type-value",
        }
    }
}
/// Search parameters for the TestPlan resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TestPlanSearchParameter {
    /// An identifier for the test plan
    Identifier,
    /// The current status of the test plan
    Status,
    /// The uri that identifies the test plan
    Url,
    /// The scope that is to be tested with this test plan
    Scope,
}
impl ResourceSearchParameterDefinition for TestPlanSearchParameter {
    fn resource_type(&self) -> &'static str {
        "TestPlan"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Status => "status",
            Self::Url => "url",
            Self::Scope => "scope",
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
    /// The current status of the test report
    Status,
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
            Self::Status => "status",
            Self::Tester => "tester",
            Self::Testscript => "testscript",
        }
    }
}
/// Search parameters for the TestScript resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TestScriptSearchParameter {
    /// A use context assigned to the test script
    Context,
    /// A quantity- or range-valued use context assigned to the test script
    ContextQuantity,
    /// A type of use context assigned to the test script
    ContextType,
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
    /// The human-friendly name of the test script
    Title,
    /// The uri that identifies the test script
    Url,
    /// The business version of the test script
    Version,
    /// A use context type and quantity- or range-based value assigned to the test script
    ContextTypeQuantity,
    /// A use context type and value assigned to the test script
    ContextTypeValue,
    /// The artifact under test
    ScopeArtifact,
    /// TestScript required and validated capability
    TestscriptCapability,
    /// The artifact under test and conformance testing expectation
    ScopeArtifactConformance,
    /// The artifact under test and phase of testing
    ScopeArtifactPhase,
}
impl ResourceSearchParameterDefinition for TestScriptSearchParameter {
    fn resource_type(&self) -> &'static str {
        "TestScript"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Context => "context",
            Self::ContextQuantity => "context-quantity",
            Self::ContextType => "context-type",
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
            Self::ContextTypeQuantity => "context-type-quantity",
            Self::ContextTypeValue => "context-type-value",
            Self::ScopeArtifact => "scope-artifact",
            Self::TestscriptCapability => "testscript-capability",
            Self::ScopeArtifactConformance => "scope-artifact-conformance",
            Self::ScopeArtifactPhase => "scope-artifact-phase",
        }
    }
}
/// Search parameters for the Transport resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TransportSearchParameter {
    /// External identifier
    Identifier,
    /// in-progress | completed | entered-in-error
    Status,
}
impl ResourceSearchParameterDefinition for TransportSearchParameter {
    fn resource_type(&self) -> &'static str {
        "Transport"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Identifier => "identifier",
            Self::Status => "status",
        }
    }
}
/// Search parameters for the ValueSet resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ValueSetSearchParameter {
    /// A use context assigned to the value set
    Context,
    /// A quantity- or range-valued use context assigned to the value set
    ContextQuantity,
    /// A type of use context assigned to the value set
    ContextType,
    /// The value set publication date
    Date,
    /// A resource that the ValueSet is derived from
    DerivedFrom,
    /// The description of the value set
    Description,
    /// The time during which the ValueSet is intended to be in use
    Effective,
    /// External identifier for the value set
    Identifier,
    /// Intended jurisdiction for the value set
    Jurisdiction,
    /// Computationally friendly name of the value set
    Name,
    /// The predecessor of the ValueSet
    Predecessor,
    /// Name of the publisher of the value set
    Publisher,
    /// The current status of the value set
    Status,
    /// The human-friendly name of the value set
    Title,
    /// Topics associated with the ValueSet
    Topic,
    /// The uri that identifies the value set
    Url,
    /// The business version of the value set
    Version,
    /// A use context type and quantity- or range-based value assigned to the value set
    ContextTypeQuantity,
    /// A use context type and value assigned to the value set
    ContextTypeValue,
    /// This special parameter searches for codes in the value set. See additional notes on the ValueSet resource
    Code,
    /// Identifies the value set expansion (business identifier)
    Expansion,
    /// A code system included or excluded in the value set or an imported value set
    Reference,
}
impl ResourceSearchParameterDefinition for ValueSetSearchParameter {
    fn resource_type(&self) -> &'static str {
        "ValueSet"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::Context => "context",
            Self::ContextQuantity => "context-quantity",
            Self::ContextType => "context-type",
            Self::Date => "date",
            Self::DerivedFrom => "derived-from",
            Self::Description => "description",
            Self::Effective => "effective",
            Self::Identifier => "identifier",
            Self::Jurisdiction => "jurisdiction",
            Self::Name => "name",
            Self::Predecessor => "predecessor",
            Self::Publisher => "publisher",
            Self::Status => "status",
            Self::Title => "title",
            Self::Topic => "topic",
            Self::Url => "url",
            Self::Version => "version",
            Self::ContextTypeQuantity => "context-type-quantity",
            Self::ContextTypeValue => "context-type-value",
            Self::Code => "code",
            Self::Expansion => "expansion",
            Self::Reference => "reference",
        }
    }
}
/// Search parameters for the VerificationResult resource
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum VerificationResultSearchParameter {
    /// Select validation information for data that was attested to using the specified method
    AttestationMethod,
    /// Select validation information for data that was attested to on behalf of the specified source
    AttestationOnbehalfof,
    /// Select validation information for data that was attested to by the specified source
    AttestationWho,
    /// Select validation information for data that was validated against a primary source on the specified date
    PrimarysourceDate,
    /// Select validation information for data that was validated against the specified type of primary source organization
    PrimarysourceType,
    /// Select validation information for data that was validated against the specified primary source
    PrimarysourceWho,
    /// Select validation information with the specified status
    Status,
    /// Select validation information with a validation status that occurred at the specified date/time
    StatusDate,
    /// A resource that was validated
    Target,
    /// Select validation information for data that was validated by the specified organization
    ValidatorOrganization,
}
impl ResourceSearchParameterDefinition for VerificationResultSearchParameter {
    fn resource_type(&self) -> &'static str {
        "VerificationResult"
    }
    fn code(&self) -> &'static str {
        match self {
            Self::AttestationMethod => "attestation-method",
            Self::AttestationOnbehalfof => "attestation-onbehalfof",
            Self::AttestationWho => "attestation-who",
            Self::PrimarysourceDate => "primarysource-date",
            Self::PrimarysourceType => "primarysource-type",
            Self::PrimarysourceWho => "primarysource-who",
            Self::Status => "status",
            Self::StatusDate => "status-date",
            Self::Target => "target",
            Self::ValidatorOrganization => "validator-organization",
        }
    }
}
impl SearchableResource for resources::ActivityDefinition {
    /// Parameters that can be used when searching ActivityDefinition resources
    type Params = ActivityDefinitionSearchParameter;
}
impl SearchableResource for resources::ActorDefinition {
    /// Parameters that can be used when searching ActorDefinition resources
    type Params = ActorDefinitionSearchParameter;
}
impl SearchableResource for resources::AdministrableProductDefinition {
    /// Parameters that can be used when searching AdministrableProductDefinition resources
    type Params = AdministrableProductDefinitionSearchParameter;
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
impl SearchableResource for resources::ArtifactAssessment {
    /// Parameters that can be used when searching ArtifactAssessment resources
    type Params = ArtifactAssessmentSearchParameter;
}
impl SearchableResource for resources::AuditEvent {
    /// Parameters that can be used when searching AuditEvent resources
    type Params = AuditEventSearchParameter;
}
impl SearchableResource for resources::BiologicallyDerivedProduct {
    /// Parameters that can be used when searching BiologicallyDerivedProduct resources
    type Params = BiologicallyDerivedProductSearchParameter;
}
impl SearchableResource for resources::BiologicallyDerivedProductDispense {
    /// Parameters that can be used when searching BiologicallyDerivedProductDispense resources
    type Params = BiologicallyDerivedProductDispenseSearchParameter;
}
impl SearchableResource for resources::BodyStructure {
    /// Parameters that can be used when searching BodyStructure resources
    type Params = BodyStructureSearchParameter;
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
impl SearchableResource for resources::ChargeItemDefinition {
    /// Parameters that can be used when searching ChargeItemDefinition resources
    type Params = ChargeItemDefinitionSearchParameter;
}
impl SearchableResource for resources::Citation {
    /// Parameters that can be used when searching Citation resources
    type Params = CitationSearchParameter;
}
impl SearchableResource for resources::ClinicalUseDefinition {
    /// Parameters that can be used when searching ClinicalUseDefinition resources
    type Params = ClinicalUseDefinitionSearchParameter;
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
impl SearchableResource for resources::ConditionDefinition {
    /// Parameters that can be used when searching ConditionDefinition resources
    type Params = ConditionDefinitionSearchParameter;
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
impl SearchableResource for resources::CoverageEligibilityRequest {
    /// Parameters that can be used when searching CoverageEligibilityRequest resources
    type Params = CoverageEligibilityRequestSearchParameter;
}
impl SearchableResource for resources::CoverageEligibilityResponse {
    /// Parameters that can be used when searching CoverageEligibilityResponse resources
    type Params = CoverageEligibilityResponseSearchParameter;
}
impl SearchableResource for resources::DetectedIssue {
    /// Parameters that can be used when searching DetectedIssue resources
    type Params = DetectedIssueSearchParameter;
}
impl SearchableResource for resources::Device {
    /// Parameters that can be used when searching Device resources
    type Params = DeviceSearchParameter;
}
impl SearchableResource for resources::DeviceAssociation {
    /// Parameters that can be used when searching DeviceAssociation resources
    type Params = DeviceAssociationSearchParameter;
}
impl SearchableResource for resources::DeviceDefinition {
    /// Parameters that can be used when searching DeviceDefinition resources
    type Params = DeviceDefinitionSearchParameter;
}
impl SearchableResource for resources::DeviceDispense {
    /// Parameters that can be used when searching DeviceDispense resources
    type Params = DeviceDispenseSearchParameter;
}
impl SearchableResource for resources::DeviceMetric {
    /// Parameters that can be used when searching DeviceMetric resources
    type Params = DeviceMetricSearchParameter;
}
impl SearchableResource for resources::DeviceRequest {
    /// Parameters that can be used when searching DeviceRequest resources
    type Params = DeviceRequestSearchParameter;
}
impl SearchableResource for resources::DeviceUsage {
    /// Parameters that can be used when searching DeviceUsage resources
    type Params = DeviceUsageSearchParameter;
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
impl SearchableResource for resources::EncounterHistory {
    /// Parameters that can be used when searching EncounterHistory resources
    type Params = EncounterHistorySearchParameter;
}
impl SearchableResource for resources::EpisodeOfCare {
    /// Parameters that can be used when searching EpisodeOfCare resources
    type Params = EpisodeOfCareSearchParameter;
}
impl SearchableResource for resources::EventDefinition {
    /// Parameters that can be used when searching EventDefinition resources
    type Params = EventDefinitionSearchParameter;
}
impl SearchableResource for resources::Evidence {
    /// Parameters that can be used when searching Evidence resources
    type Params = EvidenceSearchParameter;
}
impl SearchableResource for resources::EvidenceReport {
    /// Parameters that can be used when searching EvidenceReport resources
    type Params = EvidenceReportSearchParameter;
}
impl SearchableResource for resources::EvidenceVariable {
    /// Parameters that can be used when searching EvidenceVariable resources
    type Params = EvidenceVariableSearchParameter;
}
impl SearchableResource for resources::ExampleScenario {
    /// Parameters that can be used when searching ExampleScenario resources
    type Params = ExampleScenarioSearchParameter;
}
impl SearchableResource for resources::Flag {
    /// Parameters that can be used when searching Flag resources
    type Params = FlagSearchParameter;
}
impl SearchableResource for resources::FormularyItem {
    /// Parameters that can be used when searching FormularyItem resources
    type Params = FormularyItemSearchParameter;
}
impl SearchableResource for resources::GenomicStudy {
    /// Parameters that can be used when searching GenomicStudy resources
    type Params = GenomicStudySearchParameter;
}
impl SearchableResource for resources::GraphDefinition {
    /// Parameters that can be used when searching GraphDefinition resources
    type Params = GraphDefinitionSearchParameter;
}
impl SearchableResource for resources::GuidanceResponse {
    /// Parameters that can be used when searching GuidanceResponse resources
    type Params = GuidanceResponseSearchParameter;
}
impl SearchableResource for resources::ImagingSelection {
    /// Parameters that can be used when searching ImagingSelection resources
    type Params = ImagingSelectionSearchParameter;
}
impl SearchableResource for resources::Immunization {
    /// Parameters that can be used when searching Immunization resources
    type Params = ImmunizationSearchParameter;
}
impl SearchableResource for resources::ImmunizationEvaluation {
    /// Parameters that can be used when searching ImmunizationEvaluation resources
    type Params = ImmunizationEvaluationSearchParameter;
}
impl SearchableResource for resources::ImmunizationRecommendation {
    /// Parameters that can be used when searching ImmunizationRecommendation resources
    type Params = ImmunizationRecommendationSearchParameter;
}
impl SearchableResource for resources::ImplementationGuide {
    /// Parameters that can be used when searching ImplementationGuide resources
    type Params = ImplementationGuideSearchParameter;
}
impl SearchableResource for resources::Ingredient {
    /// Parameters that can be used when searching Ingredient resources
    type Params = IngredientSearchParameter;
}
impl SearchableResource for resources::InsurancePlan {
    /// Parameters that can be used when searching InsurancePlan resources
    type Params = InsurancePlanSearchParameter;
}
impl SearchableResource for resources::InventoryItem {
    /// Parameters that can be used when searching InventoryItem resources
    type Params = InventoryItemSearchParameter;
}
impl SearchableResource for resources::InventoryReport {
    /// Parameters that can be used when searching InventoryReport resources
    type Params = InventoryReportSearchParameter;
}
impl SearchableResource for resources::Invoice {
    /// Parameters that can be used when searching Invoice resources
    type Params = InvoiceSearchParameter;
}
impl SearchableResource for resources::Linkage {
    /// Parameters that can be used when searching Linkage resources
    type Params = LinkageSearchParameter;
}
impl SearchableResource for resources::Location {
    /// Parameters that can be used when searching Location resources
    type Params = LocationSearchParameter;
}
impl SearchableResource for resources::ManufacturedItemDefinition {
    /// Parameters that can be used when searching ManufacturedItemDefinition resources
    type Params = ManufacturedItemDefinitionSearchParameter;
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
impl SearchableResource for resources::MedicationKnowledge {
    /// Parameters that can be used when searching MedicationKnowledge resources
    type Params = MedicationKnowledgeSearchParameter;
}
impl SearchableResource for resources::MedicationRequest {
    /// Parameters that can be used when searching MedicationRequest resources
    type Params = MedicationRequestSearchParameter;
}
impl SearchableResource for resources::MedicationStatement {
    /// Parameters that can be used when searching MedicationStatement resources
    type Params = MedicationStatementSearchParameter;
}
impl SearchableResource for resources::MedicinalProductDefinition {
    /// Parameters that can be used when searching MedicinalProductDefinition resources
    type Params = MedicinalProductDefinitionSearchParameter;
}
impl SearchableResource for resources::MessageDefinition {
    /// Parameters that can be used when searching MessageDefinition resources
    type Params = MessageDefinitionSearchParameter;
}
impl SearchableResource for resources::MessageHeader {
    /// Parameters that can be used when searching MessageHeader resources
    type Params = MessageHeaderSearchParameter;
}
impl SearchableResource for resources::MolecularSequence {
    /// Parameters that can be used when searching MolecularSequence resources
    type Params = MolecularSequenceSearchParameter;
}
impl SearchableResource for resources::NamingSystem {
    /// Parameters that can be used when searching NamingSystem resources
    type Params = NamingSystemSearchParameter;
}
impl SearchableResource for resources::NutritionIntake {
    /// Parameters that can be used when searching NutritionIntake resources
    type Params = NutritionIntakeSearchParameter;
}
impl SearchableResource for resources::NutritionOrder {
    /// Parameters that can be used when searching NutritionOrder resources
    type Params = NutritionOrderSearchParameter;
}
impl SearchableResource for resources::NutritionProduct {
    /// Parameters that can be used when searching NutritionProduct resources
    type Params = NutritionProductSearchParameter;
}
impl SearchableResource for resources::Observation {
    /// Parameters that can be used when searching Observation resources
    type Params = ObservationSearchParameter;
}
impl SearchableResource for resources::ObservationDefinition {
    /// Parameters that can be used when searching ObservationDefinition resources
    type Params = ObservationDefinitionSearchParameter;
}
impl SearchableResource for resources::OperationDefinition {
    /// Parameters that can be used when searching OperationDefinition resources
    type Params = OperationDefinitionSearchParameter;
}
impl SearchableResource for resources::Organization {
    /// Parameters that can be used when searching Organization resources
    type Params = OrganizationSearchParameter;
}
impl SearchableResource for resources::OrganizationAffiliation {
    /// Parameters that can be used when searching OrganizationAffiliation resources
    type Params = OrganizationAffiliationSearchParameter;
}
impl SearchableResource for resources::PackagedProductDefinition {
    /// Parameters that can be used when searching PackagedProductDefinition resources
    type Params = PackagedProductDefinitionSearchParameter;
}
impl SearchableResource for resources::Patient {
    /// Parameters that can be used when searching Patient resources
    type Params = PatientSearchParameter;
}
impl SearchableResource for resources::Permission {
    /// Parameters that can be used when searching Permission resources
    type Params = PermissionSearchParameter;
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
impl SearchableResource for resources::RegulatedAuthorization {
    /// Parameters that can be used when searching RegulatedAuthorization resources
    type Params = RegulatedAuthorizationSearchParameter;
}
impl SearchableResource for resources::RelatedPerson {
    /// Parameters that can be used when searching RelatedPerson resources
    type Params = RelatedPersonSearchParameter;
}
impl SearchableResource for resources::RequestOrchestration {
    /// Parameters that can be used when searching RequestOrchestration resources
    type Params = RequestOrchestrationSearchParameter;
}
impl SearchableResource for resources::Requirements {
    /// Parameters that can be used when searching Requirements resources
    type Params = RequirementsSearchParameter;
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
impl SearchableResource for resources::ServiceRequest {
    /// Parameters that can be used when searching ServiceRequest resources
    type Params = ServiceRequestSearchParameter;
}
impl SearchableResource for resources::Specimen {
    /// Parameters that can be used when searching Specimen resources
    type Params = SpecimenSearchParameter;
}
impl SearchableResource for resources::SpecimenDefinition {
    /// Parameters that can be used when searching SpecimenDefinition resources
    type Params = SpecimenDefinitionSearchParameter;
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
impl SearchableResource for resources::SubscriptionTopic {
    /// Parameters that can be used when searching SubscriptionTopic resources
    type Params = SubscriptionTopicSearchParameter;
}
impl SearchableResource for resources::Substance {
    /// Parameters that can be used when searching Substance resources
    type Params = SubstanceSearchParameter;
}
impl SearchableResource for resources::SubstanceDefinition {
    /// Parameters that can be used when searching SubstanceDefinition resources
    type Params = SubstanceDefinitionSearchParameter;
}
impl SearchableResource for resources::SupplyDelivery {
    /// Parameters that can be used when searching SupplyDelivery resources
    type Params = SupplyDeliverySearchParameter;
}
impl SearchableResource for resources::SupplyRequest {
    /// Parameters that can be used when searching SupplyRequest resources
    type Params = SupplyRequestSearchParameter;
}
impl SearchableResource for resources::TerminologyCapabilities {
    /// Parameters that can be used when searching TerminologyCapabilities resources
    type Params = TerminologyCapabilitiesSearchParameter;
}
impl SearchableResource for resources::TestPlan {
    /// Parameters that can be used when searching TestPlan resources
    type Params = TestPlanSearchParameter;
}
impl SearchableResource for resources::TestReport {
    /// Parameters that can be used when searching TestReport resources
    type Params = TestReportSearchParameter;
}
impl SearchableResource for resources::TestScript {
    /// Parameters that can be used when searching TestScript resources
    type Params = TestScriptSearchParameter;
}
impl SearchableResource for resources::Transport {
    /// Parameters that can be used when searching Transport resources
    type Params = TransportSearchParameter;
}
impl SearchableResource for resources::ValueSet {
    /// Parameters that can be used when searching ValueSet resources
    type Params = ValueSetSearchParameter;
}
impl SearchableResource for resources::VerificationResult {
    /// Parameters that can be used when searching VerificationResult resources
    type Params = VerificationResultSearchParameter;
}
