import { defineStore } from "pinia";
import { TeamService } from "../services";
import type { StarTeam } from "../types";
import { ref } from "vue";
import { useAuthStore } from "./auth";

export const useTeamStore = defineStore("team", () => {
    const authStore = useAuthStore();

    /**
     * 所有明星团队列表
     */
    const starTeams = ref<StarTeam[]>(TeamService.getStarTeams());

    /**
     * 获取所有明星团队
     */
    function fetchStarTeams() {
        starTeams.value = TeamService.getStarTeams();
    }

    /**
     * 根据 ID 获取明星团队
     */
    function getStarTeamById(id: string): StarTeam | undefined {
        return starTeams.value.find((team) => team.id === id);
    }

    /**
     * 创建新明星团队
     */
    function createStarTeam(teamData: Omit<StarTeam, "id" | "createdAt" | "updatedAt">): StarTeam {
        const newTeam = TeamService.createStarTeam(teamData);
        starTeams.value.push(newTeam);
        return newTeam;
    }

    /**
     * 更新明星团队
     */
    function updateStarTeam(id: string, teamData: Partial<Omit<StarTeam, "id" | "createdAt">>): StarTeam | undefined {
        const updatedTeam = TeamService.updateStarTeam(id, teamData);
        if (updatedTeam) {
            const index = starTeams.value.findIndex((team) => team.id === id);
            if (index !== -1) {
                starTeams.value[index] = updatedTeam;
            }
        }
        return updatedTeam;
    }

    /**
     * 删除明星团队
     */
    function deleteStarTeam(id: string): boolean {
        const success = TeamService.deleteStarTeam(id);
        if (success) {
            starTeams.value = starTeams.value.filter((team) => team.id !== id);
        }
        return success;
    }

    return {
        starTeams,
        fetchStarTeams,
        getStarTeamById,
        createStarTeam,
        updateStarTeam,
        deleteStarTeam,
    };
});
