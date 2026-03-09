import type { User } from "../types";
import { Storage } from "./mock/storage";

function uuidv4() {
    return "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(/[xy]/g, function (c) {
        const r = (Math.random() * 16) | 0,
            v = c == "x" ? r : (r & 0x3) | 0x8;
        return v.toString(16);
    });
}

export class AuthService {
    static login(name: string, role: "admin" | "user" = "user"): User {
        const users = Storage.getUsers();
        let user = users.find((u) => u.name === name);

        if (!user) {
            user = {
                id: uuidv4(),
                name,
                role,
                avatar: `https://api.dicebear.com/7.x/avataaars/svg?seed=${name}`,
            };
            users.push(user);
            Storage.setUsers(users);
        } else {
            if (user.role !== role) {
                user.role = role;
                Storage.setUsers(users);
            }
        }

        Storage.setCurrentUser(user);
        return user;
    }

    static logout() {
        Storage.setCurrentUser(null);
    }

    static getCurrentUser(): User | null {
        return Storage.getCurrentUser();
    }
}
