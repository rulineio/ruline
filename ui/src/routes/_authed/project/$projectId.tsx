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
            <main className="md:ml-48 bg-teal-1 min-h-screen text-white">
                <Outlet />
            </main>
        </>
    );
}
