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
        todo!()
    }

    async fn delete_post(
        &self,
        request: Request<DeletePostRequest>,
    ) -> Result<Response<DeletePostResponse>, Status> {
        todo!()
    }

    async fn get_post(
        &self,
        request: Request<GetPostRequest>,
    ) -> Result<Response<GetPostResponse>, Status> {
        todo!()
    }

    async fn list_posts(
        &self,
        request: Request<ListPostsRequest>,
    ) -> Result<Response<ListPostsResponse>, Status> {
        todo!()
    }

    async fn comment(
        &self,
        request: Request<CommentRequest>,
    ) -> Result<Response<CommentResponse>, Status> {
        todo!()
    }

    async fn delete_comment(
        &self,
        request: Request<DeleteCommentRequest>,
    ) -> Result<Response<DeleteCommentResponse>, Status> {
        todo!()
    }

    async fn like(&self, request: Request<LikeRequest>) -> Result<Response<LikeResponse>, Status> {
        todo!()
    }

    async fn unlike(
        &self,
        request: Request<UnlikeRequest>,
    ) -> Result<Response<UnlikeResponse>, Status> {
        todo!()
    }

    async fn favorate(
        &self,
        request: Request<FavorateRequest>,
    ) -> Result<Response<FavorateResponse>, Status> {
        todo!()
    }

    async fn unfavorate(
        &self,
        request: Request<UnfavorateRequest>,
    ) -> Result<Response<UnfavorateResponse>, Status> {
        todo!()
    }
}
