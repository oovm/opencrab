import type { Company } from "../types";
import { Storage } from "./mock/storage";

function uuidv4() {
    return "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(/[xy]/g, function (c) {
        const r = (Math.random() * 16) | 0,
            v = c == "x" ? r : (r & 0x3) | 0x8;
        return v.toString(16);
    });
}

export class CompanyService {
    /**
     * 获取所有公司
     */
    static getCompanies(): Company[] {
        return Storage.getCompanies();
    }

    /**
     * 根据 ID 获取公司
     */
    static getCompanyById(id: string): Company | undefined {
        return Storage.getCompanies().find((company) => company.id === id);
    }

    /**
     * 根据名称获取公司
     */
    static getCompanyByName(name: string): Company | undefined {
        return Storage.getCompanies().find((company) => company.name === name);
    }

    /**
     * 创建新公司
     */
    static createCompany(companyData: Omit<Company, "id" | "createdAt" | "updatedAt">): Company {
        const now = new Date();
        const newCompany: Company = {
            ...companyData,
            id: uuidv4(),
            createdAt: now,
            updatedAt: now,
        };
        const companies = Storage.getCompanies();
        companies.push(newCompany);
        Storage.setCompanies(companies);
        return newCompany;
    }

    /**
     * 更新公司
     */
    static updateCompany(id: string, companyData: Partial<Omit<Company, "id" | "createdAt">>): Company | undefined {
        const companies = Storage.getCompanies();
        const index = companies.findIndex((company) => company.id === id);
        if (index === -1) return undefined;

        companies[index] = {
            ...companies[index],
            ...companyData,
            updatedAt: new Date(),
        };
        Storage.setCompanies(companies);
        return companies[index];
    }

    /**
     * 删除公司
     */
    static deleteCompany(id: string): boolean {
        const companies = Storage.getCompanies();
        const initialLength = companies.length;
        const filteredCompanies = companies.filter((company) => company.id !== id);
        Storage.setCompanies(filteredCompanies);
        return filteredCompanies.length !== initialLength;
    }
}
