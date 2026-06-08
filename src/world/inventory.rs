#[derive(Debug, Clone)]
pub struct InventoryItem {

    pub id: String,

    pub quantity: u32,
}

#[derive(Debug, Clone)]
pub struct Inventory {

    pub slots:
        Vec<InventoryItem>,

    pub equipped_weapon:
        Option<String>,
}

impl Default for Inventory {

    fn default() -> Self {

        Self {

            slots:
                Vec::new(),

            equipped_weapon:
                None,
        }
    }
}

impl Inventory {

    pub fn add_item(
        &mut self,
        id: &str,
        quantity: u32,
    ) {

        for slot in
            &mut self.slots
        {

            if slot.id == id {

                slot.quantity +=
                    quantity;

                return;
            }
        }

        self.slots.push(

            InventoryItem {

                id:
                    id.to_string(),

                quantity,
            }
        );
    }

    pub fn has_item(
        &self,
        id: &str,
    ) -> bool {

        self.slots.iter()
            .any(
                |item|
                    item.id == id
            )
    }
}