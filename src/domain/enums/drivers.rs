use core::fmt;
use std::str::FromStr;

pub enum DriverKind {
    Postgres,
    MSSQL,
    MySql,
    MongoDB,
    Sqlite,
}

impl fmt::Display for DriverKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            DriverKind::Postgres => "postgresql",
            DriverKind::MSSQL => "mssql",
            DriverKind::MySql => "mysql",
            DriverKind::MongoDB => "mongodb",
            DriverKind::Sqlite => "sqlite",
        };
        write!(f, "{}", output)
    }
}

impl FromStr for DriverKind {
    type Err = ();
    fn from_str(input: &str) -> Result<DriverKind, Self::Err> {
        match input {
            "postgresql" => Ok(DriverKind::Postgres),
            "mssql" => Ok(DriverKind::MSSQL),
            "mysql" => Ok(DriverKind::MySql),
            "mongodb" => Ok(DriverKind::MongoDB),
            "sqlite" => Ok(DriverKind::Sqlite),
            _ => Err(()),
        }
    }
}
