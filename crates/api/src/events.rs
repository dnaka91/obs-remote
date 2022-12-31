use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

pub use self::events_service_server::EventsServiceServer;

tonic::include_proto!("events.v1");

pub struct EventsService;

#[tonic::async_trait]
impl events_service_server::EventsService for EventsService {
    type AllStream = ReceiverStream<Result<AllResponse, Status>>;
    type ConfigStream = ReceiverStream<Result<ConfigResponse, Status>>;
    type FiltersStream = ReceiverStream<Result<FiltersResponse, Status>>;
    type GeneralStream = ReceiverStream<Result<GeneralResponse, Status>>;
    type HighVolumeStream = ReceiverStream<Result<HighVolumeResponse, Status>>;
    type InputsStream = ReceiverStream<Result<InputsResponse, Status>>;
    type MediaInputsStream = ReceiverStream<Result<MediaInputsResponse, Status>>;
    type OutputsStream = ReceiverStream<Result<OutputsResponse, Status>>;
    type SceneItemsStream = ReceiverStream<Result<SceneItemsResponse, Status>>;
    type ScenesStream = ReceiverStream<Result<ScenesResponse, Status>>;
    type TransitionsStream = ReceiverStream<Result<TransitionsResponse, Status>>;

    async fn all(&self, request: Request<AllRequest>) -> Result<Response<Self::AllStream>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn general(
        &self,
        request: Request<GeneralRequest>,
    ) -> Result<Response<Self::GeneralStream>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn config(
        &self,
        request: Request<ConfigRequest>,
    ) -> Result<Response<Self::ConfigStream>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn scenes(
        &self,
        request: Request<ScenesRequest>,
    ) -> Result<Response<Self::ScenesStream>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn inputs(
        &self,
        request: Request<InputsRequest>,
    ) -> Result<Response<Self::InputsStream>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn transitions(
        &self,
        request: Request<TransitionsRequest>,
    ) -> Result<Response<Self::TransitionsStream>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn filters(
        &self,
        request: Request<FiltersRequest>,
    ) -> Result<Response<Self::FiltersStream>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn outputs(
        &self,
        request: Request<OutputsRequest>,
    ) -> Result<Response<Self::OutputsStream>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn scene_items(
        &self,
        request: Request<SceneItemsRequest>,
    ) -> Result<Response<Self::SceneItemsStream>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn media_inputs(
        &self,
        request: Request<MediaInputsRequest>,
    ) -> Result<Response<Self::MediaInputsStream>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn high_volume(
        &self,
        request: Request<HighVolumeRequest>,
    ) -> Result<Response<Self::HighVolumeStream>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
