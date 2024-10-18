import { defineConfig } from '@rsbuild/core';
import { pluginReact } from '@rsbuild/plugin-react';
import { TanStackRouterRspack } from '@tanstack/router-plugin/rspack';

export default defineConfig({
    plugins: [pluginReact()],
    tools: {
        rspack: {
            plugins: [TanStackRouterRspack()],
        },
    },
    dev: {
        writeToDisk: true,
    },
    html: {
        title: 'Ruline',
        meta: {
            description: 'Ruline console',
        },
    },
});
