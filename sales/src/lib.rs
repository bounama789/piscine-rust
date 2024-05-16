#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>
}
impl Cart {
    pub fn new() -> Cart {
        Self { items: Vec::new(),receipt:Vec::new() }
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        let products: Vec<&(String, f32)> = s.products.iter().filter(|c| c.0 == ele).collect();
        if let Some(&p) = products.first() {
            // ensure that the product exist in the store
            self.items.push((p.0.clone(), p.1))
        }
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let num_of_free_items = self.items.len() / 3;
        let mut prices: Vec<f32> = self.items.iter().map(|e| e.1).collect();
        let total = prices.iter().fold(0.0, |acc, p| acc + p);

        if num_of_free_items > 0 {
            prices.sort_by(|a, b| a.total_cmp(b));
            let after_reduction = total
                - prices
                    .get(0..num_of_free_items)
                    .unwrap()
                    .iter()
                    .fold(0.0, |acc, p| acc + p);

            let coef = 1.0 - (total - after_reduction) / total;

            self.receipt = prices.iter().map(|p| (p * coef*100.0).round()/100.0).collect();

            return self.receipt.clone()
        };
        self.receipt =prices.clone();
        prices
    }
}
