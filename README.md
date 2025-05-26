# VictorOps Rust Client

[![Build Status](https://github.com/endoze/victorops/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/endoze/victorops/actions?query=branch%3Amaster)
[![Coverage Status](https://coveralls.io/repos/github/endoze/victorops/badge.svg?branch=master)](https://coveralls.io/github/endoze/victorops?branch=master)
[![Crate](https://img.shields.io/crates/v/victorops.svg)](https://crates.io/crates/victorops)
[![Docs](https://docs.rs/victorops/badge.svg)](https://docs.rs/victorops)

A Rust client library for the VictorOps (Splunk On-Call) REST API.

## Features

- Complete VictorOps REST API coverage
- Async/await support with Tokio
- Type-safe API responses with Serde
- Comprehensive error handling
- Request/response details for debugging
- Configurable HTTP client with timeout support

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
victorops = "0.1.0"
```

## Quick Start

```rust,no_run
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let api_id = "your-api-id".to_string();
  let api_key = "your-api-key".to_string();
  
  let client = victorops::Client::new(
    api_id,
    api_key,
    "https://api.victorops.com".to_string(),
  )?;
  
  let (incidents, _details) = client.get_incidents().await?;
  println!("Found {} incidents", incidents.incidents.map_or(0, |i| i.len()));

  Ok(())
}
```

## Examples

Run the team schedule example:

```bash
export VICTOROPS_API_ID="your-api-id"
export VICTOROPS_API_KEY="your-api-key"
cargo run --example team_schedule team-slug
```

Run the incidents example:

```bash
export VICTOROPS_API_ID="your-api-id"
export VICTOROPS_API_KEY="your-api-key"
cargo run --example incidents
```

## API Coverage

### Incidents
- `get_incident(id)` - Get a specific incident
- `get_incidents()` - Get all incidents

### Users
- `create_user(user)` - Create a new user
- `get_user(username)` - Get user by username
- `get_user_by_email(email)` - Get user by email address
- `get_all_users()` - Get all users (v1)
- `get_all_users_v2()` - Get all users (v2)
- `update_user(user)` - Update user information
- `delete_user(username, replacement)` - Delete user with replacement

### Teams
- `create_team(team)` - Create a new team
- `get_team(team_id)` - Get team by ID
- `get_all_teams()` - Get all teams
- `get_team_members(team_id)` - Get team members
- `get_team_admins(team_id)` - Get team administrators
- `update_team(team)` - Update team information
- `delete_team(team_id)` - Delete team
- `add_team_member(team_id, username)` - Add member to team
- `remove_team_member(team_id, username, replacement)` - Remove member from team
- `is_team_member(team_id, username)` - Check if user is team member

### On-Call Schedules
- `get_api_team_schedule()` - Get team on-call schedule
- `get_user_on_call_schedule()` - Get user on-call schedule
- `take_on_call_for_team()` - Take on-call for team
- `take_on_call_for_policy()` - Take on-call for escalation policy

### Escalation Policies
- `create_escalation_policy(policy)` - Create escalation policy
- `get_escalation_policy(id)` - Get escalation policy by ID
- `get_all_escalation_policies()` - Get all escalation policies
- `delete_escalation_policy(id)` - Delete escalation policy

### Routing Keys
- `create_routing_key(key)` - Create routing key
- `get_routing_key(name)` - Get routing key by name
- `get_all_routing_keys()` - Get all routing keys

### Contact Methods
- `create_contact(username, contact)` - Create contact method
- `get_contact(username, ext_id, type)` - Get contact method
- `get_all_contacts(username)` - Get all contact methods for user
- `get_contact_by_id(username, id, type)` - Get contact method by ID
- `delete_contact(username, ext_id, type)` - Delete contact method

## Configuration

### Basic Client
```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let client = victorops::Client::new(
    "api-id".to_string(),
    "api-key".to_string(),
    "https://api.victorops.com".to_string(),
 )?;

 Ok(())
}
```

### Client with Custom Timeout
```rust
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let client = victorops::Client::with_timeout(
    "api-id".to_string(),
    "api-key".to_string(),
    "https://api.victorops.com".to_string(),
    Duration::from_secs(60),
  )?;

  Ok(())
}
```

## Request Details

All API methods return a tuple containing the response data and request details:

```rust,no_run
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let client = victorops::Client::new(
    "api-id".to_string(),
    "api-key".to_string(), 
    "https://api.victorops.com".to_string(),
  )?;

  let (user, details) = client.get_user("username").await?;

  println!("Status: {}", details.status_code);
  println!("Response: {}", details.response_body);
  println!("Request: {}", details.request_body);

  Ok(())
}
```

## Error Handling

The library provides comprehensive error handling through the `Error` enum:

```rust,no_run
use victorops::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let client = victorops::Client::new(
    "api-id".to_string(),
    "api-key".to_string(),
    "https://api.victorops.com".to_string(),
  )?;
  
  match client.get_user("nonexistent").await {
    Ok((user, _)) => println!("User found: {:?}", user),
    Err(Error::Api { status: 404, .. }) => println!("User not found"),
    Err(Error::Http(e)) => println!("HTTP error: {}", e),
    Err(e) => println!("Other error: {}", e),
  }

  Ok(())
}
```

### Error Types
- `Http` - HTTP request failures
- `Json` - JSON serialization/deserialization errors
- `UrlParse` - URL parsing errors
- `InvalidHeaderValue` - Invalid HTTP header values
- `Api` - API-specific errors with status codes
- `Authentication` - Authentication failures
- `NotFound` - Resource not found
- `InvalidInput` - Invalid input parameters

## Types

The library includes comprehensive type definitions for all VictorOps entities:

- `User` - User account information
- `Team` - Team details and membership
- `Incident` - Incident data and transitions
- `EscalationPolicy` - Escalation policy configuration
- `Contact` - Contact method information
- `RoutingKey` - Routing key configuration
- Schedule types for on-call management

All types support Serde serialization/deserialization and include optional fields as appropriate for the VictorOps API.

## License

This project is licensed under the MIT License.
