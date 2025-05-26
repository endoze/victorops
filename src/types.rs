use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Details about an HTTP request made to the VictorOps API.
#[derive(Debug, Clone)]
pub struct RequestDetails {
  /// The HTTP status code of the response.
  pub status_code: u16,
  /// The response body as a string.
  pub response_body: String,
  /// The request body that was sent.
  pub request_body: String,
}

/// A paged entity containing basic name and slug information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagedEntity {
  /// The name of the entity.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  /// The slug identifier of the entity.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub slug: Option<String>,
}

/// A paged policy containing policy and team information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagedPolicy {
  /// The policy entity information.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub policy: Option<PagedEntity>,
  /// The team entity information.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub team: Option<PagedEntity>,
}

/// Represents a state transition in an incident.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transition {
  /// The name of the transition.
  #[serde(skip_serializing_if = "Option::is_none", rename = "Name")]
  pub name: Option<String>,
  /// When the transition occurred.
  #[serde(skip_serializing_if = "Option::is_none", rename = "At")]
  pub at: Option<DateTime<Utc>>,
  /// Message associated with the transition.
  #[serde(skip_serializing_if = "Option::is_none", rename = "Message")]
  pub message: Option<String>,
  /// Who performed the transition.
  #[serde(skip_serializing_if = "Option::is_none", rename = "By")]
  pub by: Option<String>,
  /// Whether the transition was performed manually.
  #[serde(skip_serializing_if = "Option::is_none", rename = "Manually")]
  pub manually: Option<bool>,
  /// The ID of the alert that triggered this transition.
  #[serde(skip_serializing_if = "Option::is_none", rename = "alertId")]
  pub alert_id: Option<String>,
  /// The URL of the alert that triggered this transition.
  #[serde(skip_serializing_if = "Option::is_none", rename = "alertUrl")]
  pub alert_url: Option<String>,
}

/// Represents an incident in VictorOps.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Incident {
  /// The number of alerts in this incident.
  #[serde(skip_serializing_if = "Option::is_none", rename = "alertCount")]
  pub alert_count: Option<i32>,
  /// The current phase or state of the incident.
  #[serde(skip_serializing_if = "Option::is_none", rename = "currentPhase")]
  pub current_phase: Option<String>,
  /// The display name of the entity that triggered the incident.
  #[serde(skip_serializing_if = "Option::is_none", rename = "entityDisplayName")]
  pub entity_display_name: Option<String>,
  /// The unique identifier of the entity that triggered the incident.
  #[serde(skip_serializing_if = "Option::is_none", rename = "entityId")]
  pub entity_id: Option<String>,
  /// The state of the entity that triggered the incident.
  #[serde(skip_serializing_if = "Option::is_none", rename = "entityState")]
  pub entity_state: Option<String>,
  /// The type of entity that triggered the incident.
  #[serde(skip_serializing_if = "Option::is_none", rename = "entityType")]
  pub entity_type: Option<String>,
  /// The host associated with the incident.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub host: Option<String>,
  /// The incident number or identifier.
  #[serde(skip_serializing_if = "Option::is_none", rename = "incidentNumber")]
  pub incident_number: Option<String>,
  /// The ID of the last alert in this incident.
  #[serde(skip_serializing_if = "Option::is_none", rename = "lastAlertId")]
  pub last_alert_id: Option<String>,
  /// The timestamp of the last alert in this incident.
  #[serde(skip_serializing_if = "Option::is_none", rename = "lastAlertTime")]
  pub last_alert_time: Option<DateTime<Utc>>,
  /// The service associated with the incident.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub service: Option<String>,
  /// The timestamp when the incident started.
  #[serde(skip_serializing_if = "Option::is_none", rename = "startTime")]
  pub start_time: Option<DateTime<Utc>>,
  /// The list of teams that were paged for this incident.
  #[serde(skip_serializing_if = "Option::is_none", rename = "pagedTeams")]
  pub paged_teams: Option<Vec<String>>,
  /// The list of users that were paged for this incident.
  #[serde(skip_serializing_if = "Option::is_none", rename = "pagedUsers")]
  pub paged_users: Option<Vec<String>>,
  /// The list of escalation policies that were triggered for this incident.
  #[serde(skip_serializing_if = "Option::is_none", rename = "pagedPolicies")]
  pub paged_policies: Option<Vec<PagedPolicy>>,
  /// The state transitions that occurred during this incident.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub transitions: Option<Vec<Transition>>,
}

