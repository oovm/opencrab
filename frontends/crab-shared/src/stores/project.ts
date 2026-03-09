import { defineStore } from "pinia";
import { ProjectService } from "../services";
import type { StarProject } from "../types";
import { ref } from "vue";
import { useAuthStore } from "./auth";

export const useProjectStore = defineStore("project", () => {
    const authStore = useAuthStore();

    /**
     * 所有明星项目列表
     */
    const starProjects = ref<StarProject[]>(ProjectService.getStarProjects());

    /**
     * 获取所有明星项目
     */
    function fetchStarProjects() {
        starProjects.value = ProjectService.getStarProjects();
    }

    /**
     * 根据 ID 获取明星项目
     */
    function getStarProjectById(id: string): StarProject | undefined {
        return starProjects.value.find((project) => project.id === id);
    }

    /**
     * 创建新明星项目
     */
    function createStarProject(projectData: Omit<StarProject, "id" | "createdAt" | "updatedAt">): StarProject {
        const newProject = ProjectService.createStarProject(projectData);
        starProjects.value.push(newProject);
        return newProject;
    }

    /**
     * 更新明星项目
     */
    function updateStarProject(
        id: string,
        projectData: Partial<Omit<StarProject, "id" | "createdAt">>,
    ): StarProject | undefined {
        const updatedProject = ProjectService.updateStarProject(id, projectData);
        if (updatedProject) {
            const index = starProjects.value.findIndex((project) => project.id === id);
            if (index !== -1) {
                starProjects.value[index] = updatedProject;
            }
        }
        return updatedProject;
    }

    /**
     * 删除明星项目
     */
    function deleteStarProject(id: string): boolean {
        const success = ProjectService.deleteStarProject(id);
        if (success) {
            starProjects.value = starProjects.value.filter((project) => project.id !== id);
        }
        return success;
    }

    return {
        starProjects,
        fetchStarProjects,
        getStarProjectById,
        createStarProject,
        updateStarProject,
        deleteStarProject,
    };
});
