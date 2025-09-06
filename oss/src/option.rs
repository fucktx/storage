pub struct GetOption {
    pub expire: i64,
}

pub struct GetOptionBuilder {
    expire: Option<i64>,
}

impl GetOptionBuilder {
    pub fn new() -> Self {
        Self { expire: Option::Some(0) }
    }
    pub fn expire(mut self, expire: i64) ->Self{
        self.expire = Some(expire);
        self
    }

    pub fn build(self) -> GetOption {
        GetOption{expire: self.expire.unwrap()}
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_option() {
        let get_opt = GetOptionBuilder::new().expire(2).build();
        println!("{:#?}", get_opt.expire)
    }
}