use crate::proto::admin_server::Admin;
use crate::proto::{CounterResponse, GetCountRequest};
use crate::state::State;

#[derive(Debug, Default)]
pub struct AdminService {
    pub state: State,
}

#[tonic::async_trait]
impl Admin for AdminService {
    async fn get_request_count(
        &self,
        _request: tonic::Request<GetCountRequest>,
    ) -> Result<tonic::Response<CounterResponse>, tonic::Status> {
        let count = self.state.read().await;
        let response = CounterResponse { count: *count };

        Ok(tonic::Response::new(response))
    }
}
