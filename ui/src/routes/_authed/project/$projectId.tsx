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
            <main className="sm:ml-48 min-h-screen">
                <Outlet />
            </main>
        </>
    );
}