/// Response containing a list of incidents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentResponse {
  /// The list of incidents in the response.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub incidents: Option<Vec<Incident>>,
}

/// Represents a user in VictorOps.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
  /// The first name of the user.
  #[serde(skip_serializing_if = "Option::is_none", rename = "firstName")]
  pub first_name: Option<String>,
  /// The last name of the user.
  #[serde(skip_serializing_if = "Option::is_none", rename = "lastName")]
  pub last_name: Option<String>,
  /// The username of the user.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub username: Option<String>,
  /// The email address of the user.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub email: Option<String>,
  /// Whether the user has admin privileges.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub admin: Option<bool>,
  /// The number of hours until the user's session expires.
  #[serde(skip_serializing_if = "Option::is_none", rename = "expirationHours")]
  pub expiration_hours: Option<i32>,
  /// The timestamp when the user was created.
  #[serde(skip_serializing_if = "Option::is_none", rename = "createdAt")]
  pub created_at: Option<String>,
  /// The timestamp when the user's password was last updated.
  #[serde(
    skip_serializing_if = "Option::is_none",
    rename = "passwordLastUpdated"
  )]
  pub password_last_updated: Option<String>,
  /// Whether the user's account has been verified.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub verified: Option<bool>,
}

/// Response containing a list of users (v1 API format).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserList {
  /// The nested list of users in v1 API format.
  pub users: Vec<Vec<User>>,
}

/// Response containing a list of users (v2 API format).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserListV2 {
  /// The list of users in v2 API format.
  pub users: Vec<User>,
}

/// Represents a team in VictorOps.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
  /// The name of the team.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  /// The unique slug identifier for the team.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub slug: Option<String>,
  /// The number of members in the team.
  #[serde(skip_serializing_if = "Option::is_none", rename = "memberCount")]
  pub member_count: Option<i32>,
  /// The version of the team configuration.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub version: Option<i32>,
  /// Whether this is the default team for the organization.
  #[serde(skip_serializing_if = "Option::is_none", rename = "isDefaultTeam")]
  pub is_default_team: Option<bool>,
}

/// Response containing team members.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMembers {
  /// The list of team members.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub members: Option<Vec<User>>,
}

/// Represents an admin user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Admin {
  /// The username of the admin.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub username: Option<String>,
  /// The first name of the admin.
  #[serde(skip_serializing_if = "Option::is_none", rename = "firstName")]
  pub first_name: Option<String>,
  /// The last name of the admin.
  #[serde(skip_serializing_if = "Option::is_none", rename = "lastName")]
  pub last_name: Option<String>,
  /// The URL to the admin's profile.
  #[serde(skip_serializing_if = "Option::is_none", rename = "_selfUrl")]
  pub self_url: Option<String>,
}

/// Response containing team administrators.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamAdmins {
  /// The list of team administrators.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub admin: Option<Vec<Admin>>,
}

/// Represents a contact method.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactMethod {
  /// The unique identifier of the contact method.
  pub id: f64,
  /// The label or name of the contact method.
  pub label: String,
}

/// Response containing email contact methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailsResponse {
  /// The list of email contact methods.
  #[serde(rename = "contactMethods")]
  pub contact_methods: Vec<serde_json::Value>,
}

/// Represents a team in API responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiTeam {
  /// The name of the team.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  /// The unique slug identifier for the team.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub slug: Option<String>,
}

/// Represents an escalation policy in API responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiEscalationPolicy {
  /// The name of the escalation policy.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub name: Option<String>,
  /// The unique slug identifier for the escalation policy.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub slug: Option<String>,
}

/// Represents a user in API responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiUser {
  /// The username of the user.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub username: Option<String>,
}

/// Represents an on-call override.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiOnCallOverride {
  /// The original user who was scheduled to be on-call.
  #[serde(skip_serializing_if = "Option::is_none", rename = "origOnCallUser")]
  pub orig_on_call_user: Option<ApiUser>,
  /// The user who is overriding the original on-call assignment.
  #[serde(skip_serializing_if = "Option::is_none", rename = "overrideOnCallUser")]
  pub override_on_call_user: Option<ApiUser>,
  /// The start time of the on-call override.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub start: Option<DateTime<Utc>>,
  /// The end time of the on-call override.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub end: Option<DateTime<Utc>>,
  /// The escalation policy associated with this override.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub policy: Option<ApiEscalationPolicy>,
}

