use rs_consul::{
    types::*, Config, Consul, RegisterEntityCheck, RegisterEntityPayload, RegisterEntityService,
};

use std::collections::HashMap;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main_consul() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 配置 Consul 客户端
    // 默认连接到 localhost:8500。如果你有不同的Consul地址，可以在这里配置。
    // 例如：.address("http://your-consul-host:8500")

    let var_name = Config {
        address: "http://192.168.2.6:8500".to_string(),
        token: None,          // No token required in development mode
        ..Default::default()  // Uses default values for other settings
    };
    let consul_config = var_name;

    let consul = Consul::new(consul_config);

    // 2. 使用 Consul 客户端进行服务注册、发现和 KV 存储操作

    println!("Attempting to connect to Consul at: {:?}", consul);
    // 执行服务注册
    // --- 服务注册示例 ---
    let node_id = "root-node"; //node name
    let service_name = "new-service-1"; //service name

    println!("\n--- Registering service: {} ---", service_name);

    let payload = RegisterEntityPayload {
        ID: None,
        Node: node_id.to_string(),
        Address: "192.168.2.7".to_string(), //server address
        Datacenter: None,
        TaggedAddresses: Default::default(),
        NodeMeta: Default::default(),
        Service: Some(RegisterEntityService {
            ID: None,
            Service: service_name.to_string(),
            Tags: vec![],
            TaggedAddresses: Default::default(),
            Meta: Default::default(),
            Port: Some(42424),
            Namespace: None,
        }),
        Checks: vec![],
        SkipNodeUpdate: None,
    };

    consul.register_entity(&payload).await.unwrap();

    // 4. 查询健康状态（可选）
    // 查询健康状态（使用 rs_consul 的 health().service() 方法）
    println!(
        "\n--- Checking health status for service: {} ---",
        service_name
    );

    // // --- 服务发现示例 (简单查询，确认服务已注册) ---

    let service_list = consul.get_all_registered_service_names(None).await?;
    println!("\n--- Discovering service: {:?} ---", service_list);

    let service_node = consul
        .get_service_addresses_and_ports(service_name, None)
        .await?;

    println!(
        "\n--- Discovering service: {:?} ,server instance:{:?}---",
        service_name, service_node
    );

    // // --- KV 存储示例 ---
    let kv_key = "hello_app/config/feature_x_enabled";
    let kv_value = b"true"; // KV 值是字节数组

    let create_key_request = CreateOrUpdateKeyRequest {
        key: kv_key,
        ..Default::default()
    };
    consul
        .create_or_update_key(create_key_request, kv_value.to_vec())
        .await
        .unwrap();

    // // 存储一个 KV 对
    println!("\n--- Putting KV pair: {} = {:?} ---", kv_key, kv_value);

    // // 获取 KV 对
    let read_key_request = ReadKeyRequest {
        key: kv_key,
        ..Default::default()
    };

    let read_key_result = consul.read_key(read_key_request).await.unwrap();

    println!(
        "\n--- Getting KV pair: {},value:{:?} ---",
        kv_key, read_key_result
    );

    // // --- 服务注销示例 ---
    let node_id = "root-node";
    let service_name = "new-service-1";
    println!("\n--- Deregistering service: {} ---", service_name);

    let payload = DeregisterEntityPayload {
        Node: Some(node_id.to_string()),
        Datacenter: None,
        CheckID: None,
        ServiceID: Some(service_name.to_string()),
        Namespace: None,
    };
    consul.deregister_entity(&payload).await.unwrap();

    println!("\nDemonstration complete.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore = "consul client requires a running Consul instance"]
    #[test]
    fn test_hello_client() {
        main_consul().unwrap();
    }
}
