import { defineStore } from "pinia";
import { RoleService } from "../services";
import type { Role, Skill, Rating, Review, Tag, UserTag } from "../types";
import { ref, computed } from "vue";
import { useAuthStore } from "./auth";

export const useRoleStore = defineStore("role", () => {
    const authStore = useAuthStore();

    /**
     * 所有角色列表
     */
    const roles = ref<Role[]>(RoleService.getRoles());

    /**
     * 所有技能列表
     */
    const skills = ref<Skill[]>(RoleService.getSkills());

    /**
     * 获取所有角色
     */
    function fetchRoles() {
        roles.value = RoleService.getRoles();
    }

    /**
     * 根据 ID 获取角色
     */
    function getRoleById(id: string): Role | undefined {
        return roles.value.find((role) => role.id === id);
    }

    /**
     * 创建新角色
     */
    function createRole(roleData: Omit<Role, "id" | "createdAt" | "updatedAt">): Role {
        const newRole = RoleService.createRole(roleData);
        roles.value.push(newRole);
        return newRole;
    }

    /**
     * 更新角色
     */
    function updateRole(id: string, roleData: Partial<Omit<Role, "id" | "createdAt">>): Role | undefined {
        const updatedRole = RoleService.updateRole(id, roleData);
        if (updatedRole) {
            const index = roles.value.findIndex((role) => role.id === id);
            if (index !== -1) {
                roles.value[index] = updatedRole;
            }
        }
        return updatedRole;
    }

    /**
     * 删除角色
     */
    function deleteRole(id: string): boolean {
        const success = RoleService.deleteRole(id);
        if (success) {
            roles.value = roles.value.filter((role) => role.id !== id);
        }
        return success;
    }

    /**
     * 获取所有技能
     */
    function fetchSkills() {
        skills.value = RoleService.getSkills();
    }

    /**
     * 创建新技能
     */
    function createSkill(skillData: Omit<Skill, "id">): Skill {
        const newSkill = RoleService.createSkill(skillData);
        skills.value.push(newSkill);
        return newSkill;
    }

    /**
     * 获取角色的评分
     */
    function getRatingsForRole(roleId: string): Rating[] {
        return RoleService.getRatingsForRole(roleId);
    }

    /**
     * 创建评分
     */
    function createRating(ratingData: Omit<Rating, "id" | "createdAt">): Rating {
        return RoleService.createRating(ratingData);
    }

    /**
     * 获取角色的评论
     */
    function getReviewsForRole(roleId: string): Review[] {
        return RoleService.getReviewsForRole(roleId);
    }

    /**
     * 创建评论
     */
    function createReview(reviewData: Omit<Review, "id" | "createdAt">): Review {
        return RoleService.createReview(reviewData);
    }

    /**
     * 获取当前用户对角色的标签
     */
    function getCurrentUserTags(roleId: string): UserTag | undefined {
        if (!authStore.user) return undefined;
        return RoleService.getUserTags(authStore.user.id, roleId);
    }

    /**
     * 设置当前用户对角色的标签（最多 5 个）
     */
    function setCurrentUserTags(roleId: string, tags: Tag[]): UserTag | undefined {
        if (!authStore.user) return undefined;
        return RoleService.setUserTags(authStore.user.id, roleId, tags);
    }

    /**
     * 计算角色的平均评分
     */
    function getAverageRatingForRole(roleId: string): number {
        return RoleService.getAverageRatingForRole(roleId);
    }

    /**
     * 员工类型的角色
     */
    const employees = computed(() => roles.value.filter((role) => role.type === "employee"));

    /**
     * 顾问类型的角色
     */
    const consultants = computed(() => roles.value.filter((role) => role.type === "consultant"));

    return {
        roles,
        skills,
        employees,
        consultants,
        fetchRoles,
        getRoleById,
        createRole,
        updateRole,
        deleteRole,
        fetchSkills,
        createSkill,
        getRatingsForRole,
        createRating,
        getReviewsForRole,
        createReview,
        getCurrentUserTags,
        setCurrentUserTags,
        getAverageRatingForRole,
    };
});