/// Represents an on-call roll/rotation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiOnCallRoll {
  /// The start time of the on-call period.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub start: Option<DateTime<Utc>>,
  /// The end time of the on-call period.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub end: Option<DateTime<Utc>>,
  /// The user who is on-call during this period.
  #[serde(skip_serializing_if = "Option::is_none", rename = "onCallUser")]
  pub on_call_user: Option<ApiUser>,
  /// Whether this is a roll/rotation period.
  #[serde(skip_serializing_if = "Option::is_none", rename = "isRoll")]
  pub is_roll: Option<bool>,
}

/// Represents an on-call schedule entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiOnCallEntry {
  /// The user who is scheduled to be on-call.
  #[serde(skip_serializing_if = "Option::is_none", rename = "onCallUser")]
  pub on_call_user: Option<ApiUser>,
  /// The user who is overriding the scheduled on-call user.
  #[serde(skip_serializing_if = "Option::is_none", rename = "overrideOnCallUser")]
  pub override_on_call_user: Option<ApiUser>,
  /// The type of on-call assignment.
  #[serde(skip_serializing_if = "Option::is_none", rename = "onCallType")]
  pub on_call_type: Option<String>,
  /// The name of the rotation this entry belongs to.
  #[serde(skip_serializing_if = "Option::is_none", rename = "rotationName")]
  pub rotation_name: Option<String>,
  /// The name of the shift this entry belongs to.
  #[serde(skip_serializing_if = "Option::is_none", rename = "shiftName")]
  pub shift_name: Option<String>,
  /// The timestamp when the shift roll occurs.
  #[serde(skip_serializing_if = "Option::is_none", rename = "shiftRoll")]
  pub shift_roll: Option<DateTime<Utc>>,
  /// The list of rolls/rotations for this entry.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub rolls: Option<Vec<ApiOnCallRoll>>,
}

/// Represents an escalation policy schedule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiEscalationPolicySchedule {
  /// The escalation policy this schedule belongs to.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub policy: Option<ApiEscalationPolicy>,
  /// The schedule entries for this escalation policy.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub schedule: Option<Vec<ApiOnCallEntry>>,
  /// The on-call overrides for this escalation policy.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub overrides: Option<Vec<ApiOnCallOverride>>,
}

/// Represents a team's on-call schedule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiTeamSchedule {
  /// The team this schedule belongs to.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub team: Option<ApiTeam>,
  /// The escalation policy schedules for this team.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub schedules: Option<Vec<ApiEscalationPolicySchedule>>,
}

/// Represents a user's on-call schedule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiUserSchedule {
  /// The team schedules for this user.
  #[serde(skip_serializing_if = "Option::is_none", rename = "teamSchedules")]
  pub schedules: Option<Vec<ApiTeamSchedule>>,
}

/// Request to take on-call duty.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TakeRequest {
  /// The user who is giving up on-call duty.
  #[serde(skip_serializing_if = "Option::is_none", rename = "fromUser")]
  pub from_user: Option<String>,
  /// The user who is taking on-call duty.
  #[serde(skip_serializing_if = "Option::is_none", rename = "toUser")]
  pub to_user: Option<String>,
}

/// Response from taking on-call duty.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TakeResponse {
  /// The result of the take request.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub result: Option<String>,
}

/// Represents an entry in an escalation policy step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationPolicyStepEntry {
  /// The type of execution for this escalation step.
  #[serde(skip_serializing_if = "Option::is_none", rename = "executionType")]
  pub execution_type: Option<String>,
  /// User information for user-based escalation targets.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user: Option<std::collections::HashMap<String, String>>,
  /// Rotation group information for rotation-based escalation targets.
  #[serde(skip_serializing_if = "Option::is_none", rename = "rotationGroup")]
  pub rotation_group: Option<std::collections::HashMap<String, String>>,
  /// Webhook information for webhook-based escalation targets.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub webhook: Option<std::collections::HashMap<String, String>>,
  /// Email information for email-based escalation targets.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub email: Option<std::collections::HashMap<String, String>>,
  /// Target policy information for policy-based escalation targets.
  #[serde(skip_serializing_if = "Option::is_none", rename = "targetPolicy")]
  pub target_policy: Option<std::collections::HashMap<String, String>>,
}

