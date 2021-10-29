use tonic::{Request, Response, Status};

use self::scene_items_server::SceneItems;

tonic::include_proto!("obs_remote.scene_items");

pub struct Service;

#[tonic::async_trait]
impl SceneItems for Service {
    async fn list(&self, request: Request<ListRequest>) -> Result<Response<ListReply>, Status> {
        let scene_name = request.into_inner().scene_name;

        Ok(Response::new(ListReply {
            scene_name,
            items: vec![],
        }))
    }

    async fn get_properties(
        &self,
        request: Request<GetPropertiesRequest>,
    ) -> Result<Response<GetPropertiesReply>, Status> {
        let scene_name = request.into_inner().scene_name;

        Ok(Response::new(GetPropertiesReply {
            name: scene_name,
            id: 1,
            properties: None,
        }))
    }

    async fn set_properties(
        &self,
        request: Request<SetPropertiesRequest>,
    ) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }

    async fn reset(&self, request: Request<ResetRequest>) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }

    async fn delete(&self, request: Request<DeleteRequest>) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }

    async fn add(&self, request: Request<AddRequest>) -> Result<Response<AddReply>, Status> {
        Ok(Response::new(AddReply { id: 1 }))
    }

    async fn duplicate(
        &self,
        request: Request<DuplicateRequest>,
    ) -> Result<Response<DuplicateReply>, Status> {
        let request = request.into_inner();

        Ok(Response::new(DuplicateReply {
            scene: request.to_scene,
            specification: Some(SceneItemSpecification {
                name: "test".to_owned(),
                id: 1,
            }),
        }))
    }
}
