pub struct SessionData {
    pub USERNAME: String,
    pub UID: i32,
    pub HWID: String,
    pub CLIENT: String,
    pub DOMEN: String,
}

impl SessionData {
    pub fn getUsername(&self) -> String {
        return self.USERNAME.clone()
    }
    pub fn getUid(&self) -> i32 {
        return self.UID
    }
    pub fn getHwid(&self) -> String {
        return self.HWID.clone()
    }
    pub fn getClient(&self) -> String {
        return self.CLIENT.clone()
    }
    pub fn getDomen(&self) -> String {
        return format!("{}{}", self.CLIENT, self.DOMEN);
    }
}
