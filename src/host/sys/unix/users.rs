use crate::to_str;

pub fn get_users() -> Vec<String> {
    let mut users = Vec::new();
    unsafe {
        libc::setutxent();
        loop {
            let entry = libc::getutxent();
            if entry.is_null() {
                break;
            }

            if (*entry).ut_type != libc::USER_PROCESS {
                continue;
            }

            users.push(to_str((*entry).ut_user.as_ptr()).to_owned());
        }
    }

    users
}
