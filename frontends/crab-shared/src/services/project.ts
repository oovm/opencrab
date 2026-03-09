import type { StarProject } from "../types";
import { Storage } from "./mock/storage";

function uuidv4() {
    return "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(/[xy]/g, function (c) {
        const r = (Math.random() * 16) | 0,
            v = c == "x" ? r : (r & 0x3) | 0x8;
        return v.toString(16);
    });
}

export class ProjectService {
    /**
     * 获取所有明星项目
     */
    static getStarProjects(): StarProject[] {
        return Storage.getStarProjects();
    }

    /**
     * 根据 ID 获取明星项目
     */
    static getStarProjectById(id: string): StarProject | undefined {
        return Storage.getStarProjects().find((project) => project.id === id);
    }

    /**
     * 创建新明星项目
     */
    static createStarProject(projectData: Omit<StarProject, "id" | "createdAt" | "updatedAt">): StarProject {
        const now = new Date();
        const newProject: StarProject = {
            ...projectData,
            id: uuidv4(),
            createdAt: now,
            updatedAt: now,
        };
        const projects = Storage.getStarProjects();
        projects.push(newProject);
        Storage.setStarProjects(projects);
        return newProject;
    }

    /**
     * 更新明星项目
     */
    static updateStarProject(
        id: string,
        projectData: Partial<Omit<StarProject, "id" | "createdAt">>,
    ): StarProject | undefined {
        const projects = Storage.getStarProjects();
        const index = projects.findIndex((project) => project.id === id);
        if (index === -1) return undefined;

        projects[index] = {
            ...projects[index],
            ...projectData,
            updatedAt: new Date(),
        };
        Storage.setStarProjects(projects);
        return projects[index];
    }

    /**
     * 删除明星项目
     */
    static deleteStarProject(id: string): boolean {
        const projects = Storage.getStarProjects();
        const initialLength = projects.length;
        const filteredProjects = projects.filter((project) => project.id !== id);
        Storage.setStarProjects(filteredProjects);
        return filteredProjects.length !== initialLength;
    }
}
