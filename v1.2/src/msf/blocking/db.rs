pub fn hosts() {}
pub fn get_host() {}
pub fn report_host() {}
pub fn del_host() {}

pub fn services() {}
pub fn report_service() {}
pub fn get_service() {}
pub fn del_service() {}

pub fn vulns() {}
pub fn del_vuln() {}
pub fn report_vuln() {}
pub fn get_vuln() {}

pub fn workspaces() {}
pub fn current_workspace() {}
pub fn get_workspace() {}
pub fn set_workspace() {}
pub fn del_workspace() {}
pub fn add_workspace() {}

pub fn get_note() {}
pub fn report_note() {}
pub fn notes() {}
pub fn del_note() {}

pub fn get_client() {}
pub fn clients() {}
pub fn del_client() {}
pub fn report_client() {}

pub fn get_ref() {}

pub fn events() {}
pub fn report_event() {}

pub fn report_loot() {}
pub fn loots() {}

pub fn report_cred() {}
pub fn creds() {}

pub fn import_data() {}

pub fn driver() {}

pub fn connect() {}

pub fn status() {}

pub fn disconnect() {}

fn return<T:DOwned>(body:Vec<u8>) -> Result<T,E> {
    let con=connect::connect(body,buf);
