use common_lib::errors::DatabaseErrors;

pub type TResult<T> = Result<T, DatabaseErrors>;
