use log::trace;
use tonic::{Request, Response, Status};

use crate::codegen::forum::forum_server::Forum;
use crate::codegen::forum::{CommentRequest, CommentResponse};
use crate::codegen::forum::{CreatePostRequest, CreatePostResponse};
use crate::codegen::forum::{DeleteCommentRequest, DeleteCommentResponse};
use crate::codegen::forum::{DeletePostRequest, DeletePostResponse};
use crate::codegen::forum::{FavorateRequest, FavorateResponse};
use crate::codegen::forum::{GetPostRequest, GetPostResponse};
use crate::codegen::forum::{LikeRequest, LikeResponse};
use crate::codegen::forum::{ListPostsRequest, ListPostsResponse};
use crate::codegen::forum::{UnfavorateRequest, UnfavorateResponse};
use crate::codegen::forum::{UnlikeRequest, UnlikeResponse};
use crate::db::DBClient;

#[derive(Debug)]
pub struct ForumService {
    pub client: DBClient,
}

#[tonic::async_trait]
impl Forum for ForumService {
    async fn create_post(
        &self,
        request: Request<CreatePostRequest>,
    ) -> Result<Response<CreatePostResponse>, Status> {
        let req = request.into_inner();
        trace!("CreatePost got request: {req:#?}");

        let response = CreatePostResponse {
            success: true,
            post_id: 0,
            message: "post create succeeded".into(),
        };
        Ok(Response::new(response))
    }

    async fn delete_post(
        &self,
        request: Request<DeletePostRequest>,
    ) -> Result<Response<DeletePostResponse>, Status> {
        let req = request.into_inner();
        trace!("DeletePost got request: {req:#?}");

        todo!()
    }

    async fn get_post(
        &self,
        request: Request<GetPostRequest>,
    ) -> Result<Response<GetPostResponse>, Status> {
        let req = request.into_inner();
        trace!("GetPost got request: {req:#?}");

        todo!()
    }

    async fn list_posts(
        &self,
        request: Request<ListPostsRequest>,
    ) -> Result<Response<ListPostsResponse>, Status> {
        let req = request.into_inner();
        trace!("ListPosts got request: {req:#?}");

        todo!()
    }

    async fn comment(
        &self,
        request: Request<CommentRequest>,
    ) -> Result<Response<CommentResponse>, Status> {
        let req = request.into_inner();
        trace!("Comment got request: {req:#?}");

        todo!()
    }

    async fn delete_comment(
        &self,
        request: Request<DeleteCommentRequest>,
    ) -> Result<Response<DeleteCommentResponse>, Status> {
        let req = request.into_inner();
        trace!("DeleteComment got request: {req:#?}");

        todo!()
    }

    async fn like(&self, request: Request<LikeRequest>) -> Result<Response<LikeResponse>, Status> {
        let req = request.into_inner();
        trace!("Like got request: {req:#?}");

        todo!()
    }

    async fn unlike(
        &self,
        request: Request<UnlikeRequest>,
    ) -> Result<Response<UnlikeResponse>, Status> {
        let req = request.into_inner();
        trace!("Unlike got request: {req:#?}");

        todo!()
    }

    async fn favorate(
        &self,
        request: Request<FavorateRequest>,
    ) -> Result<Response<FavorateResponse>, Status> {
        let req = request.into_inner();
        trace!("Favorate got request: {req:#?}");

        todo!()
    }

    async fn unfavorate(
        &self,
        request: Request<UnfavorateRequest>,
    ) -> Result<Response<UnfavorateResponse>, Status> {
        let req = request.into_inner();
        trace!("Unfavorate got request: {req:#?}");

        todo!()
    }
}
