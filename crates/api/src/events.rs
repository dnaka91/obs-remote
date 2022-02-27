use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

pub use self::events_server::EventsServer;

tonic::include_proto!("obs_remote.events");

pub struct EventsService;

#[tonic::async_trait]
impl events_server::Events for EventsService {
    type AllStream = ReceiverStream<Result<Event, Status>>;

    async fn all(&self, request: Request<()>) -> Result<Response<Self::AllStream>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    type GeneralStream = ReceiverStream<Result<GeneralEvent, Status>>;

    async fn general(&self, request: Request<()>) -> Result<Response<Self::GeneralStream>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    type ConfigStream = ReceiverStream<Result<ConfigEvent, Status>>;

    async fn config(&self, request: Request<()>) -> Result<Response<Self::ConfigStream>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    type ScenesStream = ReceiverStream<Result<ScenesEvent, Status>>;

    async fn scenes(&self, request: Request<()>) -> Result<Response<Self::ScenesStream>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    type InputsStream = ReceiverStream<Result<InputsEvent, Status>>;

    async fn inputs(&self, request: Request<()>) -> Result<Response<Self::InputsStream>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    type TransitionsStream = ReceiverStream<Result<TransitionsEvent, Status>>;

    async fn transitions(
        &self,
        request: Request<()>,
    ) -> Result<Response<Self::TransitionsStream>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    type FiltersStream = ReceiverStream<Result<FiltersEvent, Status>>;

    async fn filters(&self, request: Request<()>) -> Result<Response<Self::FiltersStream>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    type OutputsStream = ReceiverStream<Result<OutputsEvent, Status>>;

    async fn outputs(&self, request: Request<()>) -> Result<Response<Self::OutputsStream>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    type SceneItemsStream = ReceiverStream<Result<SceneItemsEvent, Status>>;

    async fn scene_items(
        &self,
        request: Request<()>,
    ) -> Result<Response<Self::SceneItemsStream>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    type MediaInputsStream = ReceiverStream<Result<MediaInputsEvent, Status>>;

    async fn media_inputs(
        &self,
        request: Request<()>,
    ) -> Result<Response<Self::MediaInputsStream>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    type HighVolumeStream = ReceiverStream<Result<HighVolumeEvent, Status>>;

    async fn high_volume(
        &self,
        request: Request<()>,
    ) -> Result<Response<Self::HighVolumeStream>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
