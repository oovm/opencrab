import { defineStore } from "pinia";
import { ForumService } from "../services";
import type { ForumPost, ForumComment } from "../types";
import { ref } from "vue";

export const useForumStore = defineStore("forum", () => {
    const posts = ref<ForumPost[]>(ForumService.getPosts());
    const comments = ref<Record<string, ForumComment[]>>({});

    function fetchPosts() {
        posts.value = ForumService.getPosts();
    }

    function getPostById(id: string): ForumPost | undefined {
        return posts.value.find((post) => post.id === id);
    }

    function fetchCommentsByPostId(postId: string) {
        comments.value[postId] = ForumService.getCommentsByPostId(postId);
    }

    function getCommentsByPostId(postId: string): ForumComment[] {
        if (!comments.value[postId]) {
            fetchCommentsByPostId(postId);
        }
        return comments.value[postId] || [];
    }

    function createPost(
        postData: Omit<ForumPost, "id" | "views" | "likes" | "commentCount" | "createdAt" | "updatedAt">,
    ) {
        const newPost = ForumService.createPost(postData);
        posts.value.unshift(newPost);
        return newPost;
    }

    function createComment(commentData: Omit<ForumComment, "id" | "likes" | "createdAt">) {
        const newComment = ForumService.createComment(commentData);
        if (!comments.value[commentData.postId]) {
            comments.value[commentData.postId] = [];
        }
        comments.value[commentData.postId].push(newComment);

        const post = posts.value.find((p) => p.id === commentData.postId);
        if (post) {
            post.commentCount++;
        }

        return newComment;
    }

    function incrementViews(postId: string) {
        ForumService.incrementViews(postId);
        const post = posts.value.find((p) => p.id === postId);
        if (post) {
            post.views++;
        }
    }

    function likePost(postId: string) {
        ForumService.likePost(postId);
        const post = posts.value.find((p) => p.id === postId);
        if (post) {
            post.likes++;
        }
    }

    function likeComment(commentId: string, postId: string) {
        ForumService.likeComment(commentId);
        const postComments = comments.value[postId];
        if (postComments) {
            const comment = postComments.find((c) => c.id === commentId);
            if (comment) {
                comment.likes++;
            }
        }
    }

    return {
        posts,
        comments,
        fetchPosts,
        getPostById,
        fetchCommentsByPostId,
        getCommentsByPostId,
        createPost,
        createComment,
        incrementViews,
        likePost,
        likeComment,
    };
});
