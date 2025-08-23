use inventory::submit;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Define a trait for inventory operations
trait InventoryOp: Send + Sync {
    fn name(&self) -> &'static str;
    fn execute(&self, inventory: &Mutex<HashMap<String, u32>>, item: &str, quantity: u32);
}

// Structure for registry
#[derive(Clone, Copy)]
struct InventoryPlugin {
    name: &'static str,
    handler: &'static dyn InventoryOp,
}

inventory::collect!(InventoryPlugin);

// Plugin: Empty Operation
struct EmptyOp;
impl InventoryOp for EmptyOp {
    fn name(&self) -> &'static str {
        ""
    }
    fn execute(&self, _inventory: &Mutex<HashMap<String, u32>>, _item: &str, _quantity: u32) {}
}

inventory::submit! {
    InventoryPlugin {
        name: "",
        handler: &EmptyOp,
    }
}

// Plugin 1: Add Item
struct AddItem;
impl InventoryOp for AddItem {
    fn name(&self) -> &'static str {
        "add"
    }
    fn execute(&self, inventory: &Mutex<HashMap<String, u32>>, item: &str, quantity: u32) {
        let mut inv = inventory.lock().unwrap();
        let current = inv.get(item).copied().unwrap_or(0);
        inv.insert(item.to_string(), current + quantity);
        println!("Added {} {} to inventory", quantity, item);
    }
}

inventory::submit! {
    InventoryPlugin {
        name: "add",
        handler: &AddItem,
    }
}

// Plugin 2: Remove Item
struct RemoveItem;
impl InventoryOp for RemoveItem {
    fn name(&self) -> &'static str {
        "remove"
    }
    fn execute(&self, inventory: &Mutex<HashMap<String, u32>>, item: &str, quantity: u32) {
        let mut inv = inventory.lock().unwrap();
        if let Some(current) = inv.get_mut(item) {
            if *current >= quantity {
                *current -= quantity;
                println!("Removed {} {} from inventory", quantity, item);
            } else {
                println!("Error: Not enough {} in inventory", item);
            }
        } else {
            println!("Error: {} not found in inventory", item);
        }
    }
}

inventory::submit! {
    InventoryPlugin {
        name: "remove",
        handler: &RemoveItem,
    }
}

fn inventory_main() {
    // Thread-safe inventory
    let inventory = Arc::new(Mutex::new(HashMap::<String, u32>::new()));
    inventory.lock().unwrap().insert("apple".to_string(), 10);

    // Collect plugins
    let mut plugins: HashMap<&'static str, &'static dyn InventoryOp> = HashMap::new();

    for plugin in inventory::iter::<InventoryPlugin> {
        if !plugin.name.is_empty() {
            plugins.insert(plugin.name, plugin.handler);
        }
    }

    // Simulate operations
    let ops = vec![
        ("add", "apple", 5),
        ("remove", "apple", 3),
        ("add", "sword", 2),
        ("remove", "apple", 20),
    ];

    for (op_name, item, quantity) in ops {
        if let Some(op) = plugins.get(op_name) {
            op.execute(&inventory, item, quantity);
        } else {
            println!("Unknown operation: {}", op_name);
        }
    }

    // Print final inventory
    println!("Final inventory:");
    let inv = inventory.lock().unwrap();
    for (key, quantity) in inv.iter() {
        println!("{}: {}", key, quantity);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inventory_main() {
        inventory_main();
    }
}
