import { createLazyFileRoute } from '@tanstack/react-router';

export const Route = createLazyFileRoute('/')({
    component: Index,
});

function Index() {
    return (
        <div className="flex items-center justify-center h-screen bg-blue-400">
            <h1 className="text-white text-6xl">Welcome to Ruline!</h1>
        </div>
    );
}
