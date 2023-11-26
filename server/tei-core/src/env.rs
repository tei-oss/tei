use std::env;

#[must_use]
pub fn get(name: &str) -> Option<String> {
    match env::var(name) {
        Ok(v) => Some(v),
        Err(e) => match e {
            env::VarError::NotPresent => None,
            env::VarError::NotUnicode(os_value) => {
                panic!("requested env variable {name} is not unicode. actual value: {os_value:?}")
            }
        },
    }
}

#[must_use]
pub fn get_u16(name: &str) -> Option<u16> {
    get(name).map(|v| {
        v.parse()
            .unwrap_or_else(|_| panic!("expected {name} to be u16 but found {v}"))
    })
}
