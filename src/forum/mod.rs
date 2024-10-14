use log::{error, trace};
use tonic::{Request, Response, Status};

use crate::codegen::forum::forum_server::Forum;
use crate::codegen::forum::CreateAmusementPostRequest;
use crate::codegen::forum::CreateFoodPostRequest;
use crate::codegen::forum::CreatePostResponse;
use crate::codegen::forum::CreateSellPostRequest;
use crate::codegen::forum::GetAmusementPostResponse;
use crate::codegen::forum::GetFoodPostResponse;
use crate::codegen::forum::GetPostRequest;
use crate::codegen::forum::GetSellPostResponse;
use crate::codegen::forum::{CommentRequest, CommentResponse};
use crate::codegen::forum::{DeleteCommentRequest, DeleteCommentResponse};
use crate::codegen::forum::{DeletePostRequest, DeletePostResponse};
use crate::codegen::forum::{FavorateRequest, FavorateResponse};
use crate::codegen::forum::{LikeCommentRequest, LikeCommentResponse};
use crate::codegen::forum::{LikePostRequest, LikePostResponse};
use crate::codegen::forum::{ListAmusementPostsRequest, ListAmusementPostsResponse};
use crate::codegen::forum::{ListFoodPostsRequest, ListFoodPostsResponse};
use crate::codegen::forum::{ListPersonalPostsRequest, ListPersonalPostsResponse};
use crate::codegen::forum::{ListSellPostsRequest, ListSellPostsResponse};
use crate::codegen::forum::{NoTakePartAmusePostRequest, NoTakePartAmusePostResponse};
use crate::codegen::forum::{SetSoldRequest, SetSoldResponse};
use crate::codegen::forum::{TakePartAmusePostRequest, TakePartAmusePostResponse};
use crate::codegen::forum::{UnfavorateRequest, UnfavorateResponse};
use crate::codegen::forum::{UnlikeCommentRequest, UnlikeCommentResponse};
use crate::codegen::forum::{UnlikePostRequest, UnlikePostResponse};

use crate::db::models::Post;
use crate::db::schema::Comments::user_id;
use crate::db::DBClient;

#[derive(Debug)]
pub struct ForumService {
    pub client: DBClient,
}

#[tonic::async_trait]
impl Forum for ForumService {
    async fn delete_post(
        &self,
        request: tonic::Request<DeletePostRequest>,
    ) -> std::result::Result<tonic::Response<DeletePostResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("DeletePost got request: {req:#?}");

        todo!();
    }

    async fn list_personal_posts(
        &self,
        request: tonic::Request<ListPersonalPostsRequest>,
    ) -> std::result::Result<tonic::Response<ListPersonalPostsResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("ListPersonalPost got request: {req:#?}");

        todo!();
    }

    async fn comment(
        &self,
        request: tonic::Request<CommentRequest>,
    ) -> std::result::Result<tonic::Response<CommentResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("Comment got request: {req:#?}");

        todo!();
    }

    async fn delete_comment(
        &self,
        request: tonic::Request<DeleteCommentRequest>,
    ) -> std::result::Result<tonic::Response<DeleteCommentResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("DeleteComment got request: {req:#?}");

        todo!();
    }

    async fn like_post(
        &self,
        request: tonic::Request<LikePostRequest>,
    ) -> std::result::Result<tonic::Response<LikePostResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("LikePost got request: {req:#?}");

        todo!();
    }

    async fn unlike_post(
        &self,
        request: tonic::Request<UnlikePostRequest>,
    ) -> std::result::Result<tonic::Response<UnlikePostResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("UnlikePost got request: {req:#?}");

        todo!();
    }

    async fn like_comment(
        &self,
        request: tonic::Request<LikeCommentRequest>,
    ) -> std::result::Result<tonic::Response<LikeCommentResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("LikeComment got request: {req:#?}");

        todo!();
    }

    async fn unlike_comment(
        &self,
        request: tonic::Request<UnlikeCommentRequest>,
    ) -> std::result::Result<tonic::Response<UnlikeCommentResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("UnlikeComment got request: {req:#?}");

        todo!();
    }

    async fn favorate(
        &self,
        request: tonic::Request<FavorateRequest>,
    ) -> std::result::Result<tonic::Response<FavorateResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("Favorate got request: {req:#?}");

        todo!();
    }

    async fn unfavorate(
        &self,
        request: tonic::Request<UnfavorateRequest>,
    ) -> std::result::Result<tonic::Response<UnfavorateResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("Unfavorate got request: {req:#?}");

        todo!();
    }

    // about amusement

    async fn create_amusement_post(
        &self,
        request: tonic::Request<CreateAmusementPostRequest>,
    ) -> std::result::Result<tonic::Response<CreatePostResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("CreateAmusementPost got request: {req:#?}");

        todo!();
    }
    async fn get_amusement_post(
        &self,
        request: tonic::Request<GetPostRequest>,
    ) -> std::result::Result<tonic::Response<GetAmusementPostResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("GetAmusementPost got request: {req:#?}");

        todo!();
    }
    async fn list_amusement_posts(
        &self,
        request: tonic::Request<ListAmusementPostsRequest>,
    ) -> std::result::Result<tonic::Response<ListAmusementPostsResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("ListAmusementPost got request: {req:#?}");

        todo!();
    }
    async fn take_part(
        &self,
        request: tonic::Request<TakePartAmusePostRequest>,
    ) -> std::result::Result<tonic::Response<TakePartAmusePostResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("TakePart got request: {req:#?}");

        todo!();
    }

    async fn no_take_part(
        &self,
        request: tonic::Request<NoTakePartAmusePostRequest>,
    ) -> std::result::Result<tonic::Response<NoTakePartAmusePostResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("NoTakePart got request: {req:#?}");

        todo!();
    }

    // about food

    async fn create_food_post(
        &self,
        request: tonic::Request<CreateFoodPostRequest>,
    ) -> std::result::Result<tonic::Response<CreatePostResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("CreateFoodPost got request: {req:#?}");

        todo!();
    }
    async fn get_food_post(
        &self,
        request: tonic::Request<GetPostRequest>,
    ) -> std::result::Result<tonic::Response<GetFoodPostResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("GetFoodPost got request: {req:#?}");

        todo!();
    }
    async fn list_food_posts(
        &self,
        request: tonic::Request<ListFoodPostsRequest>,
    ) -> std::result::Result<tonic::Response<ListFoodPostsResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("ListFoodPost got request: {req:#?}");

        todo!();
    }

    // about sell

    async fn create_sell_post(
        &self,
        request: tonic::Request<CreateSellPostRequest>,
    ) -> std::result::Result<tonic::Response<CreatePostResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("CreateSellPost got request: {req:#?}");

        todo!();
    }
    async fn get_sell_post(
        &self,
        request: tonic::Request<GetPostRequest>,
    ) -> std::result::Result<tonic::Response<GetSellPostResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("GetSellPost got request: {req:#?}");

        todo!();
    }
    async fn list_sell_posts(
        &self,
        request: tonic::Request<ListSellPostsRequest>,
    ) -> std::result::Result<tonic::Response<ListSellPostsResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("ListSellPost got request: {req:#?}");

        todo!();
    }
    async fn set_sold(
        &self,
        request: tonic::Request<SetSoldRequest>,
    ) -> std::result::Result<tonic::Response<SetSoldResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("SetSold got request: {req:#?}");

        todo!();
    }
}
