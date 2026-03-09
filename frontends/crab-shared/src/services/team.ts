import type { StarTeam } from "../types";
import { Storage } from "./mock/storage";

function uuidv4() {
    return "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(/[xy]/g, function (c) {
        const r = (Math.random() * 16) | 0,
            v = c == "x" ? r : (r & 0x3) | 0x8;
        return v.toString(16);
    });
}

export class TeamService {
    /**
     * 获取所有明星团队
     */
    static getStarTeams(): StarTeam[] {
        return Storage.getStarTeams();
    }

    /**
     * 根据 ID 获取明星团队
     */
    static getStarTeamById(id: string): StarTeam | undefined {
        return Storage.getStarTeams().find((team) => team.id === id);
    }

    /**
     * 创建新明星团队
     */
    static createStarTeam(teamData: Omit<StarTeam, "id" | "createdAt" | "updatedAt">): StarTeam {
        const now = new Date();
        const newTeam: StarTeam = {
            ...teamData,
            id: uuidv4(),
            createdAt: now,
            updatedAt: now,
        };
        const teams = Storage.getStarTeams();
        teams.push(newTeam);
        Storage.setStarTeams(teams);
        return newTeam;
    }

    /**
     * 更新明星团队
     */
    static updateStarTeam(id: string, teamData: Partial<Omit<StarTeam, "id" | "createdAt">>): StarTeam | undefined {
        const teams = Storage.getStarTeams();
        const index = teams.findIndex((team) => team.id === id);
        if (index === -1) return undefined;

        teams[index] = {
            ...teams[index],
            ...teamData,
            updatedAt: new Date(),
        };
        Storage.setStarTeams(teams);
        return teams[index];
    }

    /**
     * 删除明星团队
     */
    static deleteStarTeam(id: string): boolean {
        const teams = Storage.getStarTeams();
        const initialLength = teams.length;
        const filteredTeams = teams.filter((team) => team.id !== id);
        Storage.setStarTeams(filteredTeams);
        return filteredTeams.length !== initialLength;
    }
}
