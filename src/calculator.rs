use crate::proto::calculator_server::Calculator;
use crate::proto::{CalculationRequest, CalculationResponse};
use crate::state::State;

#[derive(Debug, Default)]
pub struct CalculatorService {
    pub state: State,
}

impl CalculatorService {
    async fn increment_counter(&self) {
        let mut count = self.state.write().await;
        *count += 1;
        println!("Request count: {}", *count);
    }
}

#[tonic::async_trait]
impl Calculator for CalculatorService {
    async fn add(
        &self,
        request: tonic::Request<CalculationRequest>,
    ) -> Result<tonic::Response<CalculationResponse>, tonic::Status> {
        self.increment_counter().await;
        let input = request.get_ref();

        let response = CalculationResponse {
            result: input.a + input.b,
        };

        Ok(tonic::Response::new(response))
    }

    async fn subtract(
        &self,
        request: tonic::Request<CalculationRequest>,
    ) -> Result<tonic::Response<CalculationResponse>, tonic::Status> {
        self.increment_counter().await;

        let input = request.get_ref();

        let response = CalculationResponse {
            result: input.a - input.b,
        };

        Ok(tonic::Response::new(response))
    }

    async fn divide(
        &self,
        request: tonic::Request<CalculationRequest>,
    ) -> Result<tonic::Response<CalculationResponse>, tonic::Status> {
        self.increment_counter().await;

        let input = request.get_ref();

        if input.b == 0 {
            return Err(tonic::Status::invalid_argument("cannot divide by zedo"));
        };

        let response = CalculationResponse {
            result: input.a / input.b,
        };

        Ok(tonic::Response::new(response))
    }

    async fn multiplicate(
        &self,
        request: tonic::Request<CalculationRequest>,
    ) -> Result<tonic::Response<CalculationResponse>, tonic::Status> {
        self.increment_counter().await;

        let input = request.get_ref();

        let response = CalculationResponse {
            result: input.a * input.b,
        };

        Ok(tonic::Response::new(response))
    }
}