/// Represents a step in an escalation policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationPolicySteps {
  /// The timeout in seconds before escalating to the next step.
  pub timeout: i32,
  /// The list of entries/targets for this escalation step.
  pub entries: Vec<EscalationPolicyStepEntry>,
}

/// Represents an escalation policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationPolicy {
  /// The name of the escalation policy.
  pub name: String,
  /// The team slug/ID that this escalation policy belongs to.
  #[serde(rename = "teamSlug")]
  pub team_id: String,
  /// Whether to ignore custom paging policies.
  #[serde(rename = "ignoreCustomPagingPolicies")]
  pub ignore_custom_paging_policies: bool,
  /// The escalation steps for this policy.
  pub steps: Vec<EscalationPolicySteps>,
  /// The unique slug/ID of the escalation policy.
  #[serde(rename = "slug")]
  pub id: String,
}

/// Represents escalation policy details in a list response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationPolicyListDetail {
  /// The name of the escalation policy.
  pub name: String,
  /// The unique slug identifier of the escalation policy.
  pub slug: String,
}

/// Represents an element in an escalation policy list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationPolicyListElement {
  /// The escalation policy details.
  pub policy: EscalationPolicyListDetail,
  /// The team details associated with this escalation policy.
  pub team: EscalationPolicyListDetail,
}

/// Response containing a list of escalation policies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationPolicyList {
  /// The list of escalation policies.
  pub policies: Vec<EscalationPolicyListElement>,
}

/// Represents a routing key for directing alerts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingKey {
  /// The routing key value used to route alerts.
  #[serde(skip_serializing_if = "Option::is_none", rename = "routingKey")]
  pub routing_key: Option<String>,
  /// The list of targets that this routing key routes to.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub targets: Option<Vec<String>>,
}

/// Represents targets in a routing key response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingKeyResponseTargets {
  /// The slug of the escalation policy this routing key targets.
  #[serde(skip_serializing_if = "Option::is_none", rename = "policySlug")]
  pub policy_slug: Option<String>,
}

/// Response containing routing key information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingKeyResponse {
  /// The routing key value.
  #[serde(skip_serializing_if = "Option::is_none", rename = "routingKey")]
  pub routing_key: Option<String>,
  /// The targets that this routing key routes to.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub targets: Option<Vec<RoutingKeyResponseTargets>>,
}

/// Response containing a list of routing keys.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingKeyResponseList {
  /// The list of routing keys.
  #[serde(skip_serializing_if = "Option::is_none", rename = "routingKeys")]
  pub routing_keys: Option<Vec<RoutingKeyResponse>>,
}

/// Types of contact methods available in VictorOps.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContactType {
  /// Phone or SMS contact method.
  Phone,
  /// Email contact method.
  Email,
  /// Mobile device push notification contact method.
  Device,
}

impl ContactType {
  /// Returns the endpoint noun for this contact type.
  pub fn endpoint_noun(&self) -> &'static str {
    match self {
      ContactType::Phone => "phones",
      ContactType::Email => "emails",
      ContactType::Device => "devices",
    }
  }

  /// Creates a ContactType from a notification type string.
  pub fn from_notification_type(notification_type: &str) -> Option<Self> {
    match notification_type {
      "push" => Some(ContactType::Device),
      "email" => Some(ContactType::Email),
      "phone" | "sms" => Some(ContactType::Phone),
      _ => None,
    }
  }
}

/// Represents a contact method for a user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
  /// The phone number for phone-based contact methods.
  #[serde(skip_serializing_if = "Option::is_none", rename = "phone")]
  pub phone_number: Option<String>,
  /// The email address for email-based contact methods.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub email: Option<String>,
  /// The label or description of this contact method.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<String>,
  /// The priority rank of this contact method.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub rank: Option<i32>,
  /// The external ID of this contact method.
  #[serde(skip_serializing_if = "Option::is_none", rename = "extId")]
  pub ext_id: Option<String>,
  /// The unique identifier of this contact method.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<i32>,
  /// The value of this contact method.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub value: Option<String>,
  /// The verification status of this contact method.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub verified: Option<String>,
}

