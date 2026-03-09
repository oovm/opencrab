import { defineStore } from "pinia";
import { AuthService } from "../services";
import type { User } from "../types";
import { ref } from "vue";

export const useAuthStore = defineStore("auth", () => {
    const user = ref<User | null>(AuthService.getCurrentUser());

    function login(name: string, role: "admin" | "user" = "user") {
        user.value = AuthService.login(name, role);
    }

    function logout() {
        AuthService.logout();
        user.value = null;
    }

    return { user, login, logout };
});
