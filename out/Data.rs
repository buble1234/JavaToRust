pub struct SessionData
{
}

impl SessionData
{
    pub const USERNAME: &'static str = "NoCap";
    pub const UID: i32 = 1;
    pub const HWID: &'static str = "-";
    pub const CLIENT: &'static str = "Taksa";
    pub const DOMEN: &'static str = ".pw";
    pub fn getUsername() -> String
    {
        return Self::USERNAME.to_string();
    }
    pub fn getUid() -> i32
    {
        return Self::UID;
    }
    pub fn getHwid() -> String
    {
        return Self::HWID.to_string();
    }
    pub fn getClient() -> String
    {
        return Self::CLIENT.to_string();
    }
    pub fn getDomen() -> String
    {
        return format!("{}{}", Self::CLIENT, Self::DOMEN);
    }
}
