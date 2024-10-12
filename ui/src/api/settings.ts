import * as v from 'valibot';

export async function fetchSettings(): Promise<Settings> {
    const response = await fetch('/settings');

    if (response.status !== 200) {
        throw new Error(`Error fetching settings: ${response.statusText}`);
    }

    const data = await response.json();
    return v.parse(Settings, data);
}

const Settings = v.object({
    google_auth_enabled: v.boolean(),
    magic_link_enabled: v.boolean(),
});

export type Settings = v.InferInput<typeof Settings>;
