# mailinator-rs

[![Crates.io][crates-badge]][crates-url]
![](https://img.shields.io/badge/Rust-1.67+-orange.svg)
![](https://img.shields.io/badge/Edition-2021-orange.svg)
[![MIT licensed][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/crates/v/mailinator-rs.svg?color=blue
[crates-url]: https://crates.io/crates/mailinator-rs
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/tokio-rs/tokio/blob/master/LICENSE

## Non official rust wrapper for mailinator api.

[![](https://www.mailinator.com/docs/images/logo-cf14a181.png)](https://www.mailinator.com/)
## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
mailinator-rs = "0.1"
tokio = { version = "1.15", features = ["full"] }
```
Example
```rust
use mailinator_rs::prelude::{MailinatorClient, StatsApi};

#[tokio::main]
async fn main() {
    let team_api_token = String::from("XXXXX-XXXXXX");

    let client = MailinatorClient::new(None, Some(team_api_token));

    match client.get_usage_statistica().await {
        Err(e) => println!("Error: {e}"),
        Ok(usage) => println!("{usage:?}")
    }
}
```
<br/>

### Message Api
<br/>

 *  #### Fetch Inbox
```rust
use mailinator_rs::prelude::{
    FetchInboxRequestBuilder, FetchInboxRequestQueryParamsBuilder, MailinatorClient, MessageApi,Sorting
};


#[tokio::main]
async fn main() {
    let team_api_token = String::from("XXXXX-XXXXXX");
    let client = MailinatorClient::new(None, Some(team_api_token));
    let domain = String::from("example.com");
    let inbox = String::from("testinbox");

    let params = FetchInboxRequestQueryParamsBuilder::default()
        .limit(Some(1))
        .sort(Some(Sorting::Ascending))
        .build()
        .expect("failed to build query params");

    let request = FetchInboxRequestBuilder::default()
        .domain(domain)
        .inbox(inbox)
        .query_params(Some(params))
        .build()
        .expect("failed to build request");

    match client.fetch_inbox(request).await {
        Err(e) => println!("Error: {e}"),
        Ok(msgs) => println!("{msgs:?}"),
    }
}

```
* #### Fetch Message
```rust
use mailinator_rs::prelude::{FetchMessageRequestBuilder, MailinatorClient, MessageApi};

#[tokio::main]
async fn main() {
    let team_api_token = String::from("XXXXX-XXXXXX");
    let domain = String::from("example.com");
    let msg_id = String::from("xxx-id-xx-msg");

    let client = MailinatorClient::new(None, Some(team_api_token));

    let request = FetchMessageRequestBuilder::default()
        .domain(domain)
        .message_id(msg_id)
        .build()
        .expect("failed to build request");

    match client.fetch_message(request).await {
        Err(e) => println!("Error: {e}"),
        Ok(msg) => println!("{msg:?}"),
    }
}
```

* ### Fetch Sms
```rust
use mailinator_rs::prelude::{FetchSmsMessageRequestBuilder, MailinatorClient, MessageApi};

#[tokio::main]
async fn main() {
    let team_api_token = String::from("XXXXX-XXXXXX");
    let domain = String::from("example.com");
    let sms_number: u64 = 12345678;

    let client = MailinatorClient::new(None, Some(team_api_token));

    let request = FetchSmsMessageRequestBuilder::default()
        .domain(domain)
        .sms_number(sms_number)
        .build()
        .expect("failed to build request");

    match client.fetch_sms_message(request).await {
        Err(e) => println!("Error: {e}"),
        Ok(msg) => println!("{msg:?}"),
    }
}

```
* ### List Attachments
```rust
use mailinator_rs::prelude::{FetchListOfAttachmentRequestBuilder, MailinatorClient, MessageApi};

#[tokio::main]
async fn main() {
    let team_api_token = String::from("XXXXX-XXXXXX");
    let domain = String::from("example.com");
    let inbox = String::from("testinbox");
    let msg_id = String::from("msg_id");

    let client = MailinatorClient::new(None, Some(team_api_token));

    let request = FetchListOfAttachmentRequestBuilder::default()
        .domain(domain)
        .inbox(inbox)
        .message_id(msg_id)
        .build()
        .expect("failed to build request");

    match client.fetch_list_of_attachments(request).await {
        Err(e) => println!("Error: {e}"),
        Ok(at_list) => println!("{at_list:?}"),
    }
}
```
* ### Fetch Links 

```rust
use mailinator_rs::prelude::{FetchLinkRequestBuilder, MailinatorClient, MessageApi};

#[tokio::main]
async fn main() {
    let team_api_token = String::from("XXXXX-XXXXXX");
    let domain = String::from("example.com");
    let inbox = String::from("testinbox");
    let msg_id = String::from("msg_id");

    let client = MailinatorClient::new(None, Some(team_api_token));

    let request = FetchLinkRequestBuilder::default()
        .domain(domain)
        .inbox(inbox)
        .message_id(msg_id)
        .build()
        .expect("failed to build request");

    match client.fetch_links(request).await {
        Err(e) => println!("Error: {e}"),
        Ok(links) => println!("{links:?}"),
    }
}

```
* ### Fetch Attachment
```rust
use mailinator_rs::prelude::{
    AttachmentLookupField, FetchAttachmentRequestBuilder, MailinatorClient, MessageApi,
};

#[tokio::main]
async fn main() {
    let team_api_token = String::from("XXXXX-XXXXXX");
    let domain = String::from("example.com");
    let inbox = String::from("testinbox");
    let msg_id = String::from("msg_id");
    let attachment_id = AttachmentLookupField::Id(1);

    let client = MailinatorClient::new(None, Some(team_api_token));

    let request = FetchAttachmentRequestBuilder::default()
        .domain(domain)
        .inbox(inbox)
        .message_id(msg_id)
        .attachment(attachment_id)
        .build()
        .expect("failed to build request");

    match client.fetch_attachment(request).await {
        Err(e) => println!("Error: {e}"),
        Ok(msg) => println!("{msg:?}"),
    }
}
```
* ### Delete All Domain Messages
```rust
use mailinator_rs::prelude::{DeleteAllDomainMessageRequestBuilder, MailinatorClient, MessageApi};

#[tokio::main]
async fn main() {
    let team_api_token = String::from("XXXXX-XXXXXX");
    let domain = String::from("example.com");

    let client = MailinatorClient::new(None, Some(team_api_token));

    let request = DeleteAllDomainMessageRequestBuilder::default()
        .domain(domain)
        .build()
        .expect("failed to build request");

    match client.delete_all_domain_messages(request).await {
        Err(e) => println!("Error: {e}"),
        Ok(status) => println!("{status:?}"),
    }
}
```
* ### Delete Message
```rust
use mailinator_rs::prelude::{DeleteMessageRequestBuilder, MailinatorClient, MessageApi};

#[tokio::main]
async fn main() {
    let team_api_token = String::from("XXXXX-XXXXXX");
    let domain = String::from("example.com");
    let inbox = String::from("testinbox");
    let msg_id = String::from("msg_id");

    let client = MailinatorClient::new(None, Some(team_api_token));

    let request = DeleteMessageRequestBuilder::default()
        .domain(domain)
        .inbox(inbox)
        .message_id(msg_id)
        .build()
        .expect("failed to build request");

    match client.delete_message(request).await {
        Err(e) => println!("Error: {e}"),
        Ok(status) => println!("{status:?}"),
    }
}
```
* ### Inject Message
```rust
use mailinator_rs::prelude::{
    InjectMessageRequestBuilder, MailinatorClient, MessageApi, NewEmailMessageBuilder,
};

#[tokio::main]
async fn main() {
    let team_api_token = String::from("XXXXX-XXXXXX");
    let domain = String::from("example.com");
    let inbox = String::from("testinbox");

    let client = MailinatorClient::new(None, Some(team_api_token));

    let request = InjectMessageRequestBuilder::default()
        .domain(domain)
        .inbox(inbox)
        .build()
        .expect("failed to build request");

    let email = NewEmailMessageBuilder::default()
        .subject(String::from("Hello world"))
        .text(Some(String::from("An example email.")))
        .from(String::from("sender@example.com"))
        .build()
        .expect("failed to build email message");

    match client.inject_message(request, email).await {
        Err(e) => println!("Error: {e}"),
        Ok(status) => println!("{status:?}"),
    }
}
```

### Stats Api

### Domains Api

### Rules Api