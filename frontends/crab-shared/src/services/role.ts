import type { Role, Skill, Rating, Review, Tag, UserTag } from "../types";
import { Storage } from "./mock/storage";

function uuidv4() {
    return "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(/[xy]/g, function (c) {
        const r = (Math.random() * 16) | 0,
            v = c == "x" ? r : (r & 0x3) | 0x8;
        return v.toString(16);
    });
}

export class RoleService {
    /**
     * 获取所有角色
     */
    static getRoles(): Role[] {
        return Storage.getRoles();
    }

    /**
     * 根据 ID 获取角色
     */
    static getRoleById(id: string): Role | undefined {
        return Storage.getRoles().find((role) => role.id === id);
    }

    /**
     * 创建新角色
     */
    static createRole(roleData: Omit<Role, "id" | "createdAt" | "updatedAt">): Role {
        const now = new Date();
        const newRole: Role = {
            ...roleData,
            id: uuidv4(),
            createdAt: now,
            updatedAt: now,
        };
        const roles = Storage.getRoles();
        roles.push(newRole);
        Storage.setRoles(roles);
        return newRole;
    }

    /**
     * 更新角色
     */
    static updateRole(id: string, roleData: Partial<Omit<Role, "id" | "createdAt">>): Role | undefined {
        const roles = Storage.getRoles();
        const index = roles.findIndex((role) => role.id === id);
        if (index === -1) return undefined;

        roles[index] = {
            ...roles[index],
            ...roleData,
            updatedAt: new Date(),
        };
        Storage.setRoles(roles);
        return roles[index];
    }

    /**
     * 删除角色
     */
    static deleteRole(id: string): boolean {
        const roles = Storage.getRoles();
        const initialLength = roles.length;
        const filteredRoles = roles.filter((role) => role.id !== id);
        Storage.setRoles(filteredRoles);
        return filteredRoles.length !== initialLength;
    }

    /**
     * 获取所有技能
     */
    static getSkills(): Skill[] {
        return Storage.getSkills();
    }

    /**
     * 创建新技能
     */
    static createSkill(skillData: Omit<Skill, "id">): Skill {
        const newSkill: Skill = {
            ...skillData,
            id: uuidv4(),
        };
        const skills = Storage.getSkills();
        skills.push(newSkill);
        Storage.setSkills(skills);
        return newSkill;
    }

    /**
     * 获取角色的评分
     */
    static getRatingsForRole(roleId: string): Rating[] {
        return Storage.getRatings().filter((rating) => rating.roleId === roleId);
    }

    /**
     * 创建评分
     */
    static createRating(ratingData: Omit<Rating, "id" | "createdAt">): Rating {
        const newRating: Rating = {
            ...ratingData,
            id: uuidv4(),
            createdAt: new Date(),
        };
        const ratings = Storage.getRatings();
        ratings.push(newRating);
        Storage.setRatings(ratings);
        return newRating;
    }

    /**
     * 获取角色的评论
     */
    static getReviewsForRole(roleId: string): Review[] {
        return Storage.getReviews().filter((review) => review.roleId === roleId);
    }

    /**
     * 创建评论
     */
    static createReview(reviewData: Omit<Review, "id" | "createdAt">): Review {
        const newReview: Review = {
            ...reviewData,
            id: uuidv4(),
            createdAt: new Date(),
        };
        const reviews = Storage.getReviews();
        reviews.push(newReview);
        Storage.setReviews(reviews);
        return newReview;
    }

    /**
     * 获取用户对角色的标签
     */
    static getUserTags(userId: string, roleId: string): UserTag | undefined {
        return Storage.getUserTags().find((ut) => ut.userId === userId && ut.roleId === roleId);
    }

    /**
     * 设置用户对角色的标签（最多 5 个）
     */
    static setUserTags(userId: string, roleId: string, tags: Tag[]): UserTag {
        if (tags.length > 5) {
            throw new Error("最多只能添加 5 个标签");
        }

        const userTags = Storage.getUserTags();
        const existingIndex = userTags.findIndex((ut) => ut.userId === userId && ut.roleId === roleId);

        const userTag: UserTag = {
            userId,
            roleId,
            tags,
        };

        if (existingIndex === -1) {
            userTags.push(userTag);
        } else {
            userTags[existingIndex] = userTag;
        }

        Storage.setUserTags(userTags);
        return userTag;
    }

    /**
     * 计算角色的平均评分
     */
    static getAverageRatingForRole(roleId: string): number {
        const ratings = this.getRatingsForRole(roleId);
        if (ratings.length === 0) return 0;
        const sum = ratings.reduce((acc, rating) => acc + rating.score, 0);
        return sum / ratings.length;
    }
}
