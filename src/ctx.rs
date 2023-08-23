#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: u64
}

impl Ctx {
    pub fn new (user_id: u64) -> Self{
        Self {
            user_id
        }
    }

    pub fn get_user_id(&self)-> u64 {
        self.user_id
    }
}
