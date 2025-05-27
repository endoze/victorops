use crate::error::{ApiResult, Error};
use crate::types::*;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;

/// HTTP client for interacting with the VictorOps API.
///
/// The Client provides methods for making authenticated requests to the VictorOps API,
/// including operations for incidents, users, teams, escalation policies, and more.
#[derive(Debug, Clone)]
pub struct Client {
  pub(crate) pub_base_url: String,
  pub(crate) api_id: String,
  pub(crate) api_key: String,
  http_client: reqwest::Client,
}

impl Client {
  /// Creates a new VictorOps API client with default timeout (30 seconds).
  ///
  /// # Arguments
  ///
  /// * `api_id` - The VictorOps API ID for authentication
  /// * `api_key` - The VictorOps API key for authentication
  /// * `pub_base_url` - The base URL for the VictorOps API
  ///
  /// # Examples
  ///
  /// ```
  /// use victorops::Client;
  ///
  /// let client = Client::new(
  ///     "your-api-id".to_string(),
  ///     "your-api-key".to_string(),
  ///     "https://api.victorops.com".to_string(),
  /// )?;
  /// # Ok::<(), victorops::Error>(())
  /// ```
  pub fn new(api_id: String, api_key: String, pub_base_url: String) -> ApiResult<Self> {
    let http_client = reqwest::Client::builder()
      .timeout(Duration::from_secs(30))
      .build()?;

    Ok(Client {
      api_id,
      api_key,
      pub_base_url,
      http_client,
    })
  }

  /// Creates a new VictorOps API client with a custom timeout.
  ///
  /// # Arguments
  ///
  /// * `api_id` - The VictorOps API ID for authentication
  /// * `api_key` - The VictorOps API key for authentication
  /// * `pub_base_url` - The base URL for the VictorOps API
  /// * `timeout` - Custom timeout duration for HTTP requests
  ///
  /// # Examples
  ///
  /// ```
  /// use victorops::Client;
  /// use std::time::Duration;
  ///
  /// let client = Client::with_timeout(
  ///     "your-api-id".to_string(),
  ///     "your-api-key".to_string(),
  ///     "https://api.victorops.com".to_string(),
  ///     Duration::from_secs(60),
  /// )?;
  /// # Ok::<(), victorops::Error>(())
  /// ```
  pub fn with_timeout(
    api_id: String,
    api_key: String,
    pub_base_url: String,
    timeout: Duration,
  ) -> ApiResult<Self> {
    let http_client = reqwest::Client::builder().timeout(timeout).build()?;

    Ok(Client {
      api_id,
      api_key,
      pub_base_url,
      http_client,
    })
  }

  async fn make_public_api_call(
    &self,
    method: reqwest::Method,
    endpoint: &str,
    body: Option<Value>,
    query_params: Option<HashMap<String, String>>,
  ) -> ApiResult<RequestDetails> {
    let url = format!("{}/api-public/{}", self.pub_base_url, endpoint);
    let mut request_builder = self.http_client.request(method, &url);

    let mut headers = HeaderMap::new();
    headers.insert("X-VO-Api-Id", HeaderValue::from_str(&self.api_id)?);
    headers.insert("X-VO-Api-Key", HeaderValue::from_str(&self.api_key)?);
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    request_builder = request_builder.headers(headers);

    if let Some(params) = query_params {
      request_builder = request_builder.query(&params);
    }

    let request_body = if let Some(body) = body {
      let body_str = serde_json::to_string(&body)?;
      request_builder = request_builder.body(body_str.clone());
      body_str
    } else {
      "{}".to_string()
    };

    let response = request_builder.send().await?;
    let status_code = response.status().as_u16();
    let response_body = response.text().await?;

    if status_code >= 400 {
      return Err(Error::Api {
        status: status_code,
        message: response_body.clone(),
      });
    }

    Ok(RequestDetails {
      status_code,
      response_body,
      request_body,
    })
  }

  /// Retrieves a specific incident by ID.
  ///
  /// # Arguments
  ///
  /// * `incident_id` - The ID of the incident to retrieve
  ///
  /// # Returns
  ///
  /// A tuple containing the incident data and request details.
  pub async fn get_incident(&self, incident_id: i32) -> ApiResult<(Incident, RequestDetails)> {
    let details = self
      .make_public_api_call(
        reqwest::Method::GET,
        &format!("v1/incidents/{}", incident_id),
        None,
        None,
      )
      .await?;

    let incident: Incident = serde_json::from_str(&details.response_body)?;
    Ok((incident, details))
  }

  /// Retrieves all incidents.
  ///
  /// # Returns
  ///
  /// A tuple containing the list of incidents and request details.
  pub async fn get_incidents(&self) -> ApiResult<(IncidentResponse, RequestDetails)> {
    let details = self
      .make_public_api_call(reqwest::Method::GET, "v1/incidents", None, None)
      .await?;

    let incidents: IncidentResponse = serde_json::from_str(&details.response_body)?;
    Ok((incidents, details))
  }

  /// Creates a new user in VictorOps.
  ///
  /// # Arguments
  ///
  /// * `user` - The user data to create
  ///
  /// # Returns
  ///
  /// A tuple containing the created user data and request details.
  pub async fn create_user(&self, user: &User) -> ApiResult<(User, RequestDetails)> {
    let body = serde_json::to_value(user)?;
    let details = self
      .make_public_api_call(reqwest::Method::POST, "v1/user", Some(body), None)
      .await?;

    let new_user: User = serde_json::from_str(&details.response_body)?;
    Ok((new_user, details))
  }

  /// Retrieves a specific user by username.
  ///
  /// # Arguments
  ///
  /// * `username` - The username of the user to retrieve
  ///
  /// # Returns
  ///
  /// A tuple containing the user data and request details.
  pub async fn get_user(&self, username: &str) -> ApiResult<(User, RequestDetails)> {
    let encoded_username =
      url::form_urlencoded::byte_serialize(username.as_bytes()).collect::<String>();
    let details = self
      .make_public_api_call(
        reqwest::Method::GET,
        &format!("v1/user/{}", encoded_username),
        None,
        None,
      )
      .await?;

    let user: User = serde_json::from_str(&details.response_body)?;
    Ok((user, details))
  }

  /// Deletes a user from VictorOps.
  ///
  /// # Arguments
  ///
  /// * `username` - The username of the user to delete
  /// * `replacement_user` - The username of the user to replace the deleted user in schedules
  ///
  /// # Returns
  ///
  /// Request details for the delete operation.
  pub async fn delete_user(
    &self,
    username: &str,
    replacement_user: &str,
  ) -> ApiResult<RequestDetails> {
    let encoded_username =
      url::form_urlencoded::byte_serialize(username.as_bytes()).collect::<String>();
    let body = serde_json::json!({ "replacement": replacement_user });

    self
      .make_public_api_call(
        reqwest::Method::DELETE,
        &format!("v1/user/{}", encoded_username),
        Some(body),
        None,
      )
      .await
  }

  /// Retrieves all users (v1 API).
  ///
  /// # Returns
  ///
  /// A tuple containing the list of users and request details.
  pub async fn get_all_users(&self) -> ApiResult<(UserList, RequestDetails)> {
    let details = self
      .make_public_api_call(reqwest::Method::GET, "v1/user", None, None)
      .await?;

    let user_list: UserList = serde_json::from_str(&details.response_body)?;
    Ok((user_list, details))
  }

  /// Retrieves all users (v2 API).
  ///
  /// # Returns
  ///
  /// A tuple containing the list of users and request details.
  pub async fn get_all_users_v2(&self) -> ApiResult<(UserListV2, RequestDetails)> {
    let details = self
      .make_public_api_call(reqwest::Method::GET, "v2/user", None, None)
      .await?;

    let user_list: UserListV2 = serde_json::from_str(&details.response_body)?;
    Ok((user_list, details))
  }

  /// Retrieves users by email address.
  ///
  /// # Arguments
  ///
  /// * `email` - The email address to search for
  ///
  /// # Returns
  ///
  /// A tuple containing the list of matching users and request details.
  pub async fn get_user_by_email(&self, email: &str) -> ApiResult<(UserListV2, RequestDetails)> {
    let mut params = HashMap::new();
    params.insert("email".to_string(), email.to_string());

    let details = self
      .make_public_api_call(reqwest::Method::GET, "v2/user", None, Some(params))
      .await?;

    let user_list: UserListV2 = serde_json::from_str(&details.response_body)?;
    Ok((user_list, details))
  }

