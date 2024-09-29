import './app.css';
import { createRouter, RouterProvider } from '@tanstack/react-router';

import { routeTree } from './routeTree.gen';
import { useAuth } from './hooks/auth';

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
