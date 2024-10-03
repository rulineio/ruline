import * as React from 'react';
import { Outlet, createRootRouteWithContext } from '@tanstack/react-router';
import type { AuthContext } from '../hooks/auth';

type RouterContext = {
    auth: AuthContext;
};

export const Route = createRootRouteWithContext<RouterContext>()({
    component: () => (
        <React.Fragment>
            <Outlet />
        </React.Fragment>
    ),
});
