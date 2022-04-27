#[macro_export]
macro_rules! try_modification {
    ($ex:expr) => {
        match $ex {
            Modification::Err(err) => return Modification::Err(err),
            other => other,
        }
    };

    ($cursor:expr, $consume:ident) => {
        match $consume($cursor.clone()) {
            Modification::Err(err) => return Modification::Err(err),
            other => other,
        }
    };

    ($cursor:expr, $consume:ident($($arg:expr)*)) => {
        match $consume($($arg)*)($cursor.clone()) {
            Modification::Err(err) => return Modification::Err(err),
            other => other,
        }
    }
}

#[macro_export]
macro_rules! modify {
    ($ex:expr) => {
        match $ex {
            Modification::Ok(ok) => ok,
            Modification::Err(err) => return Modification::Err(err),
            Modification::None => return Modification::None,
        }
    };

    ($cursor:expr, $consume:ident) => {
        match $consume($cursor.clone()) {
            Modification::Ok(ok) => ok,
            Modification::Err(err) => return Modification::Err(err),
            Modification::None => return Modification::None,
        }
    };

    ($cursor:expr, $consume:ident($($arg:expr)*)) => {
        match $consume($($arg)*)($cursor.clone()) {
            Modification::Ok(ok) => ok,
            Modification::Err(err) => return Modification::Err(err),
            Modification::None => return Modification::None,
        }
    }
}
