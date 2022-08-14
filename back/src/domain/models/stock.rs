#[derive(Clone, Debug, PartialEq)]
pub struct Stock {
    raw: i32,
}

impl From<Stock> for i32 {
    fn from(stock: Stock) -> Self {
        stock.raw
    }
}

impl TryFrom<i32> for Stock {
    type Error = ();

    fn try_from(raw: i32) -> Result<Self, Self::Error> {
        if raw < 0 {
            Err(())
        } else {
            Ok(Self { raw })
        }
    }
}
