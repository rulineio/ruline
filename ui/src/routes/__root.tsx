import type { AuthContext } from '@hooks/auth';
import { Outlet, createRootRouteWithContext } from '@tanstack/react-router';
import * as React from 'react';

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
