use chrono::{NaiveDateTime, Utc};
use diesel::Identifiable;
use log::{error, trace};
use models::NewAmusementPost;
use models::NullableIntArray;
use models::PostType;
use schema::Posts::comments_id;
use schema::Posts::images;
use schema::Posts::people_all;
use schema::Posts::sold;
use tonic::{Request, Response, Status};

use crate::codegen::amusement_post;
use crate::codegen::amusement_post::AmusementPost;
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
use crate::codegen::post::Post;
use crate::db::*;

use crate::db::models::NewComment;
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

        // get connection to Db
        let conn = &mut self.client.get_conn().map_err(|e| {
            error!("Fail to get connection to database: {e}");
            Status::internal("Fail to delete post")
        })?;

        // delete the post from Db and get the post
        let post_id = req.post_id;
        let the_post = delete_post(conn, post_id).map_err(|e| {
            error!("Fail to delete post {post_id}: {e}");
            Status::not_found("No such post")
        })?;

        // make response
        let response = DeletePostResponse { success: true };

        // delete images of the post
        let image_ids = the_post.images.0;
        for image_id in image_ids {
            if let Some(image_id) = image_id {
                let _delete_image_result = delete_image(image_id).map_err(|e| {
                    error!("Fail to delete image {image_id}: {e}");
                    // Status::not_found("No such image")
                });
                // delete image fail should not be reported to frontend
            }
        }

        Ok(Response::new(response))
    }

    async fn list_personal_posts(
        &self,
        request: tonic::Request<ListPersonalPostsRequest>,
    ) -> std::result::Result<tonic::Response<ListPersonalPostsResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("ListPersonalPost got request: {req:#?}");

        // get connection to Db
        let conn = &mut self.client.get_conn().map_err(|e| {
            error!("Fail to get connection to database: {e}");
            Status::internal("Fail to list personal post")
        })?;

        let the_user_id = req.user_id();
        let post_type = req.post_type();
        let request_type = req.r#type(); // own? star? takepart?
        let number = req.number;

        // need data struct about user
        todo!();
    }

    async fn comment(
        &self,
        request: tonic::Request<CommentRequest>,
    ) -> std::result::Result<tonic::Response<CommentResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("Comment got request: {req:#?}");

        // get connection to Db
        let conn = &mut self.client.get_conn().map_err(|e| {
            error!("Fail to get connection to database: {e}");
            Status::internal("Fail to comment")
        })?;

        // make insertable comment
        let comment = NewComment {
            post_id: req.post_id,
            user_id: req.user_id,
            content: req.content,
        };

        // insert and update post
        insert_comment_and_update_post(conn, &comment).map_err(|e| {
            error!("Fail to insert comment to database: {e}");
            Status::internal("Fail to comment")
        })?;

        let response = CommentResponse { success: true };
        Ok(Response::new(response))
    }

    async fn delete_comment(
        &self,
        request: tonic::Request<DeleteCommentRequest>,
    ) -> std::result::Result<tonic::Response<DeleteCommentResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("DeleteComment got request: {req:#?}");

        // get connection to Db
        let conn = &mut self.client.get_conn().map_err(|e| {
            error!("Fail to get connection to database: {e}");
            Status::internal("Fail to comment")
        })?;

        // delete the comment
        let comment_id_to_delete = req.comment_id;
        delete_comment_and_update_post(conn, comment_id_to_delete).map_err(|e| {
            error!("Fail to delete comment from database: {e}");
            Status::internal("Fail to delete comment")
        })?;

        let response = DeleteCommentResponse { success: true };

        Ok(Response::new(response))
    }

    async fn like_post(
        &self,
        request: tonic::Request<LikePostRequest>,
    ) -> std::result::Result<tonic::Response<LikePostResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("LikePost got request: {req:#?}");

        let the_user_id = req.user_id;
        let the_post_id = req.post_id;

        let conn = &mut self.client.get_conn().map_err(|e| {
            error!("Fail to get connection to database: {e}");
            Status::internal("Fail to comment")
        })?;

        like_post_by_id(conn, the_user_id, the_post_id).map_err(|e| {
            error!("Fail to like post from database: {e}");
            Status::internal("Fail to like post")
        })?;

        let response = LikePostResponse { success: true };
        Ok(Response::new(response))
    }

    async fn unlike_post(
        &self,
        request: tonic::Request<UnlikePostRequest>,
    ) -> std::result::Result<tonic::Response<UnlikePostResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("UnlikePost got request: {req:#?}");

        let the_user_id = req.user_id;
        let the_post_id = req.post_id;

        let conn = &mut self.client.get_conn().map_err(|e| {
            error!("Fail to get connection to database: {e}");
            Status::internal("Fail to comment")
        })?;

        unlike_post_by_id(conn, the_user_id, the_post_id).map_err(|e| {
            error!("Fail to unlike post from database: {e}");
            Status::internal("Fail to unlike post")
        })?;

        let response = UnlikePostResponse { success: true };
        Ok(Response::new(response))
    }

    async fn like_comment(
        &self,
        request: tonic::Request<LikeCommentRequest>,
    ) -> std::result::Result<tonic::Response<LikeCommentResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("LikeComment got request: {req:#?}");

        // need data struct about user, or change of 'Comment' data struct
        todo!();
    }

    async fn unlike_comment(
        &self,
        request: tonic::Request<UnlikeCommentRequest>,
    ) -> std::result::Result<tonic::Response<UnlikeCommentResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("UnlikeComment got request: {req:#?}");

        // need data struct about user, or change of 'Comment' data struct
        todo!();
    }

    async fn favorate(
        &self,
        request: tonic::Request<FavorateRequest>,
    ) -> std::result::Result<tonic::Response<FavorateResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("Favorate got request: {req:#?}");

        let the_user_id = req.user_id;
        let the_post_id = req.post_id;

        let conn = &mut self.client.get_conn().map_err(|e| {
            error!("Fail to get connection to database: {e}");
            Status::internal("Fail to comment")
        })?;

        favorate_post_by_id(conn, the_user_id, the_post_id).map_err(|e| {
            error!("Fail to favorate post from database: {e}");
            Status::internal("Fail to favorate post")
        })?;

        let response = FavorateResponse { success: true };
        Ok(Response::new(response))
    }

    async fn unfavorate(
        &self,
        request: tonic::Request<UnfavorateRequest>,
    ) -> std::result::Result<tonic::Response<UnfavorateResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("Unfavorate got request: {req:#?}");

        let the_user_id = req.user_id;
        let the_post_id = req.post_id;

        let conn = &mut self.client.get_conn().map_err(|e| {
            error!("Fail to get connection to database: {e}");
            Status::internal("Fail to comment")
        })?;

        unfavorate_post_by_id(conn, the_user_id, the_post_id).map_err(|e| {
            error!("Fail to unfavorate post from database: {e}");
            Status::internal("Fail to unfavorate post")
        })?;

        let response = UnfavorateResponse { success: true };
        Ok(Response::new(response))
    }

    // about amusement

    async fn create_amusement_post(
        &self,
        request: tonic::Request<CreateAmusementPostRequest>,
    ) -> std::result::Result<tonic::Response<CreatePostResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("CreateAmusementPost got request: {req:#?}");

        // get connection to Db
        let conn = &mut self.client.get_conn().map_err(|e| {
            error!("Fail to get connection to database: {e}");
            Status::internal("Fail to comment")
        })?;

        let new_post = models::Post::from_proto_amusement_post(req.post).map_err(|e| {
            error!("Fail to convert to amusement post: {e}");
            Status::internal("Fail to create amusement post")
        })?;

        let the_post = insert_amusement_post(conn, &new_post).map_err(|e| {
            error!("Fail to insert amusement post to database: {e}");
            Status::internal("Fail to create amusement post")
        })?;

        let response = CreatePostResponse {
            success: true,
            post_id: the_post.id,
            message: "".into(),
        };
        Ok(Response::new(response))
    }

    async fn get_amusement_post(
        &self,
        request: tonic::Request<GetPostRequest>,
    ) -> std::result::Result<tonic::Response<GetAmusementPostResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("GetAmusementPost got request: {req:#?}");

        // get connection to Db
        let conn = &mut self.client.get_conn().map_err(|e| {
            error!("Fail to get connection to database: {e}");
            Status::internal("Fail to comment")
        })?;

        let the_post_id = req.post_id;

        let the_post = query_post_by_id(conn, the_post_id).map_err(|e| {
            error!("Fail to get post from database: {e}");
            Status::internal("Fail to get post")
        })?;

        if the_post.post_type != models::PostType::AMUSEMENTPOST {
            error!("Fail to get post from database: Wrong post type");
            Err(Status::internal("Fail to get post of amusement post"))
        } else {
            let the_post = the_post.to_proto_amusement_post(conn).map_err(|e| {
                error!("Fail to get post from database: {e}");
                Status::internal("Fail to get post of amusement post")
            })?;
            let response = GetAmusementPostResponse {
                success: true,
                post: Some(the_post),
            };
            Ok(Response::new(response))
        }
    }

    async fn list_amusement_posts(
        &self,
        request: tonic::Request<ListAmusementPostsRequest>,
    ) -> std::result::Result<tonic::Response<ListAmusementPostsResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("ListAmusementPost got request: {req:#?}");

        // get connection to Db
        let conn = &mut self.client.get_conn().map_err(|e| {
            error!("Fail to get connection to database: {e}");
            Status::internal("Fail to comment")
        })?;

        let post_vec = query_and_filter_amusement_post(
            conn,
            if req.game_type.is_some() {
                Some(models::GameType::from_proto_type(&req.game_type()))
            //safe unwrap
            } else {
                None
            },
            req.people_all_lowbound,
            req.people_all_upbound,
            req.people_diff_upbound,
            if req.time_about.is_some() {
                Some(NaiveDateTime::from_timestamp(req.time_about.unwrap(), 0))
            //safe unwrap
            } else {
                None
            },
            req.number,
        )
        .map_err(|e| {
            error!("Fail to query from database: {e}");
            Status::internal("Fail get amusement posts")
        })?;

        let mut posts = vec![];
        for post in post_vec {
            let post = post.to_proto_amusement_post(conn).map_err(|e| {
                error!("Fail to convert to amusement post: {e}");
                Status::internal("Fail get amusement posts")
            })?;
            posts.push(post);
        }

        let response = ListAmusementPostsResponse { posts: posts };

        Ok(Response::new(response))
    }

    async fn take_part(
        &self,
        request: tonic::Request<TakePartAmusePostRequest>,
    ) -> std::result::Result<tonic::Response<TakePartAmusePostResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("TakePart got request: {req:#?}");

        let the_user_id = req.user_id;
        let the_post_id = req.post_id;

        let conn = &mut self.client.get_conn().map_err(|e| {
            error!("Fail to get connection to database: {e}");
            Status::internal("Fail to comment")
        })?;

        take_part_post_by_id(conn, the_user_id, the_post_id).map_err(|e| {
            error!("Fail to takepart: {e}");
            Status::internal("Fail to takepart")
        })?;

        let response = TakePartAmusePostResponse { success: true };

        Ok(Response::new(response))
    }

    async fn no_take_part(
        &self,
        request: tonic::Request<NoTakePartAmusePostRequest>,
    ) -> std::result::Result<tonic::Response<NoTakePartAmusePostResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("NoTakePart got request: {req:#?}");

        let the_user_id = req.user_id;
        let the_post_id = req.post_id;

        let conn = &mut self.client.get_conn().map_err(|e| {
            error!("Fail to get connection to database: {e}");
            Status::internal("Fail to comment")
        })?;

        no_take_part_post_by_id(conn, the_user_id, the_post_id).map_err(|e| {
            error!("Fail to no_takepart: {e}");
            Status::internal("Fail to no_takepart")
        })?;

        let response = NoTakePartAmusePostResponse { success: true };

        Ok(Response::new(response))
    }

    // about food

    async fn create_food_post(
        &self,
        request: tonic::Request<CreateFoodPostRequest>,
    ) -> std::result::Result<tonic::Response<CreatePostResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("CreateFoodPost got request: {req:#?}");

        // get connection to Db
        let conn = &mut self.client.get_conn().map_err(|e| {
            error!("Fail to get connection to database: {e}");
            Status::internal("Fail to create food post")
        })?;

        let new_post = models::Post::from_proto_food_post(req.post).map_err(|e| {
            error!("Fail to convert to food post: {e}");
            Status::internal("Fail to create food post")
        })?;

        let the_post = insert_food_post(conn, &new_post).map_err(|e| {
            error!("Fail to insert food post to database: {e}");
            Status::internal("Fail to create food post")
        })?;

        let response = CreatePostResponse {
            success: true,
            post_id: the_post.id,
            message: "".into(),
        };
        Ok(Response::new(response))
    }

    async fn get_food_post(
        &self,
        request: tonic::Request<GetPostRequest>,
    ) -> std::result::Result<tonic::Response<GetFoodPostResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("GetFoodPost got request: {req:#?}");

        // get connection to Db
        let conn = &mut self.client.get_conn().map_err(|e| {
            error!("Fail to get connection to database: {e}");
            Status::internal("Fail to comment")
        })?;

        let the_post_id = req.post_id;

        let the_post = query_post_by_id(conn, the_post_id).map_err(|e| {
            error!("Fail to get post from database: {e}");
            Status::internal("Fail to get post")
        })?;

        if the_post.post_type != models::PostType::FOODPOST {
            error!("Fail to get post from database: Wrong post type");
            Err(Status::internal("Fail to get post of food post"))
        } else {
            let the_post = the_post.to_proto_food_post(conn).map_err(|e| {
                error!("Fail to get post from database: {e}");
                Status::internal("Fail to get post of food post")
            })?;
            let response = GetFoodPostResponse {
                success: true,
                post: Some(the_post),
            };
            Ok(Response::new(response))
        }
    }

    async fn list_food_posts(
        &self,
        request: tonic::Request<ListFoodPostsRequest>,
    ) -> std::result::Result<tonic::Response<ListFoodPostsResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("ListFoodPost got request: {req:#?}");

        // get connection to Db
        let conn = &mut self.client.get_conn().map_err(|e| {
            error!("Fail to get connection to database: {e}");
            Status::internal("Fail to comment")
        })?;

        let post_vec = query_and_filter_food_post(
            conn,
            if req.food_place.is_some() {
                Some(models::Place::from_proto_type(&req.food_place()))
            } else {
                None
            },
            req.score_lowbond,
            req.random,
            req.number,
        )
        .map_err(|e| {
            error!("Fail to query from database: {e}");
            Status::internal("Fail get food posts")
        })?;

        let mut posts = vec![];
        for post in post_vec {
            let post = post.to_proto_food_post(conn).map_err(|e| {
                error!("Fail to convert to food post: {e}");
                Status::internal("Fail get food posts")
            })?;
            posts.push(post);
        }

        let response = ListFoodPostsResponse { posts: posts };

        Ok(Response::new(response))
    }

    // about sell

    async fn create_sell_post(
        &self,
        request: tonic::Request<CreateSellPostRequest>,
    ) -> std::result::Result<tonic::Response<CreatePostResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("CreateSellPost got request: {req:#?}");

        // get connection to Db
        let conn = &mut self.client.get_conn().map_err(|e| {
            error!("Fail to get connection to database: {e}");
            Status::internal("Fail to comment")
        })?;

        let new_post = models::Post::from_proto_sell_post(req.post).map_err(|e| {
            error!("Fail to convert to sell post: {e}");
            Status::internal("Fail to create sell post")
        })?;

        let the_post = insert_sell_post(conn, &new_post).map_err(|e| {
            error!("Fail to insert sell post to database: {e}");
            Status::internal("Fail to create sell post")
        })?;

        let response = CreatePostResponse {
            success: true,
            post_id: the_post.id,
            message: "".into(),
        };
        Ok(Response::new(response))
    }

    async fn get_sell_post(
        &self,
        request: tonic::Request<GetPostRequest>,
    ) -> std::result::Result<tonic::Response<GetSellPostResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("GetSellPost got request: {req:#?}");

        // get connection to Db
        let conn = &mut self.client.get_conn().map_err(|e| {
            error!("Fail to get connection to database: {e}");
            Status::internal("Fail to comment")
        })?;

        let the_post_id = req.post_id;

        let the_post = query_post_by_id(conn, the_post_id).map_err(|e| {
            error!("Fail to get post from database: {e}");
            Status::internal("Fail to get post")
        })?;

        if the_post.post_type != models::PostType::SELLPOST {
            error!("Fail to get post from database: Wrong post type");
            Err(Status::internal("Fail to get post of sell post"))
        } else {
            let the_post = the_post.to_proto_sell_post(conn).map_err(|e| {
                error!("Fail to get post from database: {e}");
                Status::internal("Fail to get post of sell post")
            })?;
            let response = GetSellPostResponse {
                success: true,
                post: Some(the_post),
            };
            Ok(Response::new(response))
        }
    }

    async fn list_sell_posts(
        &self,
        request: tonic::Request<ListSellPostsRequest>,
    ) -> std::result::Result<tonic::Response<ListSellPostsResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("ListSellPost got request: {req:#?}");

        // get connection to Db
        let conn = &mut self.client.get_conn().map_err(|e| {
            error!("Fail to get connection to database: {e}");
            Status::internal("Fail to comment")
        })?;

        let post_vec = query_and_filter_sell_post(
            conn,
            if req.goods_type.is_some() {
                Some(models::GoodsType::from_proto_type(&req.goods_type()))
            } else {
                None
            },
            req.price_upbond,
            req.number,
        )
        .map_err(|e| {
            error!("Fail to query from database: {e}");
            Status::internal("Fail get food posts")
        })?;

        let mut posts = vec![];
        for post in post_vec {
            let post = post.to_proto_sell_post(conn).map_err(|e| {
                error!("Fail to convert to food post: {e}");
                Status::internal("Fail get food posts")
            })?;
            posts.push(post);
        }

        let response = ListSellPostsResponse { posts: posts };

        Ok(Response::new(response))
    }

    async fn set_sold(
        &self,
        request: tonic::Request<SetSoldRequest>,
    ) -> std::result::Result<tonic::Response<SetSoldResponse>, tonic::Status> {
        let req = request.into_inner();
        trace!("SetSold got request: {req:#?}");

        // get connection to Db
        let conn = &mut self.client.get_conn().map_err(|e| {
            error!("Fail to get connection to database: {e}");
            Status::internal("Fail to comment")
        })?;
        let post_id = req.post_id;

        set_sold_for_sell_post_by_id(conn, post_id).map_err(|e| {
            error!("Fail to convert to food post: {e}");
            Status::internal("Fail get food posts")
        })?;

        let response = SetSoldResponse { success: true };
        Ok(Response::new(response))
    }
}
