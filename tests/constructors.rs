use std::collections::HashMap;
use victorops::{
  Contact, ContactType, EscalationPolicy, EscalationPolicyStepEntry, EscalationPolicySteps,
  RoutingKey, TakeRequest, Team, User,
};

#[test]
fn user_from_default() {
  let mut user = User::default();
  user.username = Some("jdoe".to_string());
  user.email = Some("jdoe@example.com".to_string());

  assert_eq!(
    serde_json::to_string(&user).unwrap(),
    r#"{"username":"jdoe","email":"jdoe@example.com"}"#
  );
}

#[test]
fn team_from_default() {
  let mut team = Team::default();
  team.name = Some("Ops".to_string());

  assert_eq!(serde_json::to_string(&team).unwrap(), r#"{"name":"Ops"}"#);
}

#[test]
fn contact_constructors() {
  let phone = Contact::phone("Mobile", "+15555550100");
  let email = Contact::email("Work", "jdoe@example.com");
  let mut empty = Contact::default();
  empty.rank = Some(1);

  assert_eq!(phone.contact_type(), Some(ContactType::Phone));
  assert_eq!(email.contact_type(), Some(ContactType::Email));
  assert_eq!(empty.contact_type(), None);
  assert_eq!(
    serde_json::to_string(&phone).unwrap(),
    r#"{"phone":"+15555550100","label":"Mobile"}"#
  );
  assert_eq!(
    serde_json::to_string(&email).unwrap(),
    r#"{"email":"jdoe@example.com","label":"Work"}"#
  );
}

#[test]
fn routing_key_from_default() {
  let mut key = RoutingKey::default();
  key.routing_key = Some("db-alerts".to_string());
  key.targets = vec!["pol-primary".to_string()];

  assert_eq!(
    serde_json::to_string(&key).unwrap(),
    r#"{"routingKey":"db-alerts","targets":["pol-primary"]}"#
  );
}

#[test]
fn take_request_constructors() {
  assert_eq!(
    serde_json::to_string(&TakeRequest::new("jdoe", "asmith")).unwrap(),
    r#"{"fromUser":"jdoe","toUser":"asmith"}"#
  );
  assert_eq!(
    serde_json::to_string(&TakeRequest::default()).unwrap(),
    "{}"
  );
}

#[test]
fn escalation_policy_constructors() {
  let mut entry = EscalationPolicyStepEntry::default();
  entry.execution_type = Some("user".to_string());
  entry.user = Some(HashMap::from([(
    "username".to_string(),
    "jdoe".to_string(),
  )]));

  let mut policy = EscalationPolicy::new(
    "Primary",
    "team-ops",
    vec![EscalationPolicySteps::new(300, vec![entry])],
  );

  assert_eq!(
    serde_json::to_string(&policy).unwrap(),
    r#"{"name":"Primary","teamSlug":"team-ops","ignoreCustomPagingPolicies":false,"steps":[{"timeout":300,"entries":[{"executionType":"user","user":{"username":"jdoe"}}]}],"slug":""}"#
  );

  policy.ignore_custom_paging_policies = true;
  policy.id = "pol-primary".to_string();
  assert_eq!(policy.id, "pol-primary");
}
