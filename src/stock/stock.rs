pub struct Stock{
    pub name: Option<&'static str>,
    pub symbol: String
    }

impl Stock {
    pub fn new(sym: String) -> Stock {
        Stock {
            name: None,
            symbol: sym
        }
    }

    pub fn to_string(self) -> String {
        self.symbol.to_string()
    }
}
