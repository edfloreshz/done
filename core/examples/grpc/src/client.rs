use provider::provider_client::ProviderClient;
use provider::{List, ProviderRequest, Task};

pub mod provider {
	tonic::include_proto!("provider");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ProviderClient::connect("http://[::1]:5123").await?;

	let list = List {
		id: String::new(),
		name: String::from("Shopping"),
		is_owner: false,
		count: 0,
		icon: String::new(),
		provider: String::new(),
	};

	let task = Task {
		id: String::new(),
		parent: String::new(),
		title: String::new(),
		body: String::new(),
		importance: 0,
		favorite: false,
		is_reminder_on: false,
		status: 0,
		completed_on: Default::default(),
		due_date: Default::default(),
		reminder_date: Default::default(),
		created_date_time: Default::default(),
		last_modified_date_time: Default::default(),
	};

	let request = tonic::Request::new(ProviderRequest {
		list: Some(list),
		task: Some(task),
	});

	let resposne = client.add_task(request).await?;
    let id = client.get_id(crate::provider::Void {}).await?;

    print!("Response: {:#?} with id: {}", resposne, id.into_inner());

	Ok(())
}
