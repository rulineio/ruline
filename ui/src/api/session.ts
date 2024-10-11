import * as v from 'valibot';

export async function fetchSession(): Promise<Session> {
    const response = await fetch('/session');

    if (response.status !== 200 && response.status !== 401) {
        throw new Error(`Error fetching session: ${response.statusText}`);
    }

    const data = await response.json();
    return v.parse(Session, data);
}

const Session = v.object({
    type: v.picklist(['user', 'member', 'unauthenticated']),
    user_id: v.string(),
    user_status: v.picklist(['created', 'active']),
    organization_id: v.optional(v.string()),
    organization_status: v.optional(v.picklist(['active'])),
    member_role: v.optional(
        v.picklist(['owner', 'admin', 'editor', 'viewer', 'member']),
    ),
    member_status: v.optional(v.picklist(['active', 'left', 'removed'])),
});

export type Session = v.InferInput<typeof Session>;
