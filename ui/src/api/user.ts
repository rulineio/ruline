import * as v from 'valibot';

export async function fetchUser(): Promise<User> {
    const response = await fetch('/users');

    if (response.status !== 200 && response.status !== 401) {
        throw new Error(`Error fetching user: ${response.statusText}`);
    }

    const data = await response.json();
    return v.parse(User, data);
}

const User = v.object({
    id: v.string(),
    email: v.string(),
    status: v.picklist(['created', 'active']),
    name: v.string(),
    avatar: v.string(),
});

export type User = v.InferInput<typeof User>;
