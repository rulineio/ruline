import * as v from 'valibot';

export const fetchUser = async (): Promise<User> => {
    const response = await fetch('/users/me');
    const data = await response.json();
    return v.parse(User, data);
};

const User = v.object({
    email: v.string(),
    status: v.picklist(['created', 'active', 'blocked']),
    name: v.string(),
    avatar: v.string(),
});

export type User = v.InferInput<typeof User>;
