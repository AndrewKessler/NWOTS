#[derive(Debug, Clone)]
pub struct InventoryItem {

    pub id: String,

    pub quantity: u32,
}

#[derive(Debug, Clone)]
pub struct Inventory {

    pub slots:
        Vec<InventoryItem>,
}

impl Default for Inventory {

    fn default() -> Self {

        Self {

            slots: Vec::new(),
        }
    }
}