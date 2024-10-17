import { fetchProjects } from '@api/project';
import { createFileRoute, redirect } from '@tanstack/react-router';

export const Route = createFileRoute('/_authed/')({
    preload: false,
    beforeLoad: async () => {
        const projects = await fetchProjects();
        if (projects.length === 0) {
            throw redirect({ to: '/onboarding' });
        }

        throw redirect({
            to: '/project/$projectId',
            params: { projectId: projects[0].id },
        });
    },
});