  /// Updates an existing user.
  ///
  /// # Arguments
  ///
  /// * `user` - The user data with updates (must include username)
  ///
  /// # Returns
  ///
  /// A tuple containing the updated user data and request details.
  pub async fn update_user(&self, user: &User) -> ApiResult<(User, RequestDetails)> {
    let username = user
      .username
      .as_ref()
      .ok_or_else(|| Error::InvalidInput("Username is required for user update".to_string()))?;

    let encoded_username =
      url::form_urlencoded::byte_serialize(username.as_bytes()).collect::<String>();
    let body = serde_json::to_value(user)?;

    let details = self
      .make_public_api_call(
        reqwest::Method::PUT,
        &format!("v1/user/{}", encoded_username),
        Some(body),
        None,
      )
      .await?;

    let updated_user: User = serde_json::from_str(&details.response_body)?;
    Ok((updated_user, details))
  }

  /// Retrieves the default email contact ID for a user.
  ///
  /// # Arguments
  ///
  /// * `username` - The username to get the default email contact for
  ///
  /// # Returns
  ///
  /// A tuple containing the contact ID and request details.
  pub async fn get_user_default_email_contact_id(
    &self,
    username: &str,
  ) -> ApiResult<(f64, RequestDetails)> {
    let encoded_username =
      url::form_urlencoded::byte_serialize(username.as_bytes()).collect::<String>();
    let details = self
      .make_public_api_call(
        reqwest::Method::GET,
        &format!("v1/user/{}/contact-methods/emails", encoded_username),
        None,
        None,
      )
      .await?;

    let emails_response: EmailsResponse = serde_json::from_str(&details.response_body)?;

    for contact_method in &emails_response.contact_methods {
      if let Some(label) = contact_method.get("label") {
        if label.as_str() == Some("Default") {
          if let Some(id) = contact_method.get("id") {
            if let Some(id_num) = id.as_f64() {
              return Ok((id_num, details));
            }
          }
        }
      }
    }

    Err(Error::NotFound)
  }

  /// Creates a new team in VictorOps.
  ///
  /// # Arguments
  ///
  /// * `team` - The team data to create
  ///
  /// # Returns
  ///
  /// A tuple containing the created team data and request details.
  pub async fn create_team(&self, team: &Team) -> ApiResult<(Team, RequestDetails)> {
    let body = serde_json::to_value(team)?;
    let details = self
      .make_public_api_call(reqwest::Method::POST, "v1/team", Some(body), None)
      .await?;

    let new_team: Team = serde_json::from_str(&details.response_body)?;
    Ok((new_team, details))
  }

  /// Retrieves a specific team by ID.
  ///
  /// # Arguments
  ///
  /// * `team_id` - The ID of the team to retrieve
  ///
  /// # Returns
  ///
  /// A tuple containing the team data and request details.
  pub async fn get_team(&self, team_id: &str) -> ApiResult<(Team, RequestDetails)> {
    let details = self
      .make_public_api_call(
        reqwest::Method::GET,
        &format!("v1/team/{}", team_id),
        None,
        None,
      )
      .await?;

    let team: Team = serde_json::from_str(&details.response_body)?;
    Ok((team, details))
  }

  /// Retrieves all teams.
  ///
  /// # Returns
  ///
  /// A tuple containing the list of teams and request details.
  pub async fn get_all_teams(&self) -> ApiResult<(Vec<Team>, RequestDetails)> {
    let details = self
      .make_public_api_call(reqwest::Method::GET, "v1/team", None, None)
      .await?;

    let teams: Vec<Team> = serde_json::from_str(&details.response_body)?;
    Ok((teams, details))
  }

  /// Retrieves all members of a specific team.
  ///
  /// # Arguments
  ///
  /// * `team_id` - The ID of the team
  ///
  /// # Returns
  ///
  /// A tuple containing the team members and request details.
  pub async fn get_team_members(&self, team_id: &str) -> ApiResult<(TeamMembers, RequestDetails)> {
    let details = self
      .make_public_api_call(
        reqwest::Method::GET,
        &format!("v1/team/{}/members", team_id),
        None,
        None,
      )
      .await?;

    let team_members: TeamMembers = serde_json::from_str(&details.response_body)?;
    Ok((team_members, details))
  }

  /// Deletes a team from VictorOps.
  ///
  /// # Arguments
  ///
  /// * `team_id` - The ID of the team to delete
  ///
  /// # Returns
  ///
  /// Request details for the delete operation.
  pub async fn delete_team(&self, team_id: &str) -> ApiResult<RequestDetails> {
    self
      .make_public_api_call(
        reqwest::Method::DELETE,
        &format!("v1/team/{}", team_id),
        None,
        None,
      )
      .await
  }

  /// Updates an existing team.
  ///
  /// # Arguments
  ///
  /// * `team` - The team data with updates (must include name)
  ///
  /// # Returns
  ///
  /// A tuple containing the updated team data and request details.
  pub async fn update_team(&self, team: &Team) -> ApiResult<(Team, RequestDetails)> {
    let team_name = team
      .name
      .as_ref()
      .ok_or_else(|| Error::InvalidInput("Team name is required for team update".to_string()))?;

    let body = serde_json::to_value(team)?;
    let details = self
      .make_public_api_call(
        reqwest::Method::PUT,
        &format!("v1/team/{}", team_name),
        Some(body),
        None,
      )
      .await?;

    let updated_team: Team = serde_json::from_str(&details.response_body)?;
    Ok((updated_team, details))
  }

  /// Adds a user to a team.
  ///
  /// # Arguments
  ///
  /// * `team_id` - The ID of the team
  /// * `username` - The username of the user to add
  ///
  /// # Returns
  ///
  /// Request details for the add operation.
  pub async fn add_team_member(&self, team_id: &str, username: &str) -> ApiResult<RequestDetails> {
    let body = serde_json::json!({ "username": username });

    self
      .make_public_api_call(
        reqwest::Method::POST,
        &format!("v1/team/{}/members", team_id),
        Some(body),
        None,
      )
      .await
  }

  /// Removes a user from a team.
  ///
  /// # Arguments
  ///
  /// * `team_id` - The ID of the team
  /// * `username` - The username of the user to remove
  /// * `replacement` - The username of the replacement user for schedules
  ///
  /// # Returns
  ///
  /// Request details for the remove operation.
  pub async fn remove_team_member(
    &self,
    team_id: &str,
    username: &str,
    replacement: &str,
  ) -> ApiResult<RequestDetails> {
    let encoded_username =
      url::form_urlencoded::byte_serialize(username.as_bytes()).collect::<String>();
    let body = serde_json::json!({ "replacement": replacement });

    self
      .make_public_api_call(
        reqwest::Method::DELETE,
        &format!("v1/team/{}/members/{}", team_id, encoded_username),
        Some(body),
        None,
      )
      .await
  }

  /// Checks if a user is a member of a team.
  ///
  /// # Arguments
  ///
  /// * `team_id` - The ID of the team
  /// * `username` - The username to check
  ///
  /// # Returns
  ///
  /// A tuple containing whether the user is a member and request details.
  pub async fn is_team_member(
    &self,
    team_id: &str,
    username: &str,
  ) -> ApiResult<(bool, RequestDetails)> {
    let (members, details) = self.get_team_members(team_id).await?;

    if !members.members.is_empty() {
      for member in &members.members {
        if let Some(member_username) = &member.username {
          if member_username.to_lowercase() == username.to_lowercase() {
            return Ok((true, details));
          }
        }
      }
    }

    Ok((false, details))
  }

  /// Retrieves all administrators of a specific team.
  ///
  /// # Arguments
  ///
  /// * `team_id` - The ID of the team
  ///
  /// # Returns
  ///
  /// A tuple containing the team administrators and request details.
  pub async fn get_team_admins(&self, team_id: &str) -> ApiResult<(TeamAdmins, RequestDetails)> {
    let details = self
      .make_public_api_call(
        reqwest::Method::GET,
        &format!("v1/team/{}/admins", team_id),
        None,
        None,
      )
      .await?;

    let team_admins: TeamAdmins = serde_json::from_str(&details.response_body)?;
    Ok((team_admins, details))
  }

