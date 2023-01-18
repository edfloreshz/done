# Plugin system
The Done application is equipped with an integrated plugin system, utilizing `gRPC` for effective communication between the app and its various services.

Plugins operate as distinct processes, initiated by the app upon start-up and terminated upon closure. The app utilizes `gRPC` to make requests to the 
service, as specified in the predefined `.proto` file, allowing the plugin to seamlessly communicate with the underlying service and retrieve necessary 
information.

![plugin-system](https://user-images.githubusercontent.com/22224438/213039114-5485cf37-a1d2-4d8f-ad5e-c79d632027a0.png)

This design facilitates the ability for the app to maintain its functionality, even in the event of a service outage. Additionally, it enables 
developers to utilize their preferred programming language when creating plugins, all while ensuring seamless communication through the use of `gRPC`.

In order to start, it is necessary to review the generated code specific to your chosen programming language. 

The required resources are currently available in the following languages: 

- [Rust](https://github.com/done-devs/proto-rust)
- [Go](https://github.com/done-devs/proto-rust)

Additional languages will be made available in the near future.

# How to start
Each plugin is responsible for communicating with the designated service and acquiring the necessary information. To accomplish this, it is necessary 
to implement the `Provider` interface, enabling communication with the client.

```rust
#[tonic::async_trait]
impl Provider for LocalService {
    // Implementation details
}
```

Once the implementation of the Provider interface is complete, the server can be initiated.

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:7007".parse()?;

    let local_service = LocalService {
        id: "Local".to_string(),
        name: "Local".to_string(),
        description: "Stores tasks on your computer.".to_string(),
        icon: "user-home-symbolic".to_string(),
    };

    Server::builder()
        .add_service(ProviderServer::new(local_service))
        .serve(addr)
        .await?;

    Ok(())
}
```

# Publish a new plugin
When you're done, you can include your plugin to the list of services of Done. 

To do it, you'll need to add the following object to [`dev.edfloreshz.Done.Plugins.json`](dev.edfloreshz.Done.Plugins.json).

```json
{
    "pluginId": string,
    "pluginName": string,
    "pluginDescription": string,
    "pluginIcon": string,
    "pluginPort": int,
    "pluginVersion": string,
    "pluginProcessName": string,
    "pluginDownloadUrl": string
}
```

Fill the fields with information about your plugin, send a [PR](https://github.com/done-devs/done/pulls) and wait for approval.

# Examples
You can guide yourself by looking at existing implementations:

- [Local Service](https://github.com/done-devs/local-plugin)
