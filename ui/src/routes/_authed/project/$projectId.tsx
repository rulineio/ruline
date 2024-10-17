import { Sidebar } from '@components/Sidebar';
import { createFileRoute, Outlet } from '@tanstack/react-router';

export const Route = createFileRoute('/_authed/project/$projectId')({
    component: Layout,
});

function Layout() {
    const { projectId } = Route.useParams();

    return (
        <>
            <Sidebar projectId={projectId} />
            <main className="md:ml-48 bg-background min-h-screen text-background-text">
                <Outlet />
            </main>
        </>
    );
}