  /// Retrieves the on-call schedule for a team.
  ///
  /// # Arguments
  ///
  /// * `team_slug` - The slug of the team
  /// * `days_forward` - Number of days forward to retrieve
  /// * `days_skip` - Number of days to skip from today
  /// * `step` - Step interval for the schedule
  ///
  /// # Returns
  ///
  /// A tuple containing the team schedule and request details.
  pub async fn get_api_team_schedule(
    &self,
    team_slug: &str,
    days_forward: i32,
    days_skip: i32,
    step: i32,
  ) -> ApiResult<(ApiTeamSchedule, RequestDetails)> {
    let details = self
      .make_public_api_call(
        reqwest::Method::GET,
        &format!(
          "v2/team/{}/oncall/schedule?daysForward={}&daysSkip={}&step={}",
          team_slug, days_forward, days_skip, step
        ),
        None,
        None,
      )
      .await?;

    let schedule: ApiTeamSchedule = serde_json::from_str(&details.response_body)?;
    Ok((schedule, details))
  }

  /// Retrieves the on-call schedule for a specific user.
  ///
  /// # Arguments
  ///
  /// * `username` - The username to get the schedule for
  /// * `days_forward` - Number of days forward to retrieve
  /// * `days_skip` - Number of days to skip from today
  /// * `step` - Step interval for the schedule
  ///
  /// # Returns
  ///
  /// A tuple containing the user schedule and request details.
  pub async fn get_user_on_call_schedule(
    &self,
    username: &str,
    days_forward: i32,
    days_skip: i32,
    step: i32,
  ) -> ApiResult<(ApiUserSchedule, RequestDetails)> {
    let details = self
      .make_public_api_call(
        reqwest::Method::GET,
        &format!(
          "v2/user/{}/oncall/schedule?daysForward={}&daysSkip={}&step={}",
          username, days_forward, days_skip, step
        ),
        None,
        None,
      )
      .await?;

    let schedule: ApiUserSchedule = serde_json::from_str(&details.response_body)?;
    Ok((schedule, details))
  }

  /// Takes on-call duty for a team.
  ///
  /// # Arguments
  ///
  /// * `team_slug` - The slug of the team
  /// * `request` - The take request details
  ///
  /// # Returns
  ///
  /// A tuple containing the take response and request details.
  pub async fn take_on_call_for_team(
    &self,
    team_slug: &str,
    request: &TakeRequest,
  ) -> ApiResult<(TakeResponse, RequestDetails)> {
    let body = serde_json::to_value(request)?;
    let details = self
      .make_public_api_call(
        reqwest::Method::PATCH,
        &format!("v1/team/{}/oncall/user", team_slug),
        Some(body),
        None,
      )
      .await?;

    let take_response: TakeResponse = serde_json::from_str(&details.response_body)?;
    Ok((take_response, details))
  }

  /// Takes on-call duty for a specific escalation policy.
  ///
  /// # Arguments
  ///
  /// * `policy_slug` - The slug of the escalation policy
  /// * `request` - The take request details
  ///
  /// # Returns
  ///
  /// A tuple containing the take response and request details.
  pub async fn take_on_call_for_policy(
    &self,
    policy_slug: &str,
    request: &TakeRequest,
  ) -> ApiResult<(TakeResponse, RequestDetails)> {
    let body = serde_json::to_value(request)?;
    let details = self
      .make_public_api_call(
        reqwest::Method::PATCH,
        &format!("v1/policies/{}/oncall/user", policy_slug),
        Some(body),
        None,
      )
      .await?;

    let take_response: TakeResponse = serde_json::from_str(&details.response_body)?;
    Ok((take_response, details))
  }

  /// Creates a new escalation policy.
  ///
  /// # Arguments
  ///
  /// * `escalation_policy` - The escalation policy data to create
  ///
  /// # Returns
  ///
  /// A tuple containing the created escalation policy and request details.
  pub async fn create_escalation_policy(
    &self,
    escalation_policy: &EscalationPolicy,
  ) -> ApiResult<(EscalationPolicy, RequestDetails)> {
    let body = serde_json::to_value(escalation_policy)?;
    let details = self
      .make_public_api_call(reqwest::Method::POST, "v1/policies", Some(body), None)
      .await?;

    let new_policy: EscalationPolicy = serde_json::from_str(&details.response_body)?;
    Ok((new_policy, details))
  }

  /// Retrieves all escalation policies.
  ///
  /// # Returns
  ///
  /// A tuple containing the list of escalation policies and request details.
  pub async fn get_all_escalation_policies(
    &self,
  ) -> ApiResult<(EscalationPolicyList, RequestDetails)> {
    let details = self
      .make_public_api_call(reqwest::Method::GET, "v1/policies", None, None)
      .await?;

    let policy_list: EscalationPolicyList = serde_json::from_str(&details.response_body)?;
    Ok((policy_list, details))
  }

  /// Retrieves a specific escalation policy by ID.
  ///
  /// # Arguments
  ///
  /// * `escalation_policy_id` - The ID of the escalation policy to retrieve
  ///
  /// # Returns
  ///
  /// A tuple containing the escalation policy and request details.
  pub async fn get_escalation_policy(
    &self,
    escalation_policy_id: &str,
  ) -> ApiResult<(EscalationPolicy, RequestDetails)> {
    let details = self
      .make_public_api_call(
        reqwest::Method::GET,
        &format!("v1/policies/{}", escalation_policy_id),
        None,
        None,
      )
      .await?;

    let policy: EscalationPolicy = serde_json::from_str(&details.response_body)?;
    Ok((policy, details))
  }

  /// Deletes an escalation policy.
  ///
  /// # Arguments
  ///
  /// * `escalation_policy_id` - The ID of the escalation policy to delete
  ///
  /// # Returns
  ///
  /// Request details for the delete operation.
  pub async fn delete_escalation_policy(
    &self,
    escalation_policy_id: &str,
  ) -> ApiResult<RequestDetails> {
    self
      .make_public_api_call(
        reqwest::Method::DELETE,
        &format!("v1/policies/{}", escalation_policy_id),
        None,
        None,
      )
      .await
  }

  /// Creates a new routing key.
  ///
  /// # Arguments
  ///
  /// * `routing_key` - The routing key data to create
  ///
  /// # Returns
  ///
  /// A tuple containing the created routing key and request details.
  pub async fn create_routing_key(
    &self,
    routing_key: &RoutingKey,
  ) -> ApiResult<(RoutingKey, RequestDetails)> {
    let body = serde_json::to_value(routing_key)?;
    let details = self
      .make_public_api_call(
        reqwest::Method::POST,
        "v1/org/routing-keys",
        Some(body),
        None,
      )
      .await?;

    let new_key: RoutingKey = serde_json::from_str(&details.response_body)?;
    Ok((new_key, details))
  }

  /// Retrieves a specific routing key by name.
  ///
  /// # Arguments
  ///
  /// * `key_name` - The name of the routing key to retrieve
  ///
  /// # Returns
  ///
  /// A tuple containing the optional routing key and request details.
  pub async fn get_routing_key(
    &self,
    key_name: &str,
  ) -> ApiResult<(Option<RoutingKeyResponse>, RequestDetails)> {
    let (rk_list, details) = self.get_all_routing_keys().await?;

    if !rk_list.routing_keys.is_empty() {
      for key in &rk_list.routing_keys {
        if let Some(routing_key) = &key.routing_key {
          if routing_key == key_name {
            return Ok((Some(key.clone()), details));
          }
        }
      }
    }

    Ok((None, details))
  }

  /// Retrieves all routing keys.
  ///
  /// # Returns
  ///
  /// A tuple containing the list of routing keys and request details.
  pub async fn get_all_routing_keys(&self) -> ApiResult<(RoutingKeyResponseList, RequestDetails)> {
    let details = self
      .make_public_api_call(reqwest::Method::GET, "v1/org/routing-keys", None, None)
      .await?;

    let rk_list: RoutingKeyResponseList = serde_json::from_str(&details.response_body)?;
    Ok((rk_list, details))
  }

