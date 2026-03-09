import { defineStore } from "pinia";
import { CompanyService } from "../services";
import type { Company } from "../types";
import { ref } from "vue";
import { useAuthStore } from "./auth";

export const useCompanyStore = defineStore("company", () => {
    const authStore = useAuthStore();

    /**
     * 所有公司列表
     */
    const companies = ref<Company[]>(CompanyService.getCompanies());

    /**
     * 获取所有公司
     */
    function fetchCompanies() {
        companies.value = CompanyService.getCompanies();
    }

    /**
     * 根据 ID 获取公司
     */
    function getCompanyById(id: string): Company | undefined {
        return companies.value.find((company) => company.id === id);
    }

    /**
     * 根据名称获取公司
     */
    function getCompanyByName(name: string): Company | undefined {
        return companies.value.find((company) => company.name === name);
    }

    /**
     * 创建新公司
     */
    function createCompany(companyData: Omit<Company, "id" | "createdAt" | "updatedAt">): Company {
        const newCompany = CompanyService.createCompany(companyData);
        companies.value.push(newCompany);
        return newCompany;
    }

    /**
     * 更新公司
     */
    function updateCompany(id: string, companyData: Partial<Omit<Company, "id" | "createdAt">>): Company | undefined {
        const updatedCompany = CompanyService.updateCompany(id, companyData);
        if (updatedCompany) {
            const index = companies.value.findIndex((company) => company.id === id);
            if (index !== -1) {
                companies.value[index] = updatedCompany;
            }
        }
        return updatedCompany;
    }

    /**
     * 删除公司
     */
    function deleteCompany(id: string): boolean {
        const success = CompanyService.deleteCompany(id);
        if (success) {
            companies.value = companies.value.filter((company) => company.id !== id);
        }
        return success;
    }

    return {
        companies,
        fetchCompanies,
        getCompanyById,
        getCompanyByName,
        createCompany,
        updateCompany,
        deleteCompany,
    };
});