impl Contact {
  /// Determines the contact type based on the contact's fields.
  pub fn contact_type(&self) -> Option<ContactType> {
    if self.phone_number.is_some() {
      Some(ContactType::Phone)
    } else if self.email.is_some() {
      Some(ContactType::Email)
    } else {
      None
    }
  }
}

/// A group of contact methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactGroup {
  /// The list of contact methods in this group.
  #[serde(rename = "contactMethods")]
  pub contact_methods: Vec<Contact>,
}

/// Response containing all contact methods for a user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllContactResponse {
  /// The phone contact methods for the user.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub phones: Option<ContactGroup>,
  /// The email contact methods for the user.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub emails: Option<ContactGroup>,
  /// The device contact methods for the user.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub devices: Option<ContactGroup>,
}

/// Response for getting all contacts of a specific type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAllContactResponse {
  /// The list of contact methods of the requested type.
  #[serde(skip_serializing_if = "Option::is_none", rename = "contactMethods")]
  pub contact_methods: Option<Vec<Contact>>,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_contact_type_endpoint_noun() {
    assert_eq!(ContactType::Phone.endpoint_noun(), "phones");
    assert_eq!(ContactType::Email.endpoint_noun(), "emails");
    assert_eq!(ContactType::Device.endpoint_noun(), "devices");
  }

  #[test]
  fn test_contact_type_from_notification_type() {
    assert_eq!(
      ContactType::from_notification_type("push"),
      Some(ContactType::Device)
    );
    assert_eq!(
      ContactType::from_notification_type("email"),
      Some(ContactType::Email)
    );
    assert_eq!(
      ContactType::from_notification_type("phone"),
      Some(ContactType::Phone)
    );
    assert_eq!(
      ContactType::from_notification_type("sms"),
      Some(ContactType::Phone)
    );
    assert_eq!(ContactType::from_notification_type("unknown"), None);
  }

  #[test]
  fn test_contact_contact_type() {
    let phone_contact = Contact {
      phone_number: Some("555-1234".to_string()),
      email: None,
      label: Some("Primary".to_string()),
      rank: Some(1),
      ext_id: None,
      id: None,
      value: None,
      verified: None,
    };
    assert_eq!(phone_contact.contact_type(), Some(ContactType::Phone));

    let email_contact = Contact {
      phone_number: None,
      email: Some("test@example.com".to_string()),
      label: Some("Work".to_string()),
      rank: Some(1),
      ext_id: None,
      id: None,
      value: None,
      verified: None,
    };
    assert_eq!(email_contact.contact_type(), Some(ContactType::Email));

    let empty_contact = Contact {
      phone_number: None,
      email: None,
      label: None,
      rank: None,
      ext_id: None,
      id: None,
      value: None,
      verified: None,
    };
    assert_eq!(empty_contact.contact_type(), None);
  }

  #[test]
  fn test_contact_serialization() {
    let contact = Contact {
      phone_number: Some("555-1234".to_string()),
      email: Some("test@example.com".to_string()),
      label: Some("Primary".to_string()),
      rank: Some(1),
      ext_id: Some("ext123".to_string()),
      id: Some(42),
      value: Some("contact-value".to_string()),
      verified: Some("true".to_string()),
    };

    let json = serde_json::to_string(&contact).unwrap();
    let deserialized: Contact = serde_json::from_str(&json).unwrap();
    
    assert_eq!(deserialized.phone_number, contact.phone_number);
    assert_eq!(deserialized.email, contact.email);
    assert_eq!(deserialized.label, contact.label);
    assert_eq!(deserialized.rank, contact.rank);
  }

  #[test]
  fn test_contact_group_serialization() {
    let contact_group = ContactGroup {
      contact_methods: vec![
        Contact {
          phone_number: Some("555-1234".to_string()),
          email: None,
          label: Some("Primary".to_string()),
          rank: Some(1),
          ext_id: None,
          id: None,
          value: None,
          verified: None,
        }
      ],
    };

    let json = serde_json::to_string(&contact_group).unwrap();
    assert!(json.contains("contactMethods"));
    
    let deserialized: ContactGroup = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.contact_methods.len(), 1);
  }
}