  /// Creates a new contact method for a user.
  ///
  /// # Arguments
  ///
  /// * `username` - The username to create the contact for
  /// * `contact` - The contact data to create
  ///
  /// # Returns
  ///
  /// A tuple containing the created contact and request details.
  pub async fn create_contact(
    &self,
    username: &str,
    contact: &Contact,
  ) -> ApiResult<(Contact, RequestDetails)> {
    let contact_type = contact.contact_type().ok_or_else(|| {
      Error::InvalidInput("Contact must have either phone_number or email".to_string())
    })?;

    let encoded_username =
      url::form_urlencoded::byte_serialize(username.as_bytes()).collect::<String>();
    let body = serde_json::to_value(contact)?;

    let details = self
      .make_public_api_call(
        reqwest::Method::POST,
        &format!(
          "v1/user/{}/contact-methods/{}",
          encoded_username,
          contact_type.endpoint_noun()
        ),
        Some(body),
        None,
      )
      .await?;

    let new_contact: Contact = serde_json::from_str(&details.response_body)?;
    Ok((new_contact, details))
  }

  /// Retrieves a specific contact method for a user.
  ///
  /// # Arguments
  ///
  /// * `username` - The username to get the contact for
  /// * `contact_ext_id` - The external ID of the contact
  /// * `contact_type` - The type of contact (Email, Phone, Device)
  ///
  /// # Returns
  ///
  /// A tuple containing the contact and request details.
  pub async fn get_contact(
    &self,
    username: &str,
    contact_ext_id: &str,
    contact_type: ContactType,
  ) -> ApiResult<(Contact, RequestDetails)> {
    let encoded_username =
      url::form_urlencoded::byte_serialize(username.as_bytes()).collect::<String>();
    let details = self
      .make_public_api_call(
        reqwest::Method::GET,
        &format!(
          "v1/user/{}/contact-methods/{}/{}",
          encoded_username,
          contact_type.endpoint_noun(),
          contact_ext_id
        ),
        None,
        None,
      )
      .await?;

    let contact: Contact = serde_json::from_str(&details.response_body)?;
    Ok((contact, details))
  }

  /// Retrieves all contact methods for a user.
  ///
  /// # Arguments
  ///
  /// * `username` - The username to get contacts for
  ///
  /// # Returns
  ///
  /// A tuple containing all contact methods and request details.
  pub async fn get_all_contacts(
    &self,
    username: &str,
  ) -> ApiResult<(AllContactResponse, RequestDetails)> {
    let encoded_username =
      url::form_urlencoded::byte_serialize(username.as_bytes()).collect::<String>();
    let details = self
      .make_public_api_call(
        reqwest::Method::GET,
        &format!("v1/user/{}/contact-methods", encoded_username),
        None,
        None,
      )
      .await?;

    let all_contacts: AllContactResponse = serde_json::from_str(&details.response_body)?;
    Ok((all_contacts, details))
  }

  /// Deletes a contact method for a user.
  ///
  /// # Arguments
  ///
  /// * `username` - The username to delete the contact from
  /// * `contact_ext_id` - The external ID of the contact to delete
  /// * `contact_type` - The type of contact (Email, Phone, Device)
  ///
  /// # Returns
  ///
  /// Request details for the delete operation.
  pub async fn delete_contact(
    &self,
    username: &str,
    contact_ext_id: &str,
    contact_type: ContactType,
  ) -> ApiResult<RequestDetails> {
    let encoded_username =
      url::form_urlencoded::byte_serialize(username.as_bytes()).collect::<String>();

    self
      .make_public_api_call(
        reqwest::Method::DELETE,
        &format!(
          "v1/user/{}/contact-methods/{}/{}",
          encoded_username,
          contact_type.endpoint_noun(),
          contact_ext_id
        ),
        None,
        None,
      )
      .await
  }

  /// Retrieves a contact method by its ID.
  ///
  /// # Arguments
  ///
  /// * `username` - The username to get the contact for
  /// * `id` - The ID of the contact
  /// * `contact_type` - The type of contact (Email, Phone, Device)
  ///
  /// # Returns
  ///
  /// A tuple containing the optional contact and request details.
  pub async fn get_contact_by_id(
    &self,
    username: &str,
    id: i32,
    contact_type: ContactType,
  ) -> ApiResult<(Option<Contact>, RequestDetails)> {
    if contact_type == ContactType::Device && id == 0 {
      let contact = Contact {
        phone_number: None,
        email: None,
        label: Some("All Devices".to_string()),
        rank: Some(0),
        ext_id: None,
        id: Some(0),
        value: Some("All Devices".to_string()),
        verified: None,
      };
      return Ok((
        Some(contact),
        RequestDetails {
          status_code: 200,
          response_body: "".to_string(),
          request_body: "".to_string(),
        },
      ));
    }

    let encoded_username =
      url::form_urlencoded::byte_serialize(username.as_bytes()).collect::<String>();
    let details = self
      .make_public_api_call(
        reqwest::Method::GET,
        &format!(
          "v1/user/{}/contact-methods/{}",
          encoded_username,
          contact_type.endpoint_noun()
        ),
        None,
        None,
      )
      .await?;

    let contacts: GetAllContactResponse = serde_json::from_str(&details.response_body)?;

    if !contacts.contact_methods.is_empty() {
      for contact in &contacts.contact_methods {
        if let Some(contact_id) = contact.id {
          if contact_id == id {
            return Ok((Some(contact.clone()), details));
          }
        }
      }
    }

    Ok((None, details))
  }
}

