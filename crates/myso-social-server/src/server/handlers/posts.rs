// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::extract::{Path, Query, State};
use axum::Json;
use std::sync::Arc;

use crate::error::SocialError;

use super::super::{AppState, PageParams, PostsQuery};

pub async fn list_posts(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PostsQuery>,
) -> Result<Json<Vec<crate::reader::PostBasicRow>>, SocialError> {
    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params
        .offset
        .unwrap_or_else(|| (params.page.unwrap_or(1).max(1) - 1) * limit);
    let posts = state
        .reader
        .list_posts(
            params.owner.as_deref(),
            params.post_type.as_deref(),
            limit,
            offset,
        )
        .await?;
    Ok(Json(posts))
}

pub async fn get_post_config(
    State(state): State<Arc<AppState>>,
) -> Result<Json<crate::reader::PostConfigRow>, SocialError> {
    let config = state
        .reader
        .get_post_config()
        .await?
        .ok_or_else(|| SocialError::not_found("Post configuration".to_string()))?;
    Ok(Json(config))
}

pub async fn get_trending_posts(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PostBasicRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let posts = state.reader.get_trending_posts(limit, offset).await?;
    Ok(Json(posts))
}

pub async fn get_post_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<crate::reader::PostBasicRow>, SocialError> {
    let post = state
        .reader
        .get_post_by_id(&id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Post '{}'", id)))?;
    Ok(Json(post))
}

pub async fn get_post_comments(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::CommentRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let comments = state.reader.get_post_comments(&id, limit, offset).await?;
    Ok(Json(comments))
}

pub async fn get_post_reactions(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::ReactionRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let reactions = state.reader.get_post_reactions(&id, limit, offset).await?;
    Ok(Json(reactions))
}

pub async fn get_post_reposts(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::RepostRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let reposts = state.reader.get_post_reposts(&id, limit, offset).await?;
    Ok(Json(reposts))
}

pub async fn get_post_promotion(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<crate::reader::PromotedPostRow>, SocialError> {
    let promotion = state
        .reader
        .get_promotion_by_post_id(&id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Promotion for post '{}'", id)))?;
    Ok(Json(promotion))
}

pub async fn get_post_poc_badges(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PocBadgeRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let badges = state.reader.get_post_poc_badges(&id, limit, offset).await?;
    Ok(Json(badges))
}

pub async fn get_post_revenue_redirections(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PocRevenueRedirectionRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let redirections = state
        .reader
        .get_post_revenue_redirections(&id, limit, offset)
        .await?;
    Ok(Json(redirections))
}

pub async fn get_post_transfers(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PostTransfer>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let transfers = state.reader.list_post_transfers(&id, limit, offset).await?;
    Ok(Json(transfers))
}
