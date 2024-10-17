import * as v from 'valibot';

export async function inviteMember(member: InviteMemberForm): Promise<void> {
    const response = await fetch('/invitations', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            email: member.email,
            role: member.role,
            name: `${member.firstName} ${member.lastName}`,
        }),
    });

    if (response.status !== 201) {
        throw new Error('Something went wrong. Please try again later.');
    }
}

export async function fetchInvitations(): Promise<Invitations> {
    const response = await fetch('/invitations');

    if (response.status !== 200) {
        throw new Error(`Error fetching invitations: ${response.statusText}`);
    }

    const data = await response.json();
    return v.parse(Invitations, data);
}

export async function acceptInvitation(id: string): Promise<void> {
    const response = await fetch(`/invitations/${id}/accept`, {
        method: 'POST',
    });

    if (response.status !== 204) {
        throw new Error('Something went wrong. Please try again later.');
    }
}

export async function declineInvitation(id: string): Promise<void> {
    const response = await fetch(`/invitations/${id}/decline`, {
        method: 'POST',
    });

    if (response.status !== 204) {
        throw new Error('Something went wrong. Please try again later.');
    }
}

export const InviteMemberSchema = v.object({
    email: v.pipe(
        v.string('Please enter a valid email address'),
        v.nonEmpty('Please enter your email address'),
        v.email('Please enter a valid email address'),
    ),
    firstName: v.pipe(
        v.string('Please enter a valid first name'),
        v.nonEmpty('Please enter your first name'),
        v.trim(),
    ),
    lastName: v.optional(v.pipe(v.string(), v.trim())),
    role: v.optional(v.picklist(['admin', 'editor', 'viewer']), 'viewer'),
});
export type InviteMemberForm = v.InferInput<typeof InviteMemberSchema>;

export const Invitations = v.array(
    v.object({
        id: v.string(),
        organization_name: v.string(),
    }),
);
export type Invitations = v.InferInput<typeof Invitations>;