impl std::fmt::Display for Client {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "VictorOps Client: publicBaseURL: {}", self.pub_base_url)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::time::Duration;

  fn create_test_client() -> Client {
    Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      "https://api.victorops.com".to_string(),
    )
    .unwrap()
  }

  #[test]
  fn test_client_creation() {
    let client = create_test_client();
    assert_eq!(client.pub_base_url, "https://api.victorops.com");
    assert_eq!(client.api_id, "test-api-id");
    assert_eq!(client.api_key, "test-api-key");
  }

  #[test]
  fn test_client_with_timeout() {
    let client = Client::with_timeout(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      "https://api.victorops.com".to_string(),
      Duration::from_secs(60),
    )
    .unwrap();

    assert_eq!(client.pub_base_url, "https://api.victorops.com");
  }

  #[test]
  fn test_client_display() {
    let client = create_test_client();
    let display_string = format!("{}", client);
    assert!(display_string.contains("VictorOps Client"));
    assert!(display_string.contains("https://api.victorops.com"));
  }

  #[test]
  fn test_get_contact_by_id_special_device() {
    use tokio::runtime::Runtime;
    let rt = Runtime::new().unwrap();
    let client = create_test_client();

    let result = rt.block_on(async {
      client
        .get_contact_by_id("testuser", 0, ContactType::Device)
        .await
    });

    assert!(result.is_ok());
    let (contact_opt, _details) = result.unwrap();
    assert!(contact_opt.is_some());
    let contact = contact_opt.unwrap();
    assert_eq!(contact.id, Some(0));
    assert_eq!(contact.value, Some("All Devices".to_string()));
    assert_eq!(contact.label, Some("All Devices".to_string()));
  }

  #[tokio::test]
  async fn test_get_incident_success() {
    let mut server = mockito::Server::new_async().await;
    let mock_response = r#"{
      "alertCount": 5,
      "currentPhase": "UNACKED",
      "entityDisplayName": "Test Incident",
      "entityId": "test-entity-123",
      "entityState": "CRITICAL"
    }"#;

    let _mock = server
      .mock("GET", "/api-public/v1/incidents/123")
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(mock_response)
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let result = client.get_incident(123).await;
    assert!(result.is_ok());

    let (incident, details) = result.unwrap();
    assert_eq!(incident.alert_count, Some(5));
    assert_eq!(incident.current_phase, Some("UNACKED".to_string()));
    assert_eq!(details.status_code, 200);
  }

  #[tokio::test]
  async fn test_get_incident_not_found() {
    let mut server = mockito::Server::new_async().await;

    let _mock = server
      .mock("GET", "/api-public/v1/incidents/999")
      .with_status(404)
      .with_body("Incident not found")
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let result = client.get_incident(999).await;
    assert!(result.is_err());

    if let Err(crate::Error::Api { status, message }) = result {
      assert_eq!(status, 404);
      assert_eq!(message, "Incident not found");
    } else {
      panic!("Expected API error");
    }
  }

  #[tokio::test]
  async fn test_get_incidents_success() {
    let mut server = mockito::Server::new_async().await;
    let mock_response = r#"{
      "incidents": [
        {
          "alertCount": 2,
          "currentPhase": "ACKED",
          "entityDisplayName": "Incident 1"
        },
        {
          "alertCount": 3,
          "currentPhase": "UNACKED",
          "entityDisplayName": "Incident 2"
        }
      ]
    }"#;

    let _mock = server
      .mock("GET", "/api-public/v1/incidents")
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(mock_response)
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let result = client.get_incidents().await;
    assert!(result.is_ok());

    let (incident_response, details) = result.unwrap();
    assert!(!incident_response.incidents.is_empty());
    let incidents = &incident_response.incidents;
    assert_eq!(incidents.len(), 2);
    assert_eq!(details.status_code, 200);
  }

  #[tokio::test]
  async fn test_create_user_success() {
    let mut server = mockito::Server::new_async().await;
    let mock_response = r#"{
      "firstName": "John",
      "lastName": "Doe",
      "username": "jdoe",
      "email": "john.doe@example.com",
      "admin": false
    }"#;

    let _mock = server
      .mock("POST", "/api-public/v1/user")
      .with_status(201)
      .with_header("content-type", "application/json")
      .with_body(mock_response)
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let user = crate::types::User {
      first_name: Some("John".to_string()),
      last_name: Some("Doe".to_string()),
      username: Some("jdoe".to_string()),
      email: Some("john.doe@example.com".to_string()),
      admin: Some(false),
      expiration_hours: None,
      created_at: None,
      password_last_updated: None,
      verified: None,
    };

    let result = client.create_user(&user).await;
    assert!(result.is_ok());

    let (created_user, details) = result.unwrap();
    assert_eq!(created_user.username, Some("jdoe".to_string()));
    assert_eq!(created_user.email, Some("john.doe@example.com".to_string()));
    assert_eq!(details.status_code, 201);
  }

  #[tokio::test]
  async fn test_get_user_with_url_encoding() {
    let mut server = mockito::Server::new_async().await;
    let mock_response = r#"{
      "firstName": "Test",
      "lastName": "User",
      "username": "test@example.com",
      "email": "test@example.com"
    }"#;

    let _mock = server
      .mock("GET", "/api-public/v1/user/test%40example.com")
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(mock_response)
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let result = client.get_user("test@example.com").await;
    assert!(result.is_ok());

    let (user, details) = result.unwrap();
    assert_eq!(user.username, Some("test@example.com".to_string()));
    assert_eq!(details.status_code, 200);
  }

  #[tokio::test]
  async fn test_api_headers_are_set() {
    let mut server = mockito::Server::new_async().await;

    let _mock = server
      .mock("GET", "/api-public/v1/incidents/123")
      .match_header("X-VO-Api-Id", "test-api-id")
      .match_header("X-VO-Api-Key", "test-api-key")
      .match_header("Content-Type", "application/json")
      .with_status(200)
      .with_body("{}")
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let _result = client.get_incident(123).await;
  }

  #[tokio::test]
  async fn test_api_error_handling() {
    let mut server = mockito::Server::new_async().await;

    let _mock = server
      .mock("GET", "/api-public/v1/incidents/123")
      .with_status(500)
      .with_body("Internal Server Error")
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let result = client.get_incident(123).await;
    assert!(result.is_err());

    if let Err(crate::Error::Api { status, message }) = result {
      assert_eq!(status, 500);
      assert_eq!(message, "Internal Server Error");
    } else {
      panic!("Expected API error, got: {:?}", result);
    }
  }

  #[tokio::test]
  async fn test_delete_user_success() {
    let mut server = mockito::Server::new_async().await;

    let _mock = server
      .mock("DELETE", "/api-public/v1/user/testuser")
      .with_status(200)
      .with_body("User deleted successfully")
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let result = client.delete_user("testuser", "replacement_user").await;
    assert!(result.is_ok());

    let details = result.unwrap();
    assert_eq!(details.status_code, 200);
    assert_eq!(details.response_body, "User deleted successfully");
  }

  #[tokio::test]
  async fn test_get_all_users_success() {
    let mut server = mockito::Server::new_async().await;
    let mock_response = r#"{
      "users": [[
        {
          "firstName": "John",
          "lastName": "Doe",
          "username": "jdoe",
          "email": "john@example.com"
        }
      ]]
    }"#;

    let _mock = server
      .mock("GET", "/api-public/v1/user")
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(mock_response)
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let result = client.get_all_users().await;
    assert!(result.is_ok());

    let (user_list, details) = result.unwrap();
    assert!(!user_list.users.is_empty());
    assert_eq!(details.status_code, 200);
  }

  #[tokio::test]
  async fn test_get_all_users_v2_success() {
    let mut server = mockito::Server::new_async().await;
    let mock_response = r#"{
      "users": [
        {
          "firstName": "John",
          "lastName": "Doe",
          "username": "jdoe",
          "email": "john@example.com"
        }
      ]
    }"#;

    let _mock = server
      .mock("GET", "/api-public/v2/user")
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(mock_response)
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let result = client.get_all_users_v2().await;
    assert!(result.is_ok());

    let (user_list, details) = result.unwrap();
    assert!(!user_list.users.is_empty());
    assert_eq!(details.status_code, 200);
  }

  #[tokio::test]
  async fn test_get_user_by_email_success() {
    let mut server = mockito::Server::new_async().await;
    let mock_response = r#"{
      "users": [
        {
          "firstName": "Jane",
          "lastName": "Smith",
          "username": "jsmith",
          "email": "jane@example.com"
        }
      ]
    }"#;

    let _mock = server
      .mock("GET", "/api-public/v2/user")
      .match_query(mockito::Matcher::UrlEncoded(
        "email".into(),
        "jane@example.com".into(),
      ))
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(mock_response)
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let result = client.get_user_by_email("jane@example.com").await;
    assert!(result.is_ok());

    let (user_list, details) = result.unwrap();
    assert!(!user_list.users.is_empty());
    let user = &user_list.users[0];
    assert_eq!(user.email, Some("jane@example.com".to_string()));
    assert_eq!(details.status_code, 200);
  }

  #[tokio::test]
  async fn test_update_user_success() {
    let mut server = mockito::Server::new_async().await;
    let mock_response = r#"{
      "firstName": "John",
      "lastName": "Updated",
      "username": "jdoe",
      "email": "john.updated@example.com"
    }"#;

    let _mock = server
      .mock("PUT", "/api-public/v1/user/jdoe")
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(mock_response)
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let user = crate::types::User {
      first_name: Some("John".to_string()),
      last_name: Some("Updated".to_string()),
      username: Some("jdoe".to_string()),
      email: Some("john.updated@example.com".to_string()),
      admin: Some(false),
      expiration_hours: None,
      created_at: None,
      password_last_updated: None,
      verified: None,
    };

    let result = client.update_user(&user).await;
    assert!(result.is_ok());

    let (updated_user, details) = result.unwrap();
    assert_eq!(updated_user.last_name, Some("Updated".to_string()));
    assert_eq!(details.status_code, 200);
  }

  #[tokio::test]
  async fn test_create_team_success() {
    let mut server = mockito::Server::new_async().await;
    let mock_response = r#"{
      "name": "Test Team",
      "slug": "test-team",
      "memberCount": 0,
      "version": 1
    }"#;

    let _mock = server
      .mock("POST", "/api-public/v1/team")
      .with_status(201)
      .with_header("content-type", "application/json")
      .with_body(mock_response)
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let team = crate::types::Team {
      name: Some("Test Team".to_string()),
      slug: Some("test-team".to_string()),
      member_count: Some(0),
      version: Some(1),
      is_default_team: Some(false),
    };

    let result = client.create_team(&team).await;
    assert!(result.is_ok());

    let (created_team, details) = result.unwrap();
    assert_eq!(created_team.name, Some("Test Team".to_string()));
    assert_eq!(details.status_code, 201);
  }

  #[tokio::test]
  async fn test_get_team_success() {
    let mut server = mockito::Server::new_async().await;
    let mock_response = r#"{
      "name": "Engineering",
      "slug": "engineering",
      "memberCount": 5,
      "version": 2
    }"#;

    let _mock = server
      .mock("GET", "/api-public/v1/team/engineering")
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(mock_response)
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let result = client.get_team("engineering").await;
    assert!(result.is_ok());

    let (team, details) = result.unwrap();
    assert_eq!(team.name, Some("Engineering".to_string()));
    assert_eq!(team.member_count, Some(5));
    assert_eq!(details.status_code, 200);
  }

  #[tokio::test]
  async fn test_get_all_teams_success() {
    let mut server = mockito::Server::new_async().await;
    let mock_response = r#"[
      {
        "name": "Team A",
        "slug": "team-a",
        "memberCount": 3
      },
      {
        "name": "Team B", 
        "slug": "team-b",
        "memberCount": 5
      }
    ]"#;

    let _mock = server
      .mock("GET", "/api-public/v1/team")
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(mock_response)
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let result = client.get_all_teams().await;
    assert!(result.is_ok());

    let (teams, details) = result.unwrap();
    assert_eq!(teams.len(), 2);
    assert_eq!(details.status_code, 200);
  }

  #[tokio::test]
  async fn test_delete_team_success() {
    let mut server = mockito::Server::new_async().await;

    let _mock = server
      .mock("DELETE", "/api-public/v1/team/old-team")
      .with_status(200)
      .with_body("Team deleted successfully")
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let result = client.delete_team("old-team").await;
    assert!(result.is_ok());

    let details = result.unwrap();
    assert_eq!(details.status_code, 200);
  }

  #[tokio::test]
  async fn test_update_team_success() {
    let mut server = mockito::Server::new_async().await;
    let mock_response = r#"{
      "name": "Updated Team",
      "slug": "updated-team",
      "memberCount": 8,
      "version": 3
    }"#;

    let _mock = server
      .mock("PUT", "/api-public/v1/team/Updated%20Team")
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(mock_response)
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let team = crate::types::Team {
      name: Some("Updated Team".to_string()),
      slug: Some("updated-team".to_string()),
      member_count: Some(8),
      version: Some(3),
      is_default_team: Some(false),
    };

    let result = client.update_team(&team).await;
    assert!(result.is_ok());

    let (updated_team, details) = result.unwrap();
    assert_eq!(updated_team.name, Some("Updated Team".to_string()));
    assert_eq!(details.status_code, 200);
  }

  #[tokio::test]
  async fn test_add_team_member_success() {
    let mut server = mockito::Server::new_async().await;

    let _mock = server
      .mock("POST", "/api-public/v1/team/engineering/members")
      .with_status(200)
      .with_body("Member added successfully")
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let result = client.add_team_member("engineering", "jdoe").await;
    assert!(result.is_ok());

    let details = result.unwrap();
    assert_eq!(details.status_code, 200);
  }

  #[tokio::test]
  async fn test_remove_team_member_success() {
    let mut server = mockito::Server::new_async().await;

    let _mock = server
      .mock("DELETE", "/api-public/v1/team/engineering/members/jdoe")
      .with_status(200)
      .with_body("Member removed successfully")
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let result = client
      .remove_team_member("engineering", "jdoe", "admin")
      .await;
    assert!(result.is_ok());

    let details = result.unwrap();
    assert_eq!(details.status_code, 200);
  }

  #[tokio::test]
  async fn test_is_team_member_true() {
    let mut server = mockito::Server::new_async().await;
    let mock_response = r#"{
      "members": [
        {
          "username": "jdoe",
          "firstName": "John",
          "lastName": "Doe"
        },
        {
          "username": "alice",
          "firstName": "Alice",
          "lastName": "Smith"
        }
      ]
    }"#;

    let _mock = server
      .mock("GET", "/api-public/v1/team/engineering/members")
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(mock_response)
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let result = client.is_team_member("engineering", "jdoe").await;
    assert!(result.is_ok());

    let (is_member, details) = result.unwrap();
    assert!(is_member);
    assert_eq!(details.status_code, 200);
  }

  #[tokio::test]
  async fn test_get_user_default_email_contact_id_success() {
    let mut server = mockito::Server::new_async().await;
    let mock_response = r#"{
      "contactMethods": [
        {
          "id": 12345,
          "label": "Default",
          "email": "test@example.com"
        }
      ]
    }"#;

    let _mock = server
      .mock("GET", "/api-public/v1/user/testuser/contact-methods/emails")
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(mock_response)
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let result = client.get_user_default_email_contact_id("testuser").await;
    assert!(result.is_ok());

    let (contact_id, details) = result.unwrap();
    assert_eq!(contact_id, 12345.0);
    assert_eq!(details.status_code, 200);
  }

  #[tokio::test]
  async fn test_get_user_default_email_contact_id_no_contacts() {
    let mut server = mockito::Server::new_async().await;
    let mock_response = r#"{
      "contactMethods": []
    }"#;

    let _mock = server
      .mock("GET", "/api-public/v1/user/testuser/contact-methods/emails")
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(mock_response)
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let result = client.get_user_default_email_contact_id("testuser").await;
    assert!(result.is_err());

    if let Err(crate::Error::NotFound) = result {
      // Expected behavior when no default contact exists
    } else {
      panic!("Expected NotFound error");
    }
  }

  #[tokio::test]
  async fn test_get_team_members_success() {
    let mut server = mockito::Server::new_async().await;
    let mock_response = r#"{
      "members": [
        {
          "username": "alice",
          "firstName": "Alice",
          "lastName": "Smith"
        },
        {
          "username": "bob",
          "firstName": "Bob",
          "lastName": "Jones"
        }
      ]
    }"#;

    let _mock = server
      .mock("GET", "/api-public/v1/team/engineering/members")
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(mock_response)
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let result = client.get_team_members("engineering").await;
    assert!(result.is_ok());

    let (team_members, details) = result.unwrap();
    assert!(!team_members.members.is_empty());
    let members = &team_members.members;
    assert_eq!(members.len(), 2);
    assert_eq!(details.status_code, 200);
  }

  #[tokio::test]
  async fn test_get_team_admins_success() {
    let mut server = mockito::Server::new_async().await;
    let mock_response = r#"{
      "admin": [
        {
          "username": "admin1",
          "firstName": "Admin",
          "lastName": "User"
        }
      ]
    }"#;

    let _mock = server
      .mock("GET", "/api-public/v1/team/engineering/admins")
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(mock_response)
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let result = client.get_team_admins("engineering").await;
    assert!(result.is_ok());

    let (team_members, details) = result.unwrap();
    assert!(!team_members.admin.is_empty());
    let members = &team_members.admin;
    assert_eq!(members.len(), 1);
    assert_eq!(details.status_code, 200);
  }

  #[tokio::test]
  async fn test_get_api_team_schedule_success() {
    let mut server = mockito::Server::new_async().await;
    let mock_response = r#"{
      "team": {
        "name": "Engineering",
        "slug": "engineering"
      },
      "schedules": [
        {
          "policy": {
            "name": "Engineering Policy",
            "slug": "engineering-policy"
          }
        }
      ]
    }"#;

    let _mock = server
      .mock("GET", "/api-public/v2/team/engineering/oncall/schedule")
      .match_query(mockito::Matcher::AllOf(vec![
        mockito::Matcher::UrlEncoded("daysForward".into(), "7".into()),
        mockito::Matcher::UrlEncoded("daysSkip".into(), "0".into()),
        mockito::Matcher::UrlEncoded("step".into(), "0".into()),
      ]))
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(mock_response)
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let result = client.get_api_team_schedule("engineering", 7, 0, 0).await;
    assert!(result.is_ok());

    let (schedule, details) = result.unwrap();
    assert!(!schedule.schedules.is_empty());
    assert_eq!(details.status_code, 200);
  }

  #[tokio::test]
  async fn test_get_api_team_schedule_with_dates() {
    let mut server = mockito::Server::new_async().await;
    let mock_response = r#"{
      "team": {
        "name": "Engineering",
        "slug": "engineering"
      },
      "schedules": []
    }"#;

    let _mock = server
      .mock("GET", "/api-public/v2/team/engineering/oncall/schedule")
      .match_query(mockito::Matcher::AllOf(vec![
        mockito::Matcher::UrlEncoded("daysForward".into(), "30".into()),
        mockito::Matcher::UrlEncoded("daysSkip".into(), "0".into()),
        mockito::Matcher::UrlEncoded("step".into(), "1".into()),
      ]))
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(mock_response)
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let result = client.get_api_team_schedule("engineering", 30, 0, 1).await;
    assert!(result.is_ok());

    let (schedule, details) = result.unwrap();
    assert!(schedule.schedules.is_empty());
    assert_eq!(details.status_code, 200);
  }

  #[tokio::test]
  async fn test_get_user_on_call_schedule_success() {
    let mut server = mockito::Server::new_async().await;
    let mock_response = r#"{
      "teamSchedules": [
        {
          "team": {
            "name": "Engineering",
            "slug": "engineering"
          },
          "schedules": [
            {
              "policy": {
                "name": "Engineering Policy",
                "slug": "engineering-policy"
              }
            }
          ]
        }
      ]
    }"#;

    let _mock = server
      .mock("GET", "/api-public/v2/user/testuser/oncall/schedule")
      .match_query(mockito::Matcher::AllOf(vec![
        mockito::Matcher::UrlEncoded("daysForward".into(), "7".into()),
        mockito::Matcher::UrlEncoded("daysSkip".into(), "0".into()),
        mockito::Matcher::UrlEncoded("step".into(), "0".into()),
      ]))
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(mock_response)
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let result = client.get_user_on_call_schedule("testuser", 7, 0, 0).await;
    assert!(result.is_ok());

    let (schedule, details) = result.unwrap();
    assert!(!schedule.schedules.is_empty());
    assert_eq!(details.status_code, 200);
  }

  #[tokio::test]
  async fn test_take_on_call_for_team_success() {
    let mut server = mockito::Server::new_async().await;
    let mock_response = r#"{
      "result": "Successfully took on-call for team"
    }"#;

    let _mock = server
      .mock("PATCH", "/api-public/v1/team/engineering/oncall/user")
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(mock_response)
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let take_request = crate::types::TakeRequest {
      from_user: Some("olduser".to_string()),
      to_user: Some("newuser".to_string()),
    };

    let result = client
      .take_on_call_for_team("engineering", &take_request)
      .await;
    assert!(result.is_ok());

    let (take_response, details) = result.unwrap();
    assert_eq!(
      take_response.result,
      Some("Successfully took on-call for team".to_string())
    );
    assert_eq!(details.status_code, 200);
  }

  #[tokio::test]
  async fn test_take_on_call_for_policy_success() {
    let mut server = mockito::Server::new_async().await;
    let mock_response = r#"{
      "result": "Successfully took on-call for policy"
    }"#;

    let _mock = server
      .mock("PATCH", "/api-public/v1/policies/policy123/oncall/user")
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(mock_response)
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let take_request = crate::types::TakeRequest {
      from_user: Some("olduser".to_string()),
      to_user: Some("newuser".to_string()),
    };

    let result = client
      .take_on_call_for_policy("policy123", &take_request)
      .await;
    assert!(result.is_ok());

    let (take_response, details) = result.unwrap();
    assert_eq!(
      take_response.result,
      Some("Successfully took on-call for policy".to_string())
    );
    assert_eq!(details.status_code, 200);
  }

  #[tokio::test]
  async fn test_create_escalation_policy_success() {
    let mut server = mockito::Server::new_async().await;
    let mock_response = r#"{
      "name": "Test Policy",
      "teamSlug": "engineering",
      "ignoreCustomPagingPolicies": false,
      "steps": [],
      "slug": "test-policy-id"
    }"#;

    let _mock = server
      .mock("POST", "/api-public/v1/policies")
      .with_status(201)
      .with_header("content-type", "application/json")
      .with_body(mock_response)
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let policy = crate::types::EscalationPolicy {
      name: "Test Policy".to_string(),
      team_id: "engineering".to_string(),
      ignore_custom_paging_policies: false,
      steps: vec![],
      id: "test-policy-id".to_string(),
    };

    let result = client.create_escalation_policy(&policy).await;
    assert!(result.is_ok());

    let (created_policy, details) = result.unwrap();
    assert_eq!(created_policy.name, "Test Policy".to_string());
    assert_eq!(details.status_code, 201);
  }

  #[tokio::test]
  async fn test_get_all_escalation_policies_success() {
    let mut server = mockito::Server::new_async().await;
    let mock_response = r#"{
      "policies": [
        {
          "policy": {
            "name": "Policy 1",
            "slug": "policy-1"
          },
          "team": {
            "name": "Team 1",
            "slug": "team-1"
          }
        },
        {
          "policy": {
            "name": "Policy 2",
            "slug": "policy-2"
          },
          "team": {
            "name": "Team 2",
            "slug": "team-2"
          }
        }
      ]
    }"#;

    let _mock = server
      .mock("GET", "/api-public/v1/policies")
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(mock_response)
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let result = client.get_all_escalation_policies().await;
    assert!(result.is_ok());

    let (policies, details) = result.unwrap();
    assert_eq!(policies.policies.len(), 2);
    assert_eq!(details.status_code, 200);
  }

  #[tokio::test]
  async fn test_get_escalation_policy_success() {
    let mut server = mockito::Server::new_async().await;
    let mock_response = r#"{
      "name": "Critical Policy",
      "teamSlug": "engineering",
      "ignoreCustomPagingPolicies": false,
      "steps": [],
      "slug": "policy123"
    }"#;

    let _mock = server
      .mock("GET", "/api-public/v1/policies/policy123")
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(mock_response)
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let result = client.get_escalation_policy("policy123").await;
    assert!(result.is_ok());

    let (policy, details) = result.unwrap();
    assert_eq!(policy.name, "Critical Policy".to_string());
    assert_eq!(details.status_code, 200);
  }

  #[tokio::test]
  async fn test_delete_escalation_policy_success() {
    let mut server = mockito::Server::new_async().await;

    let _mock = server
      .mock("DELETE", "/api-public/v1/policies/policy123")
      .with_status(200)
      .with_body("Policy deleted successfully")
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let result = client.delete_escalation_policy("policy123").await;
    assert!(result.is_ok());

    let details = result.unwrap();
    assert_eq!(details.status_code, 200);
  }

  #[tokio::test]
  async fn test_create_routing_key_success() {
    let mut server = mockito::Server::new_async().await;
    let mock_response = r#"{
      "routingKey": "test-key",
      "targets": ["team1", "team2"]
    }"#;

    let _mock = server
      .mock("POST", "/api-public/v1/org/routing-keys")
      .with_status(201)
      .with_header("content-type", "application/json")
      .with_body(mock_response)
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let routing_key = crate::types::RoutingKey {
      routing_key: Some("test-key".to_string()),
      targets: vec!["team1".to_string(), "team2".to_string()],
    };

    let result = client.create_routing_key(&routing_key).await;
    assert!(result.is_ok());

    let (created_key, details) = result.unwrap();
    assert_eq!(created_key.routing_key, Some("test-key".to_string()));
    assert_eq!(details.status_code, 201);
  }

  #[tokio::test]
  async fn test_get_all_routing_keys_success() {
    let mut server = mockito::Server::new_async().await;
    let mock_response = r#"{
      "routingKeys": [
        {
          "routingKey": "key1",
          "targets": [
            {
              "policySlug": "policy1"
            }
          ]
        },
        {
          "routingKey": "key2",
          "targets": [
            {
              "policySlug": "policy2"
            }
          ]
        }
      ]
    }"#;

    let _mock = server
      .mock("GET", "/api-public/v1/org/routing-keys")
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(mock_response)
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let result = client.get_all_routing_keys().await;
    assert!(result.is_ok());

    let (routing_keys, details) = result.unwrap();
    assert!(!routing_keys.routing_keys.is_empty());
    let keys = &routing_keys.routing_keys;
    assert_eq!(keys.len(), 2);
    assert_eq!(details.status_code, 200);
  }

  #[tokio::test]
  async fn test_get_routing_key_found() {
    let mut server = mockito::Server::new_async().await;
    let mock_response = r#"{
      "routingKeys": [
        {
          "routingKey": "key1",
          "targets": [
            {
              "policySlug": "policy1"
            }
          ]
        },
        {
          "routingKey": "target-key",
          "targets": [
            {
              "policySlug": "policy2"
            }
          ]
        }
      ]
    }"#;

    let _mock = server
      .mock("GET", "/api-public/v1/org/routing-keys")
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(mock_response)
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let result = client.get_routing_key("target-key").await;
    assert!(result.is_ok());

    let (routing_key, details) = result.unwrap();
    assert!(routing_key.is_some());
    let key = routing_key.unwrap();
    assert_eq!(key.routing_key, Some("target-key".to_string()));
    assert_eq!(details.status_code, 200);
  }

  #[tokio::test]
  async fn test_get_routing_key_not_found() {
    let mut server = mockito::Server::new_async().await;
    let mock_response = r#"{
      "routingKeys": [
        {
          "routingKey": "key1",
          "targets": [
            {
              "policySlug": "policy1"
            }
          ]
        }
      ]
    }"#;

    let _mock = server
      .mock("GET", "/api-public/v1/org/routing-keys")
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(mock_response)
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let result = client.get_routing_key("nonexistent-key").await;
    assert!(result.is_ok());

    let (routing_key, details) = result.unwrap();
    assert!(routing_key.is_none());
    assert_eq!(details.status_code, 200);
  }

  #[tokio::test]
  async fn test_create_contact_success() {
    let mut server = mockito::Server::new_async().await;
    let mock_response = r#"{
      "id": 123,
      "label": "Work Email",
      "email": "test@example.com"
    }"#;

    let _mock = server
      .mock(
        "POST",
        "/api-public/v1/user/testuser/contact-methods/emails",
      )
      .with_status(201)
      .with_header("content-type", "application/json")
      .with_body(mock_response)
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let contact = crate::types::Contact {
      id: Some(123),
      label: Some("Work Email".to_string()),
      email: Some("test@example.com".to_string()),
      phone_number: None,
      rank: Some(1),
      ext_id: Some("123".to_string()),
      value: Some("test@example.com".to_string()),
      verified: Some("true".to_string()),
    };

    let result = client.create_contact("testuser", &contact).await;
    assert!(result.is_ok());

    let (created_contact, details) = result.unwrap();
    assert_eq!(created_contact.email, Some("test@example.com".to_string()));
    assert_eq!(details.status_code, 201);
  }

  #[tokio::test]
  async fn test_create_contact_invalid_input() {
    let server = mockito::Server::new_async().await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let contact = crate::types::Contact {
      id: Some(123),
      label: Some("Invalid Contact".to_string()),
      email: None,
      phone_number: None,
      rank: Some(1),
      ext_id: Some("123".to_string()),
      value: None,
      verified: None,
    };

    let result = client.create_contact("testuser", &contact).await;
    assert!(result.is_err());

    if let Err(e) = result {
      match e {
        crate::error::Error::InvalidInput(msg) => {
          assert_eq!(msg, "Contact must have either phone_number or email");
        }
        _ => panic!("Expected InvalidInput error"),
      }
    }
  }

  #[tokio::test]
  async fn test_get_contact_success() {
    let mut server = mockito::Server::new_async().await;
    let mock_response = r#"{
      "id": 123,
      "label": "Work Email",
      "email": "test@example.com"
    }"#;

    let _mock = server
      .mock(
        "GET",
        "/api-public/v1/user/testuser/contact-methods/emails/123",
      )
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(mock_response)
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let result = client
      .get_contact("testuser", "123", crate::types::ContactType::Email)
      .await;
    assert!(result.is_ok());

    let (contact, details) = result.unwrap();
    assert_eq!(contact.email, Some("test@example.com".to_string()));
    assert_eq!(details.status_code, 200);
  }

  #[tokio::test]
  async fn test_get_all_contacts_success() {
    let mut server = mockito::Server::new_async().await;
    let mock_response = r#"{
      "emails": {
        "contactMethods": [
          {
            "id": 123,
            "label": "Work Email",
            "email": "work@example.com"
          }
        ]
      },
      "phones": {
        "contactMethods": [
          {
            "id": 456,
            "label": "Cell Phone",
            "phone": "+1234567890"
          }
        ]
      }
    }"#;

    let _mock = server
      .mock("GET", "/api-public/v1/user/testuser/contact-methods")
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(mock_response)
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let result = client.get_all_contacts("testuser").await;
    assert!(result.is_ok());

    let (contact_list, details) = result.unwrap();
    assert!(contact_list.emails.is_some());
    let emails = contact_list.emails.unwrap();
    assert_eq!(emails.contact_methods.len(), 1);
    assert_eq!(details.status_code, 200);
  }

  #[tokio::test]
  async fn test_delete_contact_success() {
    let mut server = mockito::Server::new_async().await;

    let _mock = server
      .mock(
        "DELETE",
        "/api-public/v1/user/testuser/contact-methods/emails/123",
      )
      .with_status(200)
      .with_body("Contact deleted successfully")
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let result = client
      .delete_contact("testuser", "123", crate::types::ContactType::Email)
      .await;
    assert!(result.is_ok());

    let details = result.unwrap();
    assert_eq!(details.status_code, 200);
  }

  #[tokio::test]
  async fn test_get_routing_key_empty_list() {
    let mut server = mockito::Server::new_async().await;
    let mock_response = r#"{
      "routingKeys": []
    }"#;

    let _mock = server
      .mock("GET", "/api-public/v1/org/routing-keys")
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(mock_response)
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let result = client.get_routing_key("any-key").await;
    assert!(result.is_ok());

    let (routing_key, details) = result.unwrap();
    assert!(routing_key.is_none());
    assert_eq!(details.status_code, 200);
  }

  #[tokio::test]
  async fn test_create_contact_phone_success() {
    let mut server = mockito::Server::new_async().await;
    let mock_response = r#"{
      "id": 456,
      "label": "Work Phone",
      "phone": "+1234567890"
    }"#;

    let _mock = server
      .mock(
        "POST",
        "/api-public/v1/user/testuser/contact-methods/phones",
      )
      .with_status(201)
      .with_header("content-type", "application/json")
      .with_body(mock_response)
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let contact = crate::types::Contact {
      id: Some(456),
      label: Some("Work Phone".to_string()),
      email: None,
      phone_number: Some("+1234567890".to_string()),
      rank: Some(1),
      ext_id: Some("456".to_string()),
      value: Some("+1234567890".to_string()),
      verified: Some("true".to_string()),
    };

    let result = client.create_contact("testuser", &contact).await;
    assert!(result.is_ok());

    let (created_contact, details) = result.unwrap();
    assert_eq!(
      created_contact.phone_number,
      Some("+1234567890".to_string())
    );
    assert_eq!(details.status_code, 201);
  }

  #[tokio::test]
  async fn test_get_contact_phone_success() {
    let mut server = mockito::Server::new_async().await;
    let mock_response = r#"{
      "id": 456,
      "label": "Work Phone",
      "phone": "+1234567890"
    }"#;

    let _mock = server
      .mock(
        "GET",
        "/api-public/v1/user/testuser/contact-methods/phones/456",
      )
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(mock_response)
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let result = client
      .get_contact("testuser", "456", crate::types::ContactType::Phone)
      .await;
    assert!(result.is_ok());

    let (contact, details) = result.unwrap();
    assert_eq!(contact.phone_number, Some("+1234567890".to_string()));
    assert_eq!(details.status_code, 200);
  }

  #[tokio::test]
  async fn test_delete_contact_phone_success() {
    let mut server = mockito::Server::new_async().await;

    let _mock = server
      .mock(
        "DELETE",
        "/api-public/v1/user/testuser/contact-methods/phones/456",
      )
      .with_status(200)
      .with_body("Contact deleted successfully")
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let result = client
      .delete_contact("testuser", "456", crate::types::ContactType::Phone)
      .await;
    assert!(result.is_ok());

    let details = result.unwrap();
    assert_eq!(details.status_code, 200);
  }

  #[tokio::test]
  async fn test_client_with_custom_timeout() {
    let timeout = std::time::Duration::from_secs(30);
    let result = Client::with_timeout(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      "https://api.victorops.com".to_string(),
      timeout,
    );

    assert!(result.is_ok());
  }

  #[tokio::test]
  async fn test_routing_key_with_none_routing_key() {
    let mut server = mockito::Server::new_async().await;
    let mock_response = r#"{
      "routingKeys": [
        {
          "targets": [
            {
              "policySlug": "policy1"
            }
          ]
        }
      ]
    }"#;

    let _mock = server
      .mock("GET", "/api-public/v1/org/routing-keys")
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(mock_response)
      .create_async()
      .await;

    let client = Client::new(
      "test-api-id".to_string(),
      "test-api-key".to_string(),
      server.url(),
    )
    .unwrap();

    let result = client.get_routing_key("any-key").await;
    assert!(result.is_ok());

    let (routing_key, details) = result.unwrap();
    assert!(routing_key.is_none());
    assert_eq!(details.status_code, 200);
  }
}
