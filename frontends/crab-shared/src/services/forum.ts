import type { ForumPost, ForumComment } from "../types";
import { Storage } from "./mock/storage";

function uuidv4() {
    return "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(/[xy]/g, function (c) {
        const r = (Math.random() * 16) | 0,
            v = c == "x" ? r : (r & 0x3) | 0x8;
        return v.toString(16);
    });
}

export class ForumService {
    /**
     * 获取所有帖子
     */
    static getPosts(): ForumPost[] {
        return Storage.getForumPosts();
    }

    /**
     * 根据 ID 获取帖子
     */
    static getPostById(id: string): ForumPost | undefined {
        return Storage.getForumPosts().find((post) => post.id === id);
    }

    /**
     * 获取帖子的所有评论
     */
    static getCommentsByPostId(postId: string): ForumComment[] {
        return Storage.getForumComments().filter((comment) => comment.postId === postId);
    }

    /**
     * 创建新帖子
     */
    static createPost(
        postData: Omit<ForumPost, "id" | "views" | "likes" | "commentCount" | "createdAt" | "updatedAt">,
    ): ForumPost {
        const now = new Date();
        const newPost: ForumPost = {
            ...postData,
            id: uuidv4(),
            views: 0,
            likes: 0,
            commentCount: 0,
            isPinned: false,
            isFeatured: false,
            createdAt: now,
            updatedAt: now,
        };
        const posts = Storage.getForumPosts();
        posts.unshift(newPost);
        Storage.setForumPosts(posts);
        return newPost;
    }

    /**
     * 创建新评论
     */
    static createComment(commentData: Omit<ForumComment, "id" | "likes" | "createdAt">): ForumComment {
        const now = new Date();
        const newComment: ForumComment = {
            ...commentData,
            id: uuidv4(),
            likes: 0,
            createdAt: now,
        };
        const comments = Storage.getForumComments();
        comments.push(newComment);
        Storage.setForumComments(comments);

        const posts = Storage.getForumPosts();
        const postIndex = posts.findIndex((p) => p.id === commentData.postId);
        if (postIndex !== -1) {
            posts[postIndex].commentCount++;
            Storage.setForumPosts(posts);
        }

        return newComment;
    }

    /**
     * 增加帖子浏览量
     */
    static incrementViews(postId: string): void {
        const posts = Storage.getForumPosts();
        const post = posts.find((p) => p.id === postId);
        if (post) {
            post.views++;
            Storage.setForumPosts(posts);
        }
    }

    /**
     * 点赞帖子
     */
    static likePost(postId: string): void {
        const posts = Storage.getForumPosts();
        const post = posts.find((p) => p.id === postId);
        if (post) {
            post.likes++;
            Storage.setForumPosts(posts);
        }
    }

    /**
     * 点赞评论
     */
    static likeComment(commentId: string): void {
        const comments = Storage.getForumComments();
        const comment = comments.find((c) => c.id === commentId);
        if (comment) {
            comment.likes++;
            Storage.setForumComments(comments);
        }
    }
}
