#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: i64,
}

// Contructur.
impl Ctx {
    pub fn new(user_id: i64) -> Self {
        Self { user_id }
    }
}

// Property Accessors.
impl Ctx {
    pub fn user_id(&self) -> i64 {
        self.user_id
    }
}
