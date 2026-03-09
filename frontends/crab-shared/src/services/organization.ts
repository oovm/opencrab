import type { Client, Team, Organization, OrgStructure } from "../types";

const MOCK_CLIENTS: Client[] = [{ id: "client-1", name: "Acme Corp", description: "Global Leader in Anvils" }];

const MOCK_TEAMS: Team[] = [
    { id: "team-1", name: "Engineering", clientId: "client-1", description: "Product Development" },
];

const MOCK_ORGS: Organization[] = [
    { id: "org-1", name: "Core Platform", teamId: "team-1", description: "Main Platform Development" },
];

export class OrganizationService {
    static getStructure(orgId: string): OrgStructure | null {
        const org = MOCK_ORGS.find((o) => o.id === orgId);
        if (!org) return null;

        const team = MOCK_TEAMS.find((t) => t.id === org.teamId);
        if (!team) return null;

        const client = MOCK_CLIENTS.find((c) => c.id === team.clientId);
        if (!client) return null;

        return {
            client,
            team,
            organization: org,
        };
    }

    static getCurrentOrgId(): string {
        return MOCK_ORGS[0]?.id || "";
    }
}
