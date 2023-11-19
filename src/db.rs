use crate::conf::Conf;

pub fn get_db_conn_str(conf: Conf) -> String {
    let conn_string = format!(
        "postgres://{}:{}@{}/{}",
        conf.db_user, conf.db_password, conf.db_host, conf.db_name
    );

    return conn_string;
}
