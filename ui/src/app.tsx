import { createRouter, RouterProvider } from '@tanstack/react-router';
import './app.css';
import { useAuth } from './hooks/auth';
import { routeTree } from './routeTree.gen';

const router = createRouter({
    basepath: '/ui',
    routeTree,
    defaultPreload: 'intent',
    context: {
        auth: undefined!,
    },
});

declare module '@tanstack/react-router' {
    interface Register {
        router: typeof router;
    }
}

const App = () => {
    const auth = useAuth();
    return <RouterProvider router={router} context={{ auth }} />;
};

export default App;
