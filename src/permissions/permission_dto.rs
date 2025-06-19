use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PermissionCheckDto {
    pub check_status: bool,
}
