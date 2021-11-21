#[derive(Debug)]
pub struct FailAuth;

impl warp::reject::Reject for FailAuth {}

#[derive(Debug)]
pub struct InvalidID;

impl warp::reject::Reject for InvalidID {}

#[derive(Debug)]
pub struct ServerFailure;

impl warp::reject::Reject for ServerFailure {}

#[derive(Debug)]
pub struct NotFound;

impl warp::reject::Reject for NotFound {}
