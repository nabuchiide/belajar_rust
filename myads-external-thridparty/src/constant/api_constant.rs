use constcat::concat;

const PATH_API_ROUTE: &str = "/svc";
const PATH_API_ROOT: &str = "/ext";
const PATH_API_MODULE: &str = "/client";
const PATH_API_VERSION: &str = "/v1";
pub(crate) const PATH_API_BASE: &str = concat!( PATH_API_ROUTE, PATH_API_ROOT, PATH_API_MODULE, PATH_API_VERSION);


